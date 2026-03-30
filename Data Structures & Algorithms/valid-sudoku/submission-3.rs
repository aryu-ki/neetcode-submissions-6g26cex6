impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![vec![false; 9]; 9];
        let mut cols = vec![vec![false; 9]; 9];
        let mut squares = vec![vec![false; 9]; 9];
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let val = board[i][j].to_digit(10);
                let val = match val {
                    Some(v) => (v as usize) - 1,
                    None => {continue;}
                };
                if rows[i][val] { return false; }
                rows[i][val] = true;
                if cols[j][val] { return false; }
                cols[j][val] = true;
                if squares[(i / 3) * 3 + (j / 3)][val] { return false; }
                squares[(i / 3) * 3 + (j / 3)][val] = true;
            } 
        }
        true
    }
}
