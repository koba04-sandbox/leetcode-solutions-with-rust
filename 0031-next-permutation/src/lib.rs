pub struct Solution {

}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let mut target: Vec<i32> = vec![1,2,3];
        Solution::next_permutation(&mut target);
        assert_eq!(target, vec![1,3,2]);
    }

    #[test]
    fn example2() {
        let mut target: Vec<i32> = vec![3,2,1];
        Solution::next_permutation(&mut target);
        assert_eq!(target, vec![1,2,3]);
    }

    #[test]
    fn example3() {
        let mut target: Vec<i32> = vec![1,1,5];
        Solution::next_permutation(&mut target);
        assert_eq!(target, vec![1,5,1]);
    }

    #[test]
    fn example4() {
        let mut target: Vec<i32> = vec![1];
        Solution::next_permutation(&mut target);
        assert_eq!(target, vec![1]);
    }
}
