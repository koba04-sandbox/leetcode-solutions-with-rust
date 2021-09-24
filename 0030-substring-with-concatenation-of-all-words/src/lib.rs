use std::collections::{HashMap,HashSet};

pub struct Solution {
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_map = Solution::create_word_map(&words);
        let index_map = Solution::create_index_map(&s, &words);
        let len = words[0].len();

        let mut answer: Vec<i32> = Vec::new();

        let mut indices = index_map.keys().collect::<Vec<&usize>>();
        indices.sort();
        let indices_len = indices.len();

        for start_index in 0..indices_len {
            let mut used_map = HashMap::<String, i32>::new();
            let mut used_count = 0;
            let mut prev_index = None;
            for i in start_index..indices_len {
                let key = indices[i];
                let target_string = index_map.get(key).unwrap();
                if let Some(prev) = prev_index {
                    if *key != (prev + len) {
                        // println!("not matched index {} => {}:{}:{}", *indices[start_index],  *key, prev, len);
                        continue;
                    }
                }

                // println!("find {}", target_string);
                if let Some(used_count) = used_map.get(target_string) {
                    // println!("used_count, {}:{}:{}", used_count, target_string, start_index);
                    if used_count == word_map.get(target_string).unwrap() {
                        // println!("not matched used_count {}:{} => {} {}:{}", *indices[start_index], *key, target_string, used_count, word_map.get(target_string).unwrap());
                        break;
                    }
                }

                *used_map.entry(target_string.clone()).or_insert(0) += 1;
                // println!("push {}, {:?}", target_string, used);
                used_count += 1;
                prev_index = Some(*key);
                if used_count == words.len() {
                    break;
                }
            }
            if used_count == words.len() {
                // println!("matched {}: {:?}", *indices[start_index], used);
                answer.push(*indices[start_index] as i32);
            }
        }
        // println!("{:?}, {:?}", word_map, index_map);
        answer
    }
    fn create_word_map(words: &Vec<String>) -> HashMap::<String, i32> {
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
    fn create_index_map(s: &String, words: &Vec<String>) -> HashMap::<usize, String> {
        let mut index_map = HashMap::<usize, String>::new();
        let string_len = s.len();
        let word_len = words[0].len();
        let mut processed = HashSet::<String>::new();
        for word in words {
            if processed.get(word).is_some() {
                continue;
            }
            let mut i = 0;
            while i < string_len {
                // println!("s_i {}", s_i);
                let end = i + word_len;
                if end > string_len {
                    break;
                }
                let chunk_string: String = String::from(&s[i..end]);

                if &chunk_string == word {
                    index_map.insert(i, word.clone());
                    processed.insert(word.clone());
                }
                i += 1;
            }
        }
        index_map
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

    #[test]
    fn example4() {
        assert_eq!(
            Solution::find_substring(
                String::from("ababababab"),
                vec![
                    String::from("ababa"),
                    String::from("babab"),
                ]
            ),
            vec![0]
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            Solution::find_substring(
                String::from("a"),
                vec![
                    String::from("a"),
                ]
            ),
            vec![0]
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            Solution::find_substring(
                String::from("bcabbcaabbccacacbabccacaababcbb"),
                vec![
                    String::from("c"),
                    String::from("b"),
                    String::from("a"),
                    String::from("c"),
                    String::from("a"),
                    String::from("a"),
                    String::from("a"),
                    String::from("b"),
                    String::from("c"),
                ]
            ),
            vec![6,16,17,18,19,20]
        );
    }
}
