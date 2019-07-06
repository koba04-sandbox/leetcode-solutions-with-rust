fn palindromic_substring(s: &String) -> bool {
    let mut r = String::from("");
    let iter = s.chars().rev();
    for c in iter {
        r.push(c);
    }
    *s == r
}

pub fn longest_palindrome(s: String) -> String {
    let mut answer = String::from("");
    let mut current = String::from("");
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut start_index = 0;
    let mut i = 0;
    while start_index < len {
        current.push(chars[i]);
        // TODO: Skip patterns that are already evaluated
        if answer.len() < current.len() && palindromic_substring(&current) {
            // TODO: should not clone
            answer = current.clone();
        }
        i = i + 1;
        if i >= len {
            start_index = start_index + 1;
            i = start_index;
            current = String::from("");
        }
    }
    answer
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(longest_palindrome(String::from("babad")), "bab");
    }
    #[test]
    fn example2() {
        assert_eq!(longest_palindrome(String::from("cbbd")), "bb");
    }
    #[test]
    fn example3() {
        assert_eq!(longest_palindrome(String::from("")), "");
    }
    #[test]
    fn example4() {
        assert_eq!(longest_palindrome(String::from("a")), "a");
    }
    #[test]
    fn example5() {
        assert_eq!(
            longest_palindrome(
                String::from("jrjnbctoqgzimtoklkxcknwmhiztomaofwwzjnhrijwkgmwwuazcowskjhitejnvtblqyepxispasrgvgzqlvrmvhxusiqqzzibcyhpnruhrgbzsmlsuacwptmzxuewnjzmwxbdzqyvsjzxiecsnkdibudtvthzlizralpaowsbakzconeuwwpsqynaxqmgngzpovauxsqgypinywwtmekzhhlzaeatbzryreuttgwfqmmpeywtvpssznkwhzuqewuqtfuflttjcxrhwexvtxjihunpywerkktbvlsyomkxuwrqqmbmzjbfytdddnkasmdyukawrzrnhdmaefzltddipcrhuchvdcoegamlfifzistnplqabtazunlelslicrkuuhosoyduhootlwsbtxautewkvnvlbtixkmxhngidxecehslqjpcdrtlqswmyghmwlttjecvbueswsixoxmymcepbmuwtzanmvujmalyghzkvtoxynyusbpzpolaplsgrunpfgdbbtvtkahqmmlbxzcfznvhxsiytlsxmmtqiudyjlnbkzvtbqdsknsrknsykqzucevgmmcoanilsyyklpbxqosoquolvytefhvozwtwcrmbnyijbammlzrgalrymyfpysbqpjwzirsfknnyseiujadovngogvptphuyzkrwgjqwdhtvgxnmxuheofplizpxijfytfabx")
            ),
            "qosoq"
        );
    }
}
