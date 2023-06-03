pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let expected = true;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), expected);
    }

    #[test]
    fn example2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let expected = false;
        assert_eq!(Solution::can_place_flowers(flowerbed, n), expected);}
}
