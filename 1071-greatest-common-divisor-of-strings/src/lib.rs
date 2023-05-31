pub struct Solution;

impl Solution {
    fn check(target: &str, str: &str) -> bool {
        let len = str.len();
        let mut current = 0;
        while current < target.len() {
            if &target[current..current + len] != str {
                println!("{}:{}", &target[current..current + len], str);
                return false;
            }
            current += len;
        }
        true

    }
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let base;
        let target;
        if str1.len() < str2.len() {
            base = str1;
            target = str2;
        } else {
            base = str1;
            target = str2;
        }

        let mut len = base.len();
        while len > 0 {
            let current = &base[0..len];
            if base.len() % len == 0 && target.len() % len == 0 {
                // check
                if Solution::check(&base, current) && Solution::check(&target, current) {
                    return String::from(current);
                }

            }
            len -= 1;
        }
        String::from("")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("ABCABC"), String::from("ABC")),
            String::from("ABC")
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::gcd_of_strings(String::from("ABABAB"), String::from("ABAB")),
            String::from("AB")
        )
    }
}
