pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut i = 0;
        while i < height.len() {
            let mut inner_index = i + 1;
            while inner_index < height.len() - 1 {
                inner_index = inner_index + 1;
                let x = inner_index - i;
                let y = if height[i] < height[inner_index] { height[i] } else { height[inner_index] };
                let area = x as i32 * y;
                if area > max {
                    max = area;
                }
            }
            i = i + 1;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}
