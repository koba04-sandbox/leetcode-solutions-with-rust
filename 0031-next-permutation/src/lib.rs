pub struct Solution {

}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut sort_index = 0;
        if let Some(i) = Solution::find_decreasing_element(&nums) {
            // println!("i => {}", i);
            if let Some(j) = Solution::find_larger_element(nums, i) {
                // println!("j => {}", j);
                Solution::swap(nums, i, j);
                // println!("swap {:?}", nums);
                sort_index = i + 1;
            }
        }
        Solution::sort(nums, sort_index);
    }
    fn find_decreasing_element(nums: &Vec<i32>) -> Option<usize> {
        let mut i = nums.len() - 1;
        while i > 0 {
            if nums[i] > nums[i - 1] {
                return Some(i - 1);
            }
            i -= 1;
        }
        None
    }
    fn find_larger_element(nums: &Vec<i32>, target: usize) -> Option<usize> {
        let mut i = nums.len() - 1;
        while i > target {
            if nums[i] > nums[target] {
                return Some(i);
            }
            i -= 1;
        }
        None
    }
    fn swap(nums: &mut Vec<i32>, a: usize, b: usize) {
        let tmp = nums[a];
        nums[a] = nums[b];
        nums[b] = tmp;
    }
    // bubble sort
    pub fn sort(nums: &mut Vec<i32>, start: usize) {
        for i in start..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] > nums[j] {
                    Solution::swap(nums, i, j);
                }
            }
        }
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

    #[test]
    fn example5() {
        let mut target: Vec<i32> = vec![1, 2];
        Solution::next_permutation(&mut target);
        assert_eq!(target, vec![2, 1]);
    }

    #[test]
    fn example6() {
        let mut target: Vec<i32> = vec![1, 3, 2];
        Solution::next_permutation(&mut target);
        assert_eq!(target, vec![2, 1, 3]);
    }

    #[test]
    fn example7() {
        let mut target: Vec<i32> = vec![4,2,0,2,3,2,0];
        Solution::next_permutation(&mut target);
        assert_eq!(target, vec![4,2,0,3,0,2,2]);
    }
}
