impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = [[false; 9]; 9];
        let mut column = [[false; 9]; 9];
        let mut block = [[false; 9]; 9];

        for i in 0..9 as usize {
            for j in 0..9 as usize {
                let c = board[i][j].to_digit(10);
                let num = match c {
                    Some(n) => (n - 1) as usize,
                    None => continue,
                };

                let block_index = (i / 3) * 3 + j / 3;

                if row[i][num] || column[j][num] || block[block_index][num] {
                    return false;
                } else {
                    row[i][num] = true;
                    column[j][num] = true;
                    block[block_index][num] = true;
                }
            }
        }

        return true;
    }
}