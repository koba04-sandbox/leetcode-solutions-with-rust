pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        vec![]
    }
    pub fn make_patterns(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in 0..words.len() {
            Solution::recursive(i, &words, &mut result);
        }
        result
    }
    pub fn recursive(target: usize, words: &Vec<String>, result: &mut Vec<String>) {
        let mut s = String::from(&words[target]);
        for i in 0..words.len() {
            if i == target {
                continue;
            }
            s += &words[i];
        }
        result.push(s);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example0() {
        assert_eq!(
            Solution::make_patterns(vec![String::from("foo"), String::from("bar"), String::from("baz")]),
            vec![
                String::from("foobarbaz"),
                String::from("foobazbar"),
                String::from("barfoobaz"),
                String::from("barbazfoo"),
                String::from("bazfoobar"),
                String::from("bazbarfoo")
            ]
        );
    }

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
