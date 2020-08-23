pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut i = 0;
        while i < nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }
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

    #[test]
    fn example2() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]),
            5
        )
    }
}
