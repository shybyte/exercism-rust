use std::cmp::{min, max};

pub fn annotate(board_input: &Vec<&str>) -> Vec<String> {
    let board: Vec<Vec<u8>> = board_input.iter()
        .map(|row|
            row.chars()
                .map(|c|
                    match c {
                        '*' => 1,
                        _ => 0
                    })
                .collect()
        ).collect();

    board.iter().enumerate()
        .map(|(row_i, row)|
            row.iter().enumerate().map(|(col_i, &cell)|
                if cell == 1 {
                    '*'
                } else {
                    let sum = sum_square_3(&board, row_i, col_i);
                    if sum > 0 {
                        (sum + b'0') as char
                    } else {
                        ' '
                    }
                }
            ).collect()
        ).collect()
}

pub fn sum_square_3(board: &Vec<Vec<u8>>, center_row: usize, center_col: usize) -> u8 {
    let row_begin = max((center_row as i32) - 1, 0) as usize;
    let row_end = min(center_row + 2, board.len());
    (row_begin..row_end).map(|row| {
        let col_begin = max((center_col as i32) - 1, 0) as usize;
        let col_end = min(center_col + 2, board[row].len());
        let row_sum: u8 = (col_begin..col_end).map(|col| board[row][col]).sum();
        row_sum
    }).sum()
}