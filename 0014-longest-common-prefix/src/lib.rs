pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut answer = String::from("");
        if strs.len() == 0 {
            return answer;
        }

        let char_len = strs[0].len();
        let mut char_index = 0;
        while char_index < char_len {
            let mut target_char = None;
            for current_str in strs.iter() {
                let current_chars: Vec<char> = current_str.chars().collect();
                if current_chars.len() <= char_index {
                    return answer;
                }
                if let Some(c) = target_char {
                    if c != current_chars[char_index] {
                        return answer;
                    }
                } else {
                    target_char = Some(current_chars[char_index]);
                }
            }
            answer = format!("{}{}", answer, target_char.unwrap());
            char_index = char_index + 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec![String::from("flower"), String::from("flow"), String::from("flight")]
            ),
            String::from("fl")
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec![String::from("dog"), String::from("racecar"), String::from("car")]
            ),
            String::from("")
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec![]
            ),
            String::from("")
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::longest_common_prefix(
                vec![String::from("a")]
            ),
            String::from("a")
        );
    }
}
