pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return nums.len() as i32;
        }
        let mut i = 0;
        while i < nums.len() {
            if nums[i] == val {
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
            Solution::remove_element(&mut vec![3,2,2,3], 3),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2),
            5
        );
    }

}
