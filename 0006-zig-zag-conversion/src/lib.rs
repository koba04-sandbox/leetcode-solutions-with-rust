pub fn convert(s: String, num_rows: i32) -> String {
    let mut matrix: Vec<Vec<char>> = vec![];
    let size = s.len();
    for _ in 0..size {
        let mut column: Vec<char> = vec![];
        for _ in 0..num_rows {
            column.push(' ');
        }
        matrix.push(column);
    }
    let chars: Vec<char> = s.chars().collect();
    let mut column: i32 = 0;
    let mut row: i32 = 0;
    let mut direction: i32 = 1;
    let max = num_rows - 1;
    // println!("matrix {:?}", matrix);
    for c in chars {
        // println!("column:{}, row:{}, direction:{}", column, row, direction);
        matrix[column as usize][row as usize] = c;
        if row == max && direction == 1 {
            column = column + 1;
            if max > 0 {
                direction = -1;
                row = row + direction;
            }
        } else if direction == - 1 && row == 1 {
            column = column + 1;
            row = 0;
            direction = 1;
        } else {
            row = row + direction;
        }
    }

    let mut answer = String::new();
    for i in 0..num_rows {
        for m in &matrix {
            if m[i as usize] != ' ' {
                answer.push(m[i as usize]);
            }
        }
    }
    // println!("matrix {:?}", matrix);
    answer
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

    #[test]
    fn example3() {
        assert_eq!(
            convert(String::from("AB"), 1),
            String::from("AB")
        );
    }

}
