/// https://leetcode.com/problems/permutation-sequence/

// O(N). Top 100%.
// https://leetcode.com/problems/permutation-sequence/solutions/5437831/0ms-intuitive-explanation/
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let a = Self::to_factorial_base(k - 1, n);
        let mut elems: Vec<char> = (0..n as u8).map(|i| (b'1' + i) as char).collect();
        let mut out = String::with_capacity(n as usize);
        for &a_i in a.iter().rev() {
            out.push(elems.remove(a_i as usize));
        }
        out
    }

    // num    = a0 + 1!a1 + 2!a2 + 3!a3 + ... + (n-1)!a{n-1}
    //        = a0 + 1(a1 + 2(a2 + 3(a3 + 4...))
    // with a_i < i + 1
    fn to_factorial_base(num: i32, n: i32) -> Vec<u8> {
        let mut a = vec![0_u8; n as usize];
        let mut v = num;
        for i in 0..n {
            a[i as usize] = (v % (1 + i)) as u8;
            v = v / (i + 1);
        }
        a
    }
}

// ---- test ----

pub struct Solution {}

#[test]
fn test_solution() {
    assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
    assert_eq!(Solution::get_permutation(3, 1), "123".to_string());
    assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
    assert_eq!(Solution::get_permutation(4, 10), "2341".to_string());
}
