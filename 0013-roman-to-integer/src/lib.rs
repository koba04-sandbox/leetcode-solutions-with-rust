
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
        1
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