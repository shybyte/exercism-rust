#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (isize, isize),
    dir: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            dir: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            dir: match self.dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            dir: match self.dir {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.pos;
        Robot {
            pos: match self.dir {
                Direction::North => (x, y + 1),
                Direction::East => (x + 1, y),
                Direction::South => (x, y - 1),
                Direction::West => (x - 1, y),
            },
            ..self
        }

    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for inst in instructions.chars() {
            robot = match inst {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => panic!("Ivalid instruction!"),
            }
        }
        robot
    }

    pub fn position(&self) -> (isize, isize) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
