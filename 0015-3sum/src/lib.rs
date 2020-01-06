pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut answers = vec![];
        for i in 0..len {
            for j in i + 1..len {
                for k in j + 1..len {
                    // println!("{}:{}:{}", i, j, k);
                    let ans = nums[i] + nums[j] + nums[k];
                    if ans == 0 {
                        let mut answer = vec![nums[i], nums[j], nums[k]];
                        answer.sort();
                        if Solution::check(&answers, &answer) == false {
                            answers.push(answer);
                        }
                    }
                }
            }
        }
        answers
    }
    fn check(answers: &Vec<Vec<i32>>, candidate: &Vec<i32>) -> bool {
        // println!("compare {:?}:{:?}", answers, candidate);
        'outer: for answer in answers {
            for (i, a) in answer.iter().enumerate() {
                if a != &candidate[i] {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exampl1() {
        assert_eq!(
            Solution::three_sum(
                vec![-1, 0, 1, 2, -1, -4]
            ),
            vec![
                vec![-1, 0, 1],
                vec![-1, -1, 2]
            ]
        );
    }
}
