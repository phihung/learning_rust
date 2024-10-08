// https://leetcode.com/problems/word-ladder/description/

use std::collections::{HashMap, VecDeque};

// https://leetcode.com/problems/word-ladder/solutions/5508384/clean-dijkstra-graph-search-100
// 100%
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        let n = word_list.len();
        let i_end = match word_list.iter().position(|w| *w == end_word) {
            None => return 0,
            Some(x) => x,
        };

        word_list.push(begin_word);
        let neighbors = Self::build_graph(word_list);
        return Self::dijkstra(n + 1, &neighbors, n, i_end).map_or(0, |d| d + 1) as i32;
    }

    fn build_graph(word_list: Vec<String>) -> Vec<Vec<usize>> {
        let n = word_list.len();
        let mut m: HashMap<i64, Vec<usize>> = HashMap::with_capacity(n * word_list[0].len());

        // group words by (n-1) characters ({ "h_t": ["hot", "dog", ...]})
        for (i, w) in word_list.into_iter().enumerate() {
            for j in 0..w.len() {
                let s_without_j = Self::to_int(&w, j);
                m.entry(s_without_j).or_insert(vec![]).push(i);
            }
        }

        let mut neighbors = vec![vec![]; n + 1];
        for (_, v) in m {
            for &i in &v {
                for &j in &v {
                    if i != j {
                        neighbors[i].push(j);
                    }
                }
            }
        }
        return neighbors;
    }

    // build a i64 representing a word with one char ignore  ("h_llo")
    fn to_int(s: &str, remove_index: usize) -> i64 {
        let mut out = 0;
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if i != remove_index {
                out = (out << 5) + (c - 'a' as u8) as i64;
            } else {
                out = (out << 5) + 31;
            }
        }
        out
    }

    fn dijkstra(
        n_nodes: usize,
        neighbors: &Vec<Vec<usize>>,
        start: usize,
        end: usize,
    ) -> Option<usize> {
        let mut distances = vec![u8::MAX; n_nodes];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        distances[start] = 0;
        while let Some(idx) = queue.pop_front() {
            let dist = distances[idx];
            for &neighbor in &neighbors[idx] {
                let v = &mut distances[neighbor];
                if *v == u8::MAX {
                    *v = (dist + 1).min(*v);
                    if neighbor == end {
                        return Some(*v as usize);
                    }
                    queue.push_back(neighbor);
                }
            }
        }
        None
    }
}

pub struct Solution;

