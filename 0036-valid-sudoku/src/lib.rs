use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for (i, rows) in board.iter().enumerate() {
            for (j, _val) in rows.iter().enumerate() {
                // check all columns
                if i == 0 {
                    let mut set: HashSet<&char> = HashSet::new();
                    for k in 0..9 {
                        let v = &board[k][j];
                        if *v == '.' {
                            continue;
                        }
                        if set.contains(v) {
                            return false;
                        }
                        set.insert(&v);
                    }
                }

                // check all rows
                if j == 0 {
                    let mut set: HashSet<&char> = HashSet::new();
                    for k in 0..9 {
                        let v = &board[i][k];
                        if *v == '.' {
                            continue;
                        }
                        if set.contains(v) {
                            return false;
                        }
                        set.insert(&v);
                    }
                }

                // check boxes
                if (i == 0 || i == 3 || i == 6) && (j == 0 || j == 3 || j == 6) {
                    // println!("----");
                    let mut set: HashSet<&char> = HashSet::new();
                    for y in i..i + 3 {
                        for x in j..j + 3 {
                            // println!("{}:{}",y, x);
                            let v = &board[y][x];
                            if *v == '.' {
                                continue;
                            }
                            if set.contains(v) {
                                return false;
                            }
                            set.insert(&v);
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let expected = true;
        assert_eq!(Solution::is_valid_sudoku(board), expected)
    }

    #[test]
    fn example2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let expected = false;
        assert_eq!(Solution::is_valid_sudoku(board), expected)
    }

    #[test]
    fn example3() {
        let board = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];
        let expected = false;
        assert_eq!(Solution::is_valid_sudoku(board), expected)
    }

    #[test]
    fn example4() {
        let board = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '9', '.', '.', '8', '.', '.', '.'],
            vec!['.', '.', '.', '4', '.', '6', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '1', '.'],
            vec!['.', '.', '.', '.', '2', '.', '.', '3', '.'],
            vec!['.', '4', '.', '.', '.', '.', '.', '.', '4'],
            vec!['5', '.', '.', '6', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '7', '.', '6', '.', '.'],
        ];
        let expected = false;
        assert_eq!(Solution::is_valid_sudoku(board), expected)
    }
}
