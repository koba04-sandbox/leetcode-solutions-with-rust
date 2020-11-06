pub struct Solution {}

impl Solution {
    // I know this problmen should not use the / operator...
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let answer: i64 = dividend as i64 / divisor as i64;
        if answer > std::i32::MAX.into() {
            return std::i32::MAX;
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::divide(10, 3),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::divide(7, -3),
            -2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::divide(0, 1),
            0
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            Solution::divide(1, 1),
            1
        );
    }
    #[test]
    fn example5() {
        assert_eq!(
            Solution::divide(-2147483648, -1),
            2147483647
        )
    }
}
