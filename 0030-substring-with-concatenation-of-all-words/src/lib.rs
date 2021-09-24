use std::collections::{HashMap,HashSet};

pub struct Solution {
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_map = Solution::create_hash_map(&words);
        let len = words[0].len();
        let mut index_map = HashMap::<usize, String>::new();
        let s_len = s.len();
        let mut processed = HashSet::<String>::new();
        for word in &words {
            if processed.get(word).is_some() {
                continue;
            }
            let mut s_i = 0;
            while s_i < s_len {
                // println!("s_i {}", s_i);
                let end = s_i + len;
                if end > s.len() {
                    break;
                }
                let ss: String = String::from(&s[s_i..end]);

                // for (index, __) in ss.match_indices(word) {
                if &ss == word {
                    index_map.insert(s_i, word.clone());
                    processed.insert(word.clone());
                }
                s_i += 1;
            }
        }
        let mut answer: Vec<i32> = Vec::new();
        let mut indices = index_map.keys().collect::<Vec<&usize>>();
        indices.sort();
        for start_index in 0..indices.len() {
            let mut used_map = HashMap::<String, i32>::new();
            let mut used_count = 0;
            let mut prev_index = None;
            for i in start_index..indices.len() {
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
            } else {
                // println!("not matched length {}, {:?}", *indices[start_index], used);
            }
        }

        // println!("{:?}, {:?}", word_map, index_map);
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
