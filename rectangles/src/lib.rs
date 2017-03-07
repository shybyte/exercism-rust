pub fn count(diagram_input: &Vec<&str>) -> i32 {
    let height = diagram_input.len();
    if height == 0 {
        return 0;
    }

    let width = diagram_input[0].len();
    if width < 2 {
        return 0;
    }

    let diagram: Vec<Vec<char>> = diagram_input.iter()
        .map(|row| row.chars().collect()).collect();

    let mut counter = 0;
    for x1 in 0..width {
        for y1 in 0..height {
            for x2 in (x1 + 1)..width {
                for y2 in (y1 + 1)..height {
                    if test_rectangle(&diagram, x1, y1, x2, y2) {
                        counter += 1;
                    }
                }
            }
        }
    }

    counter
}

pub fn test_rectangle(diagram: &Vec<Vec<char>>, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    let corners = diagram[y1][x1] == '+' &&
        diagram[y1][x2] == '+' &&
        diagram[y2][x1] == '+' &&
        diagram[y2][x2] == '+';
    let top_edge = is_horizontal_edge(diagram, y1, x1, x2);
    let bottom_edge = is_horizontal_edge(diagram, y2, x1, x2);
    let left_edge = is_vertical_edge(diagram, x1, y1, y2);
    let right_edge = is_vertical_edge(diagram, x2, y1, y2);

    corners && top_edge && bottom_edge  && left_edge && right_edge
}


pub fn is_horizontal_edge(diagram: &Vec<Vec<char>>, y: usize, x1: usize, x2: usize) -> bool {
    diagram[y][x1..x2].iter().all(|&c| c == '+' || c == '-')
}

pub fn is_vertical_edge(diagram: &Vec<Vec<char>>, x: usize, y1: usize, y2: usize) -> bool {
    (y1..y2).map(|y| diagram[y][x]).all(|c| c == '+' || c == '|')
}
