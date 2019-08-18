pub struct Solution {}

// This implemetation is based on the solution😭
impl Solution {
    pub fn is_match(source: String, regexp: String) -> bool {
        // println!("{}:{}", source, regexp);

        let sources: Vec<char> = source.chars().collect();
        let regexps: Vec<char> = regexp.chars().collect();
        // base case
        if regexps.len() == 0 {
            return sources.len() == 0;
        }

        // compare
        let matched = sources.len() > 0 && (sources[0] == regexps[0] || regexps[0] == '.');

        // if the next char is *, we should skip the regexp or compare the regexp with a source shifting a current char
        if regexps.len() > 1 && regexps[1] == '*' {
            return
                Solution::is_match(sources.iter().collect(), regexps[2..].iter().collect())
                ||
                matched && Solution::is_match(sources[1..].iter().collect(), regexps.iter().collect())
            ;
        }
        matched && Solution::is_match(sources[1..].iter().collect(), regexps[1..].iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a*")), true);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::is_match(String::from("ab"), String::from(".*")), true);
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::is_match(String::from("aab"), String::from("c*a*b")), true);
    }
    #[test]
    fn example5() {
        assert_eq!(Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")), false);
    }
    #[test]
    fn example6() {
        assert_eq!(Solution::is_match(String::from("aaa"), String::from("aaaa")), false);
    }
    #[test]
    fn example7() {
        assert_eq!(Solution::is_match(String::from("aaba"), String::from("ab*a*c*a")), false);
    }
    #[test]
    fn example8() {
        assert_eq!(Solution::is_match(String::from("aaa"), String::from("ab*a*c*a")), true);
    }
    #[test]
    fn example9() {
        assert_eq!(Solution::is_match(String::from("abcd"), String::from("d*")), false);
    }
    #[test]
    fn example10() {
        assert_eq!(Solution::is_match(String::from("ab"), String::from(".*c")), false);
    }
    #[test]
    fn example11() {
        assert_eq!(Solution::is_match(String::from("aaaaaaaaaaaaab"), String::from("a*a*a*a*a*a*a*a*a*a*c")), false);
    }
    #[test]
    fn example12() {
        assert_eq!(Solution::is_match(String::from(""), String::from("c*c*")), true);
    }
}
