// https://leetcode.com/problems/sort-the-people/description

// https://leetcode.com/problems/sort-the-people/solutions/5515725/0ms-standard-zip-and-sort
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut ls: Vec<_> = heights
            .into_iter()
            .map(|x| -x)
            .zip(names.into_iter())
            .collect();
        ls.sort_unstable();
        let (_, names): (Vec<i32>, Vec<String>) = ls.into_iter().unzip();
        names
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let test = |ns: &[&str], hs: &[i32], exp: &[&str]| {
        assert_eq!(
            Solution::sort_people(ns.into_iter().map(|x| x.to_string()).collect(), hs.to_vec()),
            exp.into_iter().map(|x| x.to_string()).collect::<Vec<_>>()
        )
    };
    test(
        &["Mary", "John", "Emma"],
        &[180, 165, 170],
        &["Mary", "Emma", "John"],
    );
}
