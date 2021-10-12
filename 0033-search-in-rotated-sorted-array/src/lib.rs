struct Solution {

}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::search(vec![4,5,6,6,0,1,2], 0),
            4
        )
    }
}
