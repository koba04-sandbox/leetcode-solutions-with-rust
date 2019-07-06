pub fn longest_palindrome(s: String) -> String {
    s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(longest_palindrome(String::from("babad")), "bab");
    }
    #[test]
    fn example2() {
        assert_eq!(longest_palindrome(String::from("cbbd")), "bb");
    }
}
