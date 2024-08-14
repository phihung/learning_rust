// https://leetcode.com/problems/integer-to-english-words

impl Solution {
    const ZERO: &str = "Zero";
    const D1: [&str; 10] = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
    ];
    #[rustfmt::skip]
    const TEENS: [&str; 10] = [
        "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
    ];
    const D2: [&str; 10] = [
        "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ];
    const HUNDRED: &str = "Hundred";
    const D369: [&str; 3] = ["Thousand", "Million", "Billion"];

    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return Self::ZERO.to_string();
        }
        let mut num = num as usize;
        let mut out = Vec::new();
        for (i, &unit) in [1000000000, 1000000, 1000, 1].iter().enumerate() {
            let (d, r) = (num / unit, num % unit);
            if d > 0 {
                Self::f1000(d, &mut out);
                if i < 3 {
                    out.push(Self::D369[2 - i]);
                }
            }
            num = r;
        }
        out.join(" ")
    }

    fn f1000(num: usize, out: &mut Vec<&str>) {
        if num < 100 {
            Self::f100(num, out);
        } else {
            out.push(Self::D1[num / 100]);
            out.push(Self::HUNDRED);
            Self::f100(num % 100, out);
        }
    }

    fn f100(num: usize, out: &mut Vec<&str>) {
        assert!(num < 100);
        if num == 0 {
            //
        } else if num < 10 {
            out.push(Self::D1[num])
        } else if num < 20 {
            out.push(Self::TEENS[num - 10])
        } else {
            out.push(Self::D2[num / 10]);
            if num % 10 != 0 {
                out.push(Self::D1[num % 10]);
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_solution() {
        let func = Solution::number_to_words;
        assert_eq!(func(123), "One Hundred Twenty Three");
        assert_eq!(func(10013), "Ten Thousand Thirteen");
        assert_eq!(func(12345), "Twelve Thousand Three Hundred Forty Five");
        assert_eq!(
            func(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
        assert_eq!(func(2147483647), "Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Seven");
        assert_eq!(
            func(2007400640),
            "Two Billion Seven Million Four Hundred Thousand Six Hundred Forty"
        );
    }
}
