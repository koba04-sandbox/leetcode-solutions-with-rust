struct Solution {}

impl Solution {
    fn search(nums: &Vec<i32>, target: i32, start: i32, end: i32) -> i32 {
        if start > end {
            if start as usize >= nums.len() || nums[start as usize] > target {
                return start;
            } else {
                return start + 1;
            }
        }
        let middle = (start + end) / 2;
        if nums[middle as usize] == target {
            return middle;
        } else if nums[middle as usize] < target {
            return Solution::search(nums, target, middle + 1, end);
        } else {
            return Solution::search(nums, target, start, end - 1);
        }
    }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Solution::search(&nums, target, 0, (nums.len() - 1) as i32)
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

    #[test]
    fn example2() {
        let nums = vec![1,3,5,6];
        let target = 7;
        let expected = 4;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }
}
