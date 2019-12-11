use std::collections::HashMap;

pub struct Solution {}

// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.


impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_map = HashMap::new();
        roman_map.insert('I', 1);
        roman_map.insert('V', 5);
        roman_map.insert('X', 10);
        roman_map.insert('L', 50);
        roman_map.insert('C', 100);
        roman_map.insert('D', 500);
        roman_map.insert('M', 1000);

        let mut answer = 0;
        let chars: Vec<char> = s.chars().rev().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            let n = roman_map.get(&c).unwrap();
            if i != chars.len() - 1 {
                let next = chars[i + 1];
                match c {
                    'X' | 'V' => {
                        if next == 'I' {
                            answer = answer - 1;
                            i = i + 1;
                        }
                    },
                    'C' | 'L' => {
                        if next == 'X' {
                            answer = answer - 10;
                            i = i + 1;
                        }
                    },
                    'M' | 'D' => {
                        if next == 'C' {
                            answer = answer - 100;
                            i = i + 1;
                        }

                    },
                    _ => {}
                }
            }

            answer = answer + n;
            i = i + 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    }
    #[test]
    fn example5() {
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}