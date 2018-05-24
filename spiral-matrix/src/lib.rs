type Position = (isize, isize);
type Direction = usize; // East, South, West, North

static MOVEMENTS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const EMPTY_FIELD: usize = 0;

pub fn next_pos(pos: Position, direction: Direction) -> Position {
    let movement = MOVEMENTS[direction];
    (pos.0 + movement.0, pos.1 + movement.1)
}

pub fn is_in_square(pos: Position, size: usize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < size as isize && pos.1 < size as isize
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = (0..size).map(|_| vec![EMPTY_FIELD; size]).collect();

    let mut pos: Position = (0, 0);
    let mut direction: Direction = 0;

    for counter in 1..=(size * size) {
        matrix[pos.1 as usize][pos.0 as usize] = counter;
        let pos2 = next_pos(pos, direction);
        if is_in_square(pos2, size) && matrix[pos2.1 as usize][pos2.0 as usize] == EMPTY_FIELD {
            pos = pos2;
        } else {
            direction = (direction + 1) % 4;
            pos = next_pos(pos, direction);
        }
    }

    matrix
}
