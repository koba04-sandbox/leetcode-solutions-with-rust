use std::collections::{HashSet, HashMap};

pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut answers: Vec<Vec<i32>> = vec![];
        if len < 4 {
            return answers;
        }
        let mut count_by_value = HashMap::new();
        for n in &nums {
            let count = count_by_value.entry(n).or_insert(0);
            *count += 1;
        }
        let mut duplicate_check_set: HashSet<String> = HashSet::new();
        for (a, _) in count_by_value.iter() {
            for (b, _) in count_by_value.iter() {
                for (c, _) in count_by_value.iter() {
                    let d = target - (*a + *b + *c);
                    if count_by_value.get(&d).is_some() {
                        let mut candidate = vec![**a, **b, **c, d];
                        let mut candidate_count = HashMap::new();
                        for c in &candidate {
                            let count = candidate_count.entry(c).or_insert(0);
                            *count += 1;
                        }
                        if candidate_count.iter().all(|(c, count)| count <= count_by_value.get(c).unwrap()) {
                            candidate.sort();
                            let candidate_to_string = format!("{:?}", candidate);
                            if duplicate_check_set.get(&candidate_to_string).is_none() {
                                answers.push(candidate);
                                duplicate_check_set.insert(candidate_to_string);
                            }
                        }
                    }
                }
            }
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
    #[test]
    fn example2() {
        assert_eq!(
            Solution::four_sum(
                vec![-493,-470,-464,-453,-451,-446,-445,-407,-406,-393,-328,-312,-307,-303,-259,-253,-252,-243,-221,-193,-126,-126,-122,-117,-106,-105,-101,-71,-20,-12,3,4,20,20,54,84,98,111,148,149,152,171,175,176,211,218,227,331,352,389,410,420,448,485],
                1057
            ),
            vec![
                vec![-221, 410, 420, 448],
                vec![-12, 211, 410, 448],
                vec![3, 149, 420, 485],
                vec![4, 148, 420, 485],
                vec![54, 98, 420, 485],
                vec![84, 211, 352, 410],
                vec![98, 218, 331, 410],
                vec![98, 218, 352, 389],
                vec![171, 211, 227, 448]
            ]
        );
    }
}
