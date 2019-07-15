pub fn my_atoi(str: String) -> i32 {
    let chars = str.chars();
    let mut answer = String::new();
    let mut is_start = true;
    let mut is_passed_number = false;
    for c in chars {
        if is_start {
            if c == ' ' {
                continue;
            }
            if c == '-' || c == '+' {
                is_start = false;
                answer.push(c);
                continue;
            }
            if !c.is_ascii_digit() {
                answer = String::from("0");
                break;
            }
        } else {
            if !c.is_ascii_digit() {
                break;
            }
        }
        is_start = false;
        is_passed_number = true;
        answer.push(c);
    }
    if is_passed_number == false {
        return 0;
    }
    match answer.parse::<i32>() {
        Ok(a) => a,
        Err(_) => {
            if answer.starts_with('-') {
                std::i32::MIN
            } else {
                std::i32::MAX
            }
        }
    }
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

    #[test]
    fn example6() {
        assert_eq!(my_atoi(String::from("   +0 123")), 0);
    }

    #[test]
    fn example7() {
        assert_eq!(my_atoi(String::from("2147483648")), 2147483647);
    }
}
