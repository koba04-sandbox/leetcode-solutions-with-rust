pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut answer: (i32, Vec<i32>) = (0, vec![]);
        let len = nums.len();
        for i in 0..len {
            for j in i + 1..len {
                for k in j + 1..len {
                    let sum = nums[i] + nums[j] + nums[k];
                    if answer.1.len() == 0 || Solution::abs(&sum, &target) < Solution::abs(&answer.0, &target) {
                        answer = (sum, vec![nums[i], nums[j], nums[k]]);
                    }
                    // println!("{},{},{},{:?}", nums[i], nums[j], nums[k], answer);
                }
            }
        }
        answer.0
    }
    fn abs(num: &i32, target: &i32) -> i32 {
        (num - target).abs()
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::three_sum_closest(vec![-1, 2, 1, -4], 1),
            2
        );
    }
    #[test]
    fn example2() {
        assert_eq!(
            Solution::three_sum_closest(vec![1, 1, 1, 0], -100),
            2
        )
    }
    #[test]
    fn example3() {
        assert_eq!(
            Solution::three_sum_closest(vec![0, 2, 1, -3], 1),
            0
        )
    }
}
