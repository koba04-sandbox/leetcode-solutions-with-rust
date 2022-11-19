struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let nums = vec![1,3,5,6];
        let target = 5;
        let expected = 2;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }

    fn example2() {
        let nums = vec![1,3,5,6];
        let target = 7;
        let expected = 4;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }
}
