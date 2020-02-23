use std::collections::HashMap;
pub struct Solution {}


impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut digit_map: HashMap<char, Vec<char>> = HashMap::new();
        digit_map.insert('2', vec!['a', 'b', 'c']);
        digit_map.insert('3', vec!['d', 'e', 'f']);
        digit_map.insert('4', vec!['g', 'h', 'i']);
        digit_map.insert('5', vec!['j', 'k', 'l']);
        digit_map.insert('6', vec!['m', 'n', 'o']);
        digit_map.insert('7', vec!['p', 'q', 'r', 's']);
        digit_map.insert('8', vec!['t', 'u', 'v']);
        digit_map.insert('9', vec!['w', 'x', 'y', 'z']);

        let mut answers = vec![];

        let all_chars: Vec<&Vec<char>> = digits.chars().map(|c| digit_map.get(&c).unwrap()).collect();
        if all_chars.len() == 0 {
            return answers;
        }

        let mut i = 0;
        let last = all_chars.len() - 1;
        while i < last {
            let first = all_chars[i];
            let second = all_chars[i + 1];
            for f in first {
                for s in second {
                    answers.push(format!("{}{}", f, s));
                }
            }
            i += 1;
        }
        answers
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
    #[test]
    fn example2() {
        let expect: Vec<String> = vec![];
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            expect
        );
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a", "b", "c"]
        );
    }
}
