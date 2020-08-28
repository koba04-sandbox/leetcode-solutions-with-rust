pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::str_str(String::from("hello"), String::from("ll")),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::str_str(String::from("aaaaa"), String::from("bba")),
            -1
        );
    }
}
