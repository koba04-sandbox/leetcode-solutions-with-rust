pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystacks: Vec<char> = haystack.chars().collect();
        let needles: Vec<char> = needle.chars().collect();
        let mut current = 0;
        let mut candidate = -1;
        let mut target = 0;
        let mut is_match = false;
        let mut reached = false;

        if needle == "" {
            return 0
        }

        while current < haystack.len() {
            // println!("{}:{}, {}:{}", haystacks[current], needles[target], current, target);
            if haystacks[current] == needles[target] {
                if is_match == false {
                    candidate = current as i32;
                }
                is_match = true;
                target += 1;
                if target == needle.len() {
                    reached = true;
                    break;
                }
            } else {
                target = 0;
                if is_match {
                    current = candidate as usize;
                }
                candidate = -1;
                is_match = false
            }

            current += 1;
        }
        // println!("result {}:{}", target, needle.len());
        if reached {
            return candidate
        } else {
            return -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::str_str(String::from("hello"), String::from("ll")),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::str_str(String::from("aaaaa"), String::from("bba")),
            -1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::str_str(String::from(""), String::from("")),
            0
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::str_str(String::from(""), String::from("a")),
            -1
        );
    }
    #[test]
    fn example5() {
        assert_eq!(
            Solution::str_str(String::from("a"), String::from("")),
            0
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            Solution::str_str(String::from("aaa"), String::from("aaaa")),
            -1
        );
    }

    #[test]
    fn example7() {
        assert_eq!(
            Solution::str_str(String::from("mississippi"), String::from("issip")),
            4
        );
    }
}


