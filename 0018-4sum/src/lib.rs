use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut answers: Vec<Vec<i32>> = vec![];
        if len < 4 {
            return answers;
        }
        let count_by_value = Solution::create_count_by_value(&nums);
        let mut duplicate_check_set: HashSet<String> = HashSet::new();
        for a in count_by_value.keys() {
            for b in count_by_value.keys() {
                let candidate = vec![*a, *b];
                if !Solution::is_available(&candidate, &count_by_value) {
                    continue;
                }
                for c in count_by_value.keys() {
                    let d = target - (*a + *b + *c);
                    if count_by_value.get(&d).is_some() {
                        let mut candidate = vec![*a, *b, *c, d];
                        if Solution::is_available(&candidate, &count_by_value) {
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
    fn create_count_by_value(nums: &Vec<i32>) -> HashMap<i32, i32> {
        let mut count_by_value = HashMap::new();
        for &n in nums {
            let count = count_by_value.entry(n).or_insert(0);
            *count += 1;
        }
        count_by_value
    }
    fn is_available(nums: &Vec<i32>, count_by_value: &HashMap<i32, i32>) -> bool {
        let by_value = Solution::create_count_by_value(&nums);
        by_value
            .iter()
            .all(|(c, count)| count <= count_by_value.get(c).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn example1() {
        let mut actual = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        let mut expected = vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
    #[test]
    fn example2() {
        let mut actual = Solution::four_sum(
            vec![
                -493, -470, -464, -453, -451, -446, -445, -407, -406, -393, -328, -312, -307, -303,
                -259, -253, -252, -243, -221, -193, -126, -126, -122, -117, -106, -105, -101, -71,
                -20, -12, 3, 4, 20, 20, 54, 84, 98, 111, 148, 149, 152, 171, 175, 176, 211, 218,
                227, 331, 352, 389, 410, 420, 448, 485,
            ],
            1057,
        );
        let mut expected = vec![
            vec![-221, 410, 420, 448],
            vec![-12, 211, 410, 448],
            vec![3, 149, 420, 485],
            vec![4, 148, 420, 485],
            vec![54, 98, 420, 485],
            vec![84, 211, 352, 410],
            vec![98, 218, 331, 410],
            vec![98, 218, 352, 389],
            vec![171, 211, 227, 448],
        ];

        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
