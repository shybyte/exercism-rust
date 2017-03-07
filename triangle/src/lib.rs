// a,b,c are sorted
pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Result<Self, String> {
        let mut sides = sides.to_vec();
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            Ok(Triangle {
                a: sides[0],
                b: sides[1],
                c: sides[2],
            })
        } else {
            Err("Triangle inequality theorem  violated!".to_string())
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.c
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && (self.a == self.b || self.b == self.c)
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }
}
