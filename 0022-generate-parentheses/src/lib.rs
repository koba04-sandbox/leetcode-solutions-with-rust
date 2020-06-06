pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = vec![];
        Solution::recursive(0, &mut Vec::with_capacity((n * 2) as usize), &mut answer, 0, 0);
        answer
    }
    fn recursive(position: usize, tmp: &mut Vec<char>, answer: &mut Vec<String>, open: usize, close: usize) {
        // println!("{}:{:?}:{}", position, tmp, tmp.capacity());
        let capacity = tmp.capacity();
        if capacity == position {
            if Solution::check(tmp) {
                let mut s = String::new();
                for t in tmp {
                    s.push(*t)
                }
                answer.push(s);
            }
        } else {
            if open > tmp.capacity() * 2 || close > tmp.capacity() * 2 {
                return;
            }
            if tmp.len() <= position {
                tmp.push(')');
            } else {
                tmp[position] = ')';
            }
            Solution::recursive(position + 1, tmp, answer, open + 1, close);
            if tmp.len() <= position {
                tmp.push('(');
            } else {
                tmp[position] = '(';
            }
            Solution::recursive(position + 1, tmp, answer, open, close + 1);
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
