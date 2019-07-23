pub fn is_palindrome(x: i32) -> bool {
    x == x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn example2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn example3() {
        assert_eq!(is_palindrome(10), false);
    }
}
