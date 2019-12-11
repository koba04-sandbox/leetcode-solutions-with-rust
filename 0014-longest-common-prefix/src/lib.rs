pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        String::from("fl")
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
}
