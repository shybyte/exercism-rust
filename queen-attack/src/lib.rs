pub struct ChessPosition(i8, i8);

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<Self, &'static str> {
        if 0 <= x && x < 8 && 0 <= y && y < 8 {
            Ok(ChessPosition(x, y))
        } else {
            Err("Invalid Position")
        }
    }
}

pub struct Queen {
    pos: ChessPosition,
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen { pos: pos }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.0 == other.pos.0 || // horizontal
        self.pos.1 == other.pos.1 || // vertical
        (self.pos.0 - other.pos.0).abs() == (self.pos.1 - other.pos.1).abs() // diagonal
    }
}
