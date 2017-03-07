pub struct PascalsTriangle {
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: i32) -> Self {
        PascalsTriangle { row_count: row_count as usize }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.row_count == 0 {
            return Vec::new();
        }
        let mut rows = vec![vec![1]];
        for row_index in 1..self.row_count {
            let mut row = vec![1];
            for i in 1..(row_index) {
                row.push(rows[row_index - 1][i - 1] + rows[row_index - 1][i]);
            }
            row.push(1);
            rows.push(row);
        }
        rows
    }
}
