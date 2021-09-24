use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // let mut word_map = Solution::create_hash_map(&words);
        let len = words[0].len();
        let mut index_map = HashMap::<usize, String>::new();
        for word in &words {
            for (index, __) in s.match_indices(word) {
                index_map.insert(index, word.clone());
            }
        }
        let mut answer: Vec<i32> = Vec::new();
        let mut used: Vec<String> = Vec::new();
        let mut indices = index_map.keys().collect::<Vec<&usize>>();
        indices.sort();
        for start_index in 0..indices.len() {
            let mut prev_index = *indices[start_index];
            for i in start_index..indices.len() {
                let key = indices[i];
                let target_string = index_map.get(key).unwrap();
                if *key != (prev_index + len) && used.binary_search(target_string).is_ok() {
                    break;
                }
                used.push(target_string.clone());
                prev_index = *key;
            }
            if used.len() == words.len() {
                answer.push(*indices[start_index] as i32);
            }
            used = Vec::new();
        }

        println!("{:?}", index_map);
        answer
    }
    pub fn create_hash_map(words: &Vec<String>) -> HashMap::<String, i32> {
        let mut word_map = HashMap::<String, i32>::new();
        for word in words {
            let count = match word_map.get(word) {
                Some(count) => count + 1,
                None => 1
            };
            word_map.insert(word.clone(), count);
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
