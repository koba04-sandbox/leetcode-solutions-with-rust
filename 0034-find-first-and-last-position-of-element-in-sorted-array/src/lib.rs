struct Solution {}

impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32, start: i32, end: i32) -> Option<i32> {
        if start > end {
            return None;
        }

        let middle = (end + start) / 2;
        let v = nums[middle as usize];
        if v == target {
            return Some(middle)
        } else if v < target {
            return Solution::binary_search(nums, target, middle + 1, end);
        } else {
            return Solution::binary_search(nums, target, start, end - 1);
        }
    }
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let last = nums.len() as i32 - 1;

        if let Some(base) = Solution::binary_search(&nums, target, 0, last) {
            let mut start = base;
            let mut end = base;
            let mut distance = 1;
            let mut found = true;
            while found {
                // println!("{}:{}:{}", start, end, distance);
                found = false;

                // start
                let i = base - distance;
                if i >= 0 && nums[i as usize] == target {
                    start = i;
                    found = true;
                }

                // end
                let i = base + distance;
                if i <= last && nums[i as usize] == target {
                    end = i;
                    found = true;
                }
                distance += 1;
            }
            return vec![start, end];
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        )
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1])
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0])
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::search_range(vec![1,2,3,3,3,3,4,5,9], 3), vec![2, 5]);
    }
}