#[test]
fn test_solution() {
    let func = |b: &str, e: &str, l: &[&str]| {
        Solution::ladder_length(
            b.to_string(),
            e.to_string(),
            l.iter().map(ToString::to_string).collect(),
        )
    };

    assert_eq!(
        func("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"]),
        5
    );
    assert_eq!(func("hit", "cog", &["hot", "dot", "dog", "lot", "log"]), 0);
    assert_eq!(
        func(
            "talk",
            "tail",
            &["talk", "tons", "fall", "tail", "gale", "hall", "negs"]
        ),
        0
    );
    assert_eq!(
        func(
            "cet",
            "ism",
            &[
                "kid", "tag", "pup", "ail", "tun", "woo", "erg", "luz", "brr", "gay", "sip", "kay",
                "per", "val", "mes", "ohs", "now", "boa", "cet", "pal", "bar", "die", "war", "hay",
                "eco", "pub", "lob", "rue", "fry", "lit", "rex", "jan", "cot", "bid", "ali", "pay",
                "col", "gum", "ger", "row", "won", "dan", "rum", "fad", "tut", "sag", "yip", "sui",
                "ark", "has", "zip", "fez", "own", "ump", "dis", "ads", "max", "jaw", "out", "btu",
                "ana", "gap", "cry", "led", "abe", "box", "ore", "pig", "fie", "toy", "fat", "cal",
                "lie", "noh", "sew", "ono", "tam", "flu", "mgm", "ply", "awe", "pry", "tit", "tie",
                "yet", "too", "tax", "jim", "san", "pan", "map", "ski", "ova", "wed", "non", "wac",
                "nut", "why", "bye", "lye", "oct", "old", "fin", "feb", "chi", "sap", "owl", "log",
                "tod", "dot", "bow", "fob", "for", "joe", "ivy", "fan", "age", "fax", "hip", "jib",
                "mel", "hus", "sob", "ifs", "tab", "ara", "dab", "jag", "jar", "arm", "lot", "tom",
                "sax", "tex", "yum", "pei", "wen", "wry", "ire", "irk", "far", "mew", "wit", "doe",
                "gas", "rte", "ian", "pot", "ask", "wag", "hag", "amy", "nag", "ron", "soy", "gin",
                "don", "tug", "fay", "vic", "boo", "nam", "ave", "buy", "sop", "but", "orb", "fen",
                "paw", "his", "sub", "bob", "yea", "oft", "inn", "rod", "yam", "pew", "web", "hod",
                "hun", "gyp", "wei", "wis", "rob", "gad", "pie", "mon", "dog", "bib", "rub", "ere",
                "dig", "era", "cat", "fox", "bee", "mod", "day", "apr", "vie", "nev", "jam", "pam",
                "new", "aye", "ani", "and", "ibm", "yap", "can", "pyx", "tar", "kin", "fog", "hum",
                "pip", "cup", "dye", "lyx", "jog", "nun", "par", "wan", "fey", "bus", "oak", "bad",
                "ats", "set", "qom", "vat", "eat", "pus", "rev", "axe", "ion", "six", "ila", "lao",
                "mom", "mas", "pro", "few", "opt", "poe", "art", "ash", "oar", "cap", "lop", "may",
                "shy", "rid", "bat", "sum", "rim", "fee", "bmw", "sky", "maj", "hue", "thy", "ava",
                "rap", "den", "fla", "auk", "cox", "ibo", "hey", "saw", "vim", "sec", "ltd", "you",
                "its", "tat", "dew", "eva", "tog", "ram", "let", "see", "zit", "maw", "nix", "ate",
                "gig", "rep", "owe", "ind", "hog", "eve", "sam", "zoo", "any", "dow", "cod", "bed",
                "vet", "ham", "sis", "hex", "via", "fir", "nod", "mao", "aug", "mum", "hoe", "bah",
                "hal", "keg", "hew", "zed", "tow", "gog", "ass", "dem", "who", "bet", "gos", "son",
                "ear", "spy", "kit", "boy", "due", "sen", "oaf", "mix", "hep", "fur", "ada", "bin",
                "nil", "mia", "ewe", "hit", "fix", "sad", "rib", "eye", "hop", "haw", "wax", "mid",
                "tad", "ken", "wad", "rye", "pap", "bog", "gut", "ito", "woe", "our", "ado", "sin",
                "mad", "ray", "hon", "roy", "dip", "hen", "iva", "lug", "asp", "hui", "yak", "bay",
                "poi", "yep", "bun", "try", "lad", "elm", "nat", "wyo", "gym", "dug", "toe", "dee",
                "wig", "sly", "rip", "geo", "cog", "pas", "zen", "odd", "nan", "lay", "pod", "fit",
                "hem", "joy", "bum", "rio", "yon", "dec", "leg", "put", "sue", "dim", "pet", "yaw",
                "nub", "bit", "bur", "sid", "sun", "oil", "red", "doc", "moe", "caw", "eel", "dix",
                "cub", "end", "gem", "off", "yew", "hug", "pop", "tub", "sgt", "lid", "pun", "ton",
                "sol", "din", "yup", "jab", "pea", "bug", "gag", "mil", "jig", "hub", "low", "did",
                "tin", "get", "gte", "sox", "lei", "mig", "fig", "lon", "use", "ban", "flo", "nov",
                "jut", "bag", "mir", "sty", "lap", "two", "ins", "con", "ant", "net", "tux", "ode",
                "stu", "mug", "cad", "nap", "gun", "fop", "tot", "sow", "sal", "sic", "ted", "wot",
                "del", "imp", "cob", "way", "ann", "tan", "mci", "job", "wet", "ism", "err", "him",
                "all", "pad", "hah", "hie", "aim", "ike", "jed", "ego", "mac", "baa", "min", "com",
                "ill", "was", "cab", "ago", "ina", "big", "ilk", "gal", "tap", "duh", "ola", "ran",
                "lab", "top", "gob", "hot", "ora", "tia", "kip", "han", "met", "hut", "she", "sac",
                "fed", "goo", "tee", "ell", "not", "act", "gil", "rut", "ala", "ape", "rig", "cid",
                "god", "duo", "lin", "aid", "gel", "awl", "lag", "elf", "liz", "ref", "aha", "fib",
                "oho", "tho", "her", "nor", "ace", "adz", "fun", "ned", "coo", "win", "tao", "coy",
                "van", "man", "pit", "guy", "foe", "hid", "mai", "sup", "jay", "hob", "mow", "jot",
                "are", "pol", "arc", "lax", "aft", "alb", "len", "air", "pug", "pox", "vow", "got",
                "meg", "zoe", "amp", "ale", "bud", "gee", "pin", "dun", "pat", "ten", "mob"
            ]
        ),
        11
    );
}
