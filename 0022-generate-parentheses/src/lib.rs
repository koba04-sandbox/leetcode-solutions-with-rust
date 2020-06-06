pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = vec![];
        Solution::recursive(0, &mut Vec::with_capacity((n * 2) as usize), &mut answer);
        answer
    }
    fn recursive(position: usize, tmp: &mut Vec<char>, answer: &mut Vec<String>) {
        println!("{}:{:?}:{}", position, tmp, tmp.capacity());
        if tmp.capacity() == position as usize {
            if Solution::check(tmp) {
                let mut s = String::new();
                for t in tmp {
                    s.push(*t)
                }
                answer.push(s);
            }
        } else {
            if tmp.len() <= position {
                tmp.push(')');
            } else {
                tmp[position] = ')';
            }
            Solution::recursive(position + 1, tmp, answer);
            if tmp.len() <= position {
                tmp.push('(');
            } else {
                tmp[position] = '(';
            }
            Solution::recursive(position + 1, tmp, answer);
        }
    }
    fn check(target: &Vec<char>) -> bool {
        let mut count = 0;
        for t in target {
            match *t {
                '(' => {
                    count += 1;
                },
                ')' => {
                    if count == 0 {
                        return false;
                    }
                    count -= 1;
                },
                other => {
                    panic!("unsupported char {}", other)
                }
            }
        }
        count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        let actual = Solution::generate_parenthesis(3).sort();
        let expected = ["((()))", "(()())", "(())()", "()(())", "()()()"].sort();
        assert_eq!(actual, expected);
    }
}
