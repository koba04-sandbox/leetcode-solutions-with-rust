use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut answers: Vec<Vec<i32>> = vec![];
        if len < 4 {
            return answers;
        }
        let mut duplicate_check_set: HashSet<String> = HashSet::new();
        for i in 0..len {
            let mut set: HashSet<usize> = HashSet::new();
            set.insert(i);
            for j in 0..len {
                if set.get(&j).is_some() {
                    continue;
                }
                set.insert(j);
                for k in 0..len {
                    if set.get(&k).is_some() {
                        continue;
                    }
                    set.insert(k);
                    for l in 0..len {
                        if set.get(&l).is_some() {
                            continue;
                        }
                        set.insert(l);
                        if set.iter().fold(0, |sum, v| sum + nums[*v]) == target {
                            let mut candidate: Vec<i32> = set.iter().map(|v| nums[*v]).collect();
                            candidate.sort();
                            let candidate_to_string = format!("{:?}", candidate);
                            if duplicate_check_set.get(&candidate_to_string).is_none() {
                                answers.push(candidate);
                                duplicate_check_set.insert(candidate_to_string);
                            }
                        }
                        set.remove(&l);
                    }
                    set.remove(&k);
                }
                set.remove(&j);
            }
            set.remove(&i);
        }
        answers
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]]
        );
    }
}
