pub struct Solution {}

pub struct Tokenizer {
    is_valid: bool,
    stack: Vec<char>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            is_valid: true,
            stack: vec![],
        }
    }
    pub fn next(&mut self, c: char) {
        match c {
            '(' | '{' | '[' => {
                self.stack.push(c);
            },
            ')' | '}' | ']' => {
                let last = self.stack.pop();
                match last {
                    Some('(') => {
                        self.is_valid = c == ')';
                    },
                    Some('{') => {
                        self.is_valid = c == '}';
                    },
                    Some('[') => {
                        self.is_valid = c == ']';
                    },
                    _ => {
                        self.is_valid = false;
                    }
                }
            }
            _ => {}
        }
    }
    pub fn is_valid(&self) -> bool {
        self.is_valid && self.stack.len() == 0
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut tokenizer = Tokenizer::new();
        let chars = s.chars();
        for c in chars {
            tokenizer.next(c);
            if tokenizer.is_valid == false {
                return false;
            }
        }
        tokenizer.is_valid()
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

