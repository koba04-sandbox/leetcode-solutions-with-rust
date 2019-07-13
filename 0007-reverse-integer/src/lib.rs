pub fn reverse(x: i32) -> i32 {
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn example2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn example3() {
        assert_eq!(reverse(120), 21);
    }
}
