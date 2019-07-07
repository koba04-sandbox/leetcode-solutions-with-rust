fn palindromic_substring(s: &String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut start_index = 0;
    let mut last_index = chars.len() - 1;
    // println!("start: {}", s);
    while start_index != last_index && start_index < last_index {
        // println!("{}, {}", start_index, last_index);
        if chars[start_index] != chars[last_index] {
            return false;
        }
        start_index = start_index + 1;
        last_index = last_index -1;
    }
    true
}

pub fn longest_palindrome(s: String) -> String {
    let mut target = s.clone();
    let mut answer = String::from("");
    while target.len() > 0 && answer.len() < target.len() {
        let mut tmp = target.clone();
        // println!("target:{}", target);
        while tmp.len() > 0 && answer.len() < tmp.len() {
            // println!("tmp:{}", tmp);
            if palindromic_substring(&tmp) {
                answer = tmp.clone();
            }
            tmp.pop().unwrap();
        }
        target.remove(0);
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
    #[test]
    fn example6() {
        assert_eq!(
            longest_palindrome(
                String::from("zudfweormatjycujjirzjpyrmaxurectxrtqedmmgergwdvjmjtstdhcihacqnothgttgqfywcpgnuvwglvfiuxteopoyizgehkwuvvkqxbnufkcbodlhdmbqyghkojrgokpwdhtdrwmvdegwycecrgjvuexlguayzcammupgeskrvpthrmwqaqsdcgycdupykppiyhwzwcplivjnnvwhqkkxildtyjltklcokcrgqnnwzzeuqioyahqpuskkpbxhvzvqyhlegmoviogzwuiqahiouhnecjwysmtarjjdjqdrkljawzasriouuiqkcwwqsxifbndjmyprdozhwaoibpqrthpcjphgsfbeqrqqoqiqqdicvybzxhklehzzapbvcyleljawowluqgxxwlrymzojshlwkmzwpixgfjljkmwdtjeabgyrpbqyyykmoaqdambpkyyvukalbrzoyoufjqeftniddsfqnilxlplselqatdgjziphvrbokofvuerpsvqmzakbyzxtxvyanvjpfyvyiivqusfrsufjanmfibgrkwtiuoykiavpbqeyfsuteuxxjiyxvlvgmehycdvxdorpepmsinvmyzeqeiikajopqedyopirmhymozernxzaueljjrhcsofwyddkpnvcvzixdjknikyhzmstvbducjcoyoeoaqruuewclzqqqxzpgykrkygxnmlsrjudoaejxkipkgmcoqtxhelvsizgdwdyjwuumazxfstoaxeqqxoqezakdqjwpkrbldpcbbxexquqrznavcrprnydufsidakvrpuzgfisdxreldbqfizngtrilnbqboxwmwienlkmmiuifrvytukcqcpeqdwwucymgvyrektsnfijdcdoawbcwkkjkqwzffnuqituihjaklvthulmcjrhqcyzvekzqlxgddjoir")
            ),
            "gykrkyg"
        );
    }
}
