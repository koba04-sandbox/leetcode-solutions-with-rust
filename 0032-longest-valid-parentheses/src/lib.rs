struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("(()")),
            2 // "()"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from(")()())")),
            4 // "()()"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("")),
            0 // ""
        );
    }
}
