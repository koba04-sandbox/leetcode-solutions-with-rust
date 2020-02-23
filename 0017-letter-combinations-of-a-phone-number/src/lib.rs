pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<char> = digits.chars().collect();
        if digits.len() == 0 {
            return vec![];
        }
        let mut answers: Vec<String> = vec![];
        Solution::create_pattern(String::from(""), 0, &digits, &mut answers);
        answers.to_vec()
    }
    fn create_pattern(candidate: String, index: usize, digits: &Vec<char>, answers: &mut Vec<String>) {
        let digit = digits[index];
        let chars = Solution::get_chars(digit);
        let next_index = index + 1;
        for c in chars {
            let current = format!("{}{}", candidate, c);
            if next_index < digits.len() {
                Solution::create_pattern(current, next_index, &digits, answers);
            } else {
                answers.push(current);
            }
        }
    }
    fn get_chars(digit: char) -> Vec<char> {
        match digit {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _   => panic!("unsupported chars {}", digit)
        }
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
