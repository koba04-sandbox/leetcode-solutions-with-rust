enum Mode {
    Char(char),
    Wildcard,
    Asterisk,
}

pub fn is_match(s: String, p: String) -> bool {
    let mut s: Vec<char> = s.chars().collect();
    let mut p: Vec<char> = p.chars().collect();
    let mut result = true;
    let mut current_mode;
    loop {
        if let Some(b) = s.pop() {
            if let Some(a) = p.pop() {
                if a == '.' {
                    current_mode = Mode::Wildcard;
                } else if a == '*' {
                    current_mode = Mode::Asterisk;
                } else {
                    current_mode = Mode::Char(a);
                }
                match current_mode {
                    Mode::Char(a) => {
                        if a != b {
                            println!("not match {}, {}", a, b);
                            result = false;
                            break;
                        }
                    },
                    Mode::Asterisk => {
                        // noop
                    },
                    Mode::Wildcard => {
                        // noop
                    }
                }
            } else {
                result = false;
                break;
            }
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(is_match(String::from("aa"), String::from("a")), false);
    }
    #[test]
    fn example2() {
        assert_eq!(is_match(String::from("aa"), String::from("a*")), true);
    }
    #[test]
    fn example3() {
        assert_eq!(is_match(String::from("ab"), String::from(".*")), true);
    }
    #[test]
    fn example4() {
        assert_eq!(is_match(String::from("aab"), String::from("c*a*b")), true);
    }
    #[test]
    fn example5() {
        assert_eq!(is_match(String::from("mississippi"), String::from("mis*is*p*.")), false);
    }
}
