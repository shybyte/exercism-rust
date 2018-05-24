use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let height = input.len();
    if height == 0 {
        return vec![];
    }

    let width = input[0].len();
    if width == 0 {
        return vec![];
    }

    let row_maximals: Vec<u64> = input.iter()
        .map(|row| row.iter().max().unwrap())
        .cloned().collect();
    let row_max_points: HashSet<_> = (0..height).flat_map(|row_i| {
        (0..width).filter_map(|col_i|
            if input[row_i][col_i] == row_maximals[row_i] {
                Some((row_i, col_i))
            } else {
                None
            }
        ).collect::<Vec<_>>()
    }).collect();

    let col_minimals: Vec<u64> = (0..width)
        .map(|col_i| (0..height).map(|row_i| input[row_i][col_i]).min().unwrap())
        .collect();
    let col_min_points: HashSet<_> = (0..height).flat_map(|row_i| {
        (0..width).filter_map(|col_i|
            if input[row_i][col_i] == col_minimals[col_i] {
                Some((row_i, col_i))
            } else {
                None
            }
        ).collect::<Vec<_>>()
    }).collect();

    let mut result: Vec<(usize, usize)> = row_max_points
        .intersection(&col_min_points)
        .cloned()
        .collect();
    result.sort();

    result
}
