pub struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let len1 = word1.len();
        let len2 = word2.len();
        let char1: Vec<char> = word1.chars().collect();
        let char2: Vec<char> = word2.chars().collect();
        let mut i = 0;
        let mut answer = String::new();
        while i < len1 || i < len2 {
            if i < len1 {
                answer.push(char1[i]);
            }
            if i < len2 {
                answer.push(char2[i])
            }
            i += 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_alternately(String::from("abc"), String::from("pqr")),
            String::from("apbqcr")
        );
    }
}