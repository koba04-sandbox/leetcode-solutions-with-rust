pub fn length_of_longest_substring(s: String) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3)
    }

    #[test]
    fn example_2() {
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1)
    }

    #[test]
    fn example_3() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3)
    }
}
