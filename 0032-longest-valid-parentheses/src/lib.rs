pub struct Solution {}


#[derive(Debug)]
pub struct Stack {
    list: Vec<i32>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            list: vec![]
        }
    }
    pub fn push(&mut self, i: i32) {
        self.list.push(i);
    }
    pub fn pop(&mut self) -> Option<i32> {
         self.list.pop()
    }
    pub fn last(&self) -> Option<&i32> {
        self.list.last()
    }
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut longest = 0;
        let mut stack = Stack::new();
        stack.push(-1);
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            let index = i as i32;
            if chars[i] == '(' {
                stack.push(index);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(index);
                } else if let Some(last) = stack.last() {
                    let candidate = index - last;
                    if candidate > longest {
                        longest = candidate;
                    }
                }
            }
        }
        longest
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
            0
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from(")(")),
            0
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("()")),
            2
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("()(())")),
            6
        );
    }

    #[test]
    fn example7() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("(())(")),
            4
        );
    }

    #[test]
    fn example8() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("()(()")),
            2
        );
    }

    #[test]
    fn example9() {
        assert_eq!(
            Solution::longest_valid_parentheses(String::from("(())()(()((")),
            6
        );
    }
}
