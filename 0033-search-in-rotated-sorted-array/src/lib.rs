use std::collections::HashSet;

struct Solution {

}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;

        while start <= end {
            let middle = (end - start) / 2 + start;

            // println!("current start:{}({}), end:{}({}), middle:{}({})", start, nums[start], end, nums[end], middle, nums[middle]);

            if nums[middle] == target {
                return middle as i32;
            } else if nums[start] == target {
                return start as i32;
            } else if nums[end] == target {
                return end as i32;
            }

            if start == end {
                return -1;
            }

            // in the front
            if nums[start] < nums[end] && nums[start] < target && target < nums[middle] {
                end = middle - 1;
            // in the latter
            } else if nums[middle] < nums[end] && nums[middle] < target && target < nums[end] {
                start = middle + 1;
            // flip
            } else {
                start += 1;
                end -= 1;
            }
        }
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::search(vec![4,5,6,7,0,1,2], 0),
            4
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::search(vec![4,5,6,7,0,1,2], 3),
            -1
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::search(vec![1], 0),
            -1
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::search(vec![1], 1),
            0
        )
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::search(vec![1,3,5], 5),
            2
        )
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::search(vec![4,5,6,7,0,1,2], 5),
            1
        )
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::search(vec![4,5,6,7,0,1,2], 6),
            2
        )
    }

    #[test]
    fn test8() {
        assert_eq!(
            Solution::search(vec![8,1,2,3,4,5,6,7], 6),
            6
        )
    }
}
