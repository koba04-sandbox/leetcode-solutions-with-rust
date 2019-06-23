fn main() {
    println!("Hello, world!");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let last = nums.len();
    for i in 0..last {
        for j in 0..last {
            if i != j && nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9) , vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6) , vec![1, 2]);
        assert_eq!(two_sum(vec![3, 2, 3], 6) , vec![0, 2]);
    }
}