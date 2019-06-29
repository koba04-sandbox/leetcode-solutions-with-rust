use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;
    let mut current = 0;
    let mut used_chars = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    let mut start_index = 0;
    let mut i = 0;
    while i < chars.len() {
        let a = chars[i];
        // println!("char is {}", a);
        if used_chars.get(&a) == None {
            current += 1;
            used_chars.insert(a, 1);
            if current > longest {
                longest = current;
            }
            i = i + 1;
        } else {
            if current > longest {
                longest = current;
            }
            current = 0;
            used_chars = HashMap::new();
            start_index = start_index + 1;
            i = start_index;
        }
    }
    longest
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3)
    }

    #[test]
    fn example_2() {
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1)
    }

    #[test]
    fn example_3() {
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3)
    }

    #[test]
    fn example_4() {
        assert_eq!(length_of_longest_substring(String::from("")), 0)
    }

    #[test]
    fn example_5() {
        assert_eq!(length_of_longest_substring(String::from(" ")), 1)
    }

    #[test]
    fn example_6() {
        assert_eq!(length_of_longest_substring(String::from("dvdf")), 3)
    }

}
