pub struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        0
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
}
