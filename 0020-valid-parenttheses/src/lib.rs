pub struct Solution {}

pub struct Tokenizer {
    stack: Vec<char>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            stack: vec![],
        }
    }
    pub fn next(&mut self, c: char) -> bool {
        match c {
            '(' | '{' | '[' => {
                self.stack.push(c);
                true
            },
            ')' | '}' | ']' => {
                let last = self.stack.pop();
                match last {
                    Some('(') => c == ')',
                    Some('{') => c == '}',
                    Some('[') => c == ']',
                    _ => false
                }
            }
            _ => true
        }
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut tokenizer = Tokenizer::new();
        let chars = s.chars();
        for c in chars {
            if tokenizer.next(c) == false {
                return false;
            }
        }
        tokenizer.stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
    }
    #[test]
    fn example5() {
        assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    }
    #[test]
    fn example6() {
        assert_eq!(Solution::is_valid(String::from("[")), false);
    }
    #[test]
    fn example7() {
        assert_eq!(Solution::is_valid(String::from("")), true);
    }
    #[test]
    fn example8() {
        assert_eq!(Solution::is_valid(String::from("([]")), false);
    }
    #[test]
    fn example9() {
        assert_eq!(Solution::is_valid(String::from("{{)}")), false);
    }
}

