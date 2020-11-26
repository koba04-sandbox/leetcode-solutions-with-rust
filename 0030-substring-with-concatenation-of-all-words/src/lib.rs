use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut word_map = Solution::create_hash_map(words);
        println!("{:?}", word_map);
        vec![]
    }
    pub fn create_hash_map(words: Vec<String>) -> HashMap::<String, i32> {
        let mut word_map = HashMap::<String, i32>::new();
        for word in words {
            let count = match word_map.get(&word) {
                Some(count) => count + 1,
                None => 1
            };
            word_map.insert(word, count);
        }
        word_map
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoothefoobarman"),
                vec![String::from("foo"), String::from("bar")]
            ),
            vec![0, 9]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("word"),
                ]
            ),
            vec![]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoofoobarthefoobarman"),
                vec![
                    String::from("bar"),
                    String::from("foo"),
                    String::from("the"),
                ]
            ),
            vec![6, 9, 12]
        );
    }
}
