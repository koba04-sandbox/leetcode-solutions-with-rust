pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            // Given array nums = [-1, 2, 1, -4], and target = 1.
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 1),
            2
        );
    }
}
