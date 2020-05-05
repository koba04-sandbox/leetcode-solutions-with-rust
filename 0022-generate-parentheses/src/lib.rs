pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            ["((()))", "(()())", "(())()", "()(())", "()()()"]
        )
    }
}
