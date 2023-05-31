pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            [true, true, true, false, true]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            [true, false, false, false, false]
        );
    }
}
