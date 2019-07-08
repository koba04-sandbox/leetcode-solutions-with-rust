pub fn convert(s: String, num_rows: i32) -> String {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            convert(String::from("PAYPALISHIRING"), 3),
            String::from("PAHNAPLSIIGYIR")
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            convert(String::from("PAYPALISHIRING"), 4),
            String::from("PINALSIGYAHRPI")
        );
    }

}
