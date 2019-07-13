pub fn my_atoi(str: String) -> i32 {
    str.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(my_atoi(String::from("42")), 42);
    }

    #[test]
    fn example2() {
        assert_eq!(my_atoi(String::from("   -42")), -42);
    }

    #[test]
    fn example3() {
        assert_eq!(my_atoi(String::from("4193 with words")), 4193);
    }

    #[test]
    fn example4() {
        assert_eq!(my_atoi(String::from("words and 987")), 0);
    }

    #[test]
    fn example5() {
        assert_eq!(my_atoi(String::from("-91283472332")), -2147483648);
    }
}
