pub fn reverse(x: i32) -> i32 {
    let sign = if x < 0 { -1 } else { 1 };
    let mut nums: Vec<char> = x
        .to_string()
        .chars()
        .collect()
        ;
    nums.reverse();
    if sign == -1 {
        nums.pop();
    }

    let mut is_first = true;
    let mut answer = String::new();
    for n in nums {
        if is_first && n == '0' {
            continue;
        }
        is_first = false;
        answer.push(n);
    }
    // println!("answer is {}", answer);
    match answer.parse::<i32>() {
        Ok(n) => n * sign,
        Err(_) => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn example2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn example3() {
        assert_eq!(reverse(120), 21);
    }

    #[test]
    fn example4() {
        assert_eq!(reverse(0), 0);
    }

    #[test]
    fn example5() {
        assert_eq!(reverse(1534236469), 0);
    }
}
