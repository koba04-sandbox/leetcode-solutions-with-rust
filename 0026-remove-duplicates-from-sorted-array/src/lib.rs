pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![1,1,2]),
            2
        )
    }
    fn example2() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]),
            4
        )
    }
}
