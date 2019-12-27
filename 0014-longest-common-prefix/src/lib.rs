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
            println!("hoge");
            let mut str_index = 0;
            let mut target_char = ' ';
            for current_str in strs.iter() {
                let current_chars: Vec<char> = current_str.chars().collect();
                if current_chars.len() <= char_index {
                    return answer;
                }
                if str_index == 0 {
                    target_char = current_chars[char_index];
                } else {
                    if target_char != current_chars[char_index] {
                        return answer;
                    }
                }
                str_index = str_index + 1;
            }
            answer = format!("{}{}", answer, target_char);
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
