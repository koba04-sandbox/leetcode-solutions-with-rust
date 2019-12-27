pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![vec![0]]
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exampl1() {
        assert_eq!(
            Solution::three_sum(
                vec![-1, 0, 1, 2, -1, -4]
            ),
            vec![
                vec![-1, 0, 1],
                vec![-1, -1, 2]
            ]
        );
    }
}
