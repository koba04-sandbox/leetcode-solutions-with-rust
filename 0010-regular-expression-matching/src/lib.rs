enum Mode {
    Char,
    Wildcard,
    Asterisk,
}

pub fn is_match(source: String, regexp: String) -> bool {
    let sources: Vec<char> = source.chars().collect();
    let regexps: Vec<char> = regexp.chars().collect();
    let mut result = true;
    let mut current_mode;
    let mut source_index = 0;
    let mut regexp_index = 0;
    let mut prev_regexp_char = ' ';
    while source_index < sources.len() && regexp_index < regexps.len() {
        let source = sources[source_index];
        let regexp = regexps[regexp_index];
        println!("{}, {}", source, regexp);

        if regexp == '.' {
            current_mode = Mode::Wildcard;
            prev_regexp_char = regexp;
        } else if regexp == '*' {
            current_mode = Mode::Asterisk;
        } else {
            current_mode = Mode::Char;
            prev_regexp_char = regexp;
        }
        match current_mode {
            Mode::Char => {
                if source != regexp {
                    if regexps[regexp_index + 1] != '*' {
                        println!("not match {}, {}", source, regexp);
                        result = false;
                        break;
                    } else {
                        regexp_index = regexp_index + 1;
                    }
                } else {
                    source_index = source_index + 1;
                    regexp_index = regexp_index + 1;
                }
            },
            Mode::Wildcard => {
                // noop
                source_index = source_index + 1;
                regexp_index = regexp_index + 1;
            },
            Mode::Asterisk => {
                println!("asterisk {}, {}", prev_regexp_char, source);
                // noop
                if prev_regexp_char == source || prev_regexp_char == '.' {
                    source_index = source_index + 1;
                }
                regexp_index = regexp_index + 1;
            }
        }
    }
    if source_index != sources.len() || regexp_index != regexp.len() {
        result = false;
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
    #[test]
    fn example6() {
        assert_eq!(is_match(String::from("aaa"), String::from("aaaa")), false);
    }
    #[test]
    fn example7() {
        assert_eq!(is_match(String::from("aaba"), String::from("ab*a*c*a")), false);
    }
}
