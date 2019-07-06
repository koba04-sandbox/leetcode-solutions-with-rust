use std::collections::HashSet;

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
    let mut answer = String::from("");
    let mut current = String::from("");
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut start_index = 0;
    let mut i = 0;
    let mut used = HashSet::new();
    while start_index < len {
        current.push(chars[i]);
        if !used.contains(&current) && answer.len() < current.len() && palindromic_substring(&current) {
            // TODO: should not clone
            answer = current.clone();
        }
        i = i + 1;
        // TODO: should not clone
        used.insert(current.clone());
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
