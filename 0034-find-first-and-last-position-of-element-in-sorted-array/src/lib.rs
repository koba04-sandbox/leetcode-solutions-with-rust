struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        vec![-1,-1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::search_range(vec![5,7,7,8,8,10], 8),
            vec![3,4]
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::search_range(vec![5,7,7,8,8,10], 6),
            vec![-1,-1]
        )
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::search_range(vec![], 0),
            vec![-1,-1]
        )
    }
}
