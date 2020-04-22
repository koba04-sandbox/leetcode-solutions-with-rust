pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        true
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
}

