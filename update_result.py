import glob
import os
from pathlib import Path
import re
import time
import requests
import tqdm
import json
import tabulate

session_id = os.environ["LEETCODE_SESSION"]
csrftoken = os.environ["LEETCODE_CRSF"]


def call_graphql(payload):
    url = "https://leetcode.com/graphql/"
    cookies = {"csrftoken": csrftoken, "LEETCODE_SESSION": session_id}
    return requests.post(url, json=payload, cookies=cookies)


def get_submission(slug):
    payload = {
        "query": "\n    query submissionList($offset: Int!, $limit: Int!, $lastKey: String, $questionSlug: String!, $lang: Int, $status: Int) {\n  questionSubmissionList(\n    offset: $offset\n    limit: $limit\n    lastKey: $lastKey\n    questionSlug: $questionSlug\n    lang: $lang\n    status: $status\n  ) {\n    lastKey\n    hasNext\n    submissions {\n      id\n      title\n      titleSlug\n      status\n      statusDisplay\n      lang\n      langName\n      runtime\n      timestamp\n      url\n      isPending\n      memory\n      hasNotes\n      notes\n      flagType\n      topicTags {\n        id\n      }\n    }\n  }\n}\n    ",
        "variables": {
            "questionSlug": slug,
            "offset": 0,
            "limit": 20,
            "lastKey": None,
        },
        "operationName": "submissionList",
    }
    r = call_graphql(payload)
    return r.json()["data"]["questionSubmissionList"]["submissions"]


def get_solved_questions():
    payload = {
        "operationName": "problemsetQuestionList",
        "query": "\n    query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {\n  problemsetQuestionList: questionList(\n    categorySlug: $categorySlug\n    limit: $limit\n    skip: $skip\n    filters: $filters\n  ) {\n    total: totalNum\n    questions: data {\n      acRate\n      difficulty\n      freqBar\n      frontendQuestionId: questionFrontendId\n      isFavor\n      paidOnly: isPaidOnly\n      status\n      title\n      titleSlug\n      topicTags {\n        name\n        id\n        slug\n      }\n      hasSolution\n      hasVideoSolution\n    }\n  }\n}\n    ",
        "variables": {
            "categorySlug": "all-code-essentials",
            "skip": 0,
            "limit": 100,
            "filters": {"status": "AC"},
        },
    }
    r = call_graphql(payload)
    return r.json()["data"]["problemsetQuestionList"]["questions"]


def get_submission_detail(submission_id):
    payload = {
        "operationName": "submissionDetails",
        "query": "\n    query submissionDetails($submissionId: Int!) {\n  submissionDetails(submissionId: $submissionId) {\n    runtime\n    runtimeDisplay\n    runtimePercentile\n    runtimeDistribution\n    memory\n    memoryDisplay\n    memoryPercentile\n    memoryDistribution\n    code\n    timestamp\n    statusCode\n    user {\n      username\n      profile {\n        realName\n        userAvatar\n      }\n    }\n    lang {\n      name\n      verboseName\n    }\n    question {\n      questionId\n      titleSlug\n      hasFrontendPreview\n    }\n    notes\n    flagType\n    topicTags {\n      tagId\n      slug\n      name\n    }\n    runtimeError\n    compileError\n    lastTestcase\n    codeOutput\n    expectedOutput\n    totalCorrect\n    totalTestcases\n    fullCodeOutput\n    testDescriptions\n    testBodies\n    testInfo\n    stdOutput\n  }\n}\n    ",
        "variables": {"submissionId": submission_id},
    }
    r = call_graphql(payload)
    return r.json()["data"]["submissionDetails"]


def get_data():
    solved = get_solved_questions()
    solved = {x["titleSlug"]: x for x in solved}

    outputs = []
    for fn in tqdm.tqdm(list(glob.glob("src/leetcode/*.rs"))):
        if ls := re.findall("a(\d\d\d\d)", fn):
            question_id = ls[0]
        else:
            continue
        txt = Path(fn).read_text()
        if ls := re.findall(r"https://leetcode.com/problems/([^/\s]+)", txt):
            slug = ls[0]
        else:
            print(question_id, "No url found")
            continue

        time.sleep(0.5)
        submissions = get_submission(slug)
        green = [sub for sub in submissions if sub["flagType"] == "GREEN"]
        not_white = [sub for sub in submissions if sub["flagType"] != "WHITE"]
        accepted = sorted(
            [sub for sub in submissions if sub["statusDisplay"] == "Accepted"],
            key=lambda x: int(x["runtime"].split()[0]),
        )
        if ls := green or not_white or accepted:
            answer = ls[0]
        else:
            print(question_id, slug, "No answer found")
            continue
        detail = get_submission_detail(int(answer["id"]))
        problem = solved[slug]
        outputs.append(
            {
                "filepath": fn,
                "title": answer["title"],
                "slug": answer["titleSlug"],
                "status": answer["statusDisplay"],
                "timestamp": answer["timestamp"],
                "flagType": answer["flagType"],
                "notes": answer["notes"],
                "acRate": problem["acRate"],
                "difficulty": problem["difficulty"],
                "submission": answer["url"],
                "runtime": detail["runtime"],
                "runtimePercentile": detail["runtimePercentile"],
                "runtimeDisplay": detail["runtimeDisplay"],
                "memory": detail["memory"],
                "memoryDisplay": detail["memoryDisplay"],
                "memoryPercentile": detail["memoryPercentile"],
                "code": detail["code"],
                "runtimeDistribution": detail["runtimeDistribution"],
                "memoryDistribution": detail["memoryDistribution"],
            }
        )
    outputs = sorted(outputs, key=lambda x: -int(x["timestamp"]))
    return outputs


def to_table(outputs):
    table = []
    for x in outputs:
        # title = x["title"]
        difficulty = x["difficulty"]
        # acRate = f'{int(x["acRate"])}%'
        slug = x["slug"]
        link = f"[{slug}](https://leetcode.com/problems/{slug}/)"
        runtime = x["runtimeDisplay"]
        if (v := x["runtimePercentile"]) > 90 or runtime == "1 ms":
            toptime = f'<span style="color:green;">{v:3.0f} %</span>'
        else:
            toptime = f"{v:3.0f} %"

        memory = x["memoryDisplay"]
        topmemory = f'{x["memoryPercentile"]:3.0f} %'
        if (v := x["filepath"]).endswith("_.rs"):
            continue
        else:
            code = f"[Code]({v})"
        table.append((link, difficulty, runtime, toptime, memory, topmemory, code))
        headers = ["Problem", "Difficulty", "Time", "Top", "Memory", "Top", "Code"]
    return tabulate.tabulate(table, headers, tablefmt="github")


# Learning Rust by doing programming challenges

README = """\
# Learning Rust by doing programming challenges

```bash
cargo test --lib
cargo test --lib a0037
python update_result.py
```

"""


def update_readme(table):
    with open("README.md", "w") as fd:
        fd.write(README + table)


if __name__ == "__main__":
    # outputs = json.load(open("data/leetcode.json"))
    # outputs = sorted(outputs, key=lambda x: -int(x["timestamp"]))
    outputs = get_data()
    Path("data/leetcode.json").write_text(json.dumps(outputs))
    update_readme(to_table(outputs))
