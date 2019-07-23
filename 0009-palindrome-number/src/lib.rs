pub fn is_palindrome(x: i32) -> bool {
    let nums: Vec<char> = x.to_string().chars().collect();

    let mut start = 0;
    let mut end = nums.len() - 1;
    while end > 0 && start <= end {
        if nums[start] != nums[end] {
            return false;
        }
        start = start + 1;
        end = end - 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn example2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    fn example3() {
        assert_eq!(is_palindrome(10), false);
    }

    #[test]
    fn example4() {
        assert_eq!(is_palindrome(0), true);
    }
}

