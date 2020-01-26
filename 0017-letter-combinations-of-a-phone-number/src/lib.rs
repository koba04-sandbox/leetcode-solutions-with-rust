use std::collections::HashMap;
pub struct Solution {}


impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut digitMap: HashMap<char, Vec<char>> = HashMap::new();
        digitMap.insert('2', vec!['a', 'b', 'c']);
        digitMap.insert('3', vec!['d', 'e', 'f']);
        digitMap.insert('4', vec!['g', 'h', 'i']);
        digitMap.insert('5', vec!['j', 'k', 'l']);
        digitMap.insert('6', vec!['m', 'n', 'o']);
        digitMap.insert('7', vec!['p', 'q', 'r', 's']);
        digitMap.insert('8', vec!['t', 'u', 'v']);
        digitMap.insert('9', vec!['w', 'x', 'y', 'z']);

        let mut answers = vec![];

        let chars: Vec<char> = digits.chars().collect();
        let len = chars.len();

        answers
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
