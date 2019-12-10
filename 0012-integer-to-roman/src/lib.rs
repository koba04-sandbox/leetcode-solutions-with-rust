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
    pub fn int_to_roman(num: i32) -> String {
        let roman = [
            ['I', 'V'],
            ['X', 'L'],
            ['C', 'D'],
            ['M', '_'],
            ['_', '_']
        ];
        let nums: Vec<char> = num
        .to_string()
        .chars()
        .rev()
        .collect()
        ;

        let mut answer = String::new();
        let mut i = 0;
        for n in nums {
            // println!("next is {}", n);
            let current = roman[i];
            let next = roman[i + 1];

            match n.to_digit(10) {
                Some(4) => answer = format!("{}{}{}", current[0].to_string(), current[1].to_string(), answer),
                Some(5) => answer = format!("{}{}", current[1].to_string(), answer),
                Some(9) => answer = format!("{}{}{}", current[0].to_string(), next[0].to_string(), answer),
                Some(n) => {
                    let m = n % 5;
                    // println!("n:{}, m:{}", n, m);
                    for _ in 0..m {
                        answer = format!("{}{}", current[0].to_string(), answer);
                    }
                    if n > 5 {
                        answer = format!("{}{}", current[1].to_string(), answer);
                    }
                },
                None => panic!("something went wrong")
            }
            // println!("answer is {}", answer);
            i = i +1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::int_to_roman(3), "III");
    }
    #[test]
    fn example2() {
        assert_eq!(Solution::int_to_roman(4), "IV");
    }
    #[test]
    fn example3() {
        assert_eq!(Solution::int_to_roman(9), "IX");
    }
    #[test]
    fn example4() {
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }
    #[test]
    fn example5() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
