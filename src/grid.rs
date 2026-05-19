use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub struct Spiral {
    points: Vec<Coord>,
}

impl Spiral {
    pub fn new(n: usize) -> Spiral {
        let mut points = Vec::with_capacity(n);
        if n == 0 {
            return Spiral { points: vec![] };
        }
        points.push(Coord::new(0, 0));

        let mut x = 0;
        let mut y = 0;
        let mut step_count = 1;
        while points.len() < n {
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                for _ in 0..step_count {
                    if points.len() == n {
                        return Spiral { points };
                    }
                    x += dx;
                    y += dy;
                    points.push(Coord::new(x, y));
                }
                if dx == 0 {
                    step_count += 1;
                }
            }
        }

        Spiral { points }
    }

    pub fn points(&self) -> &[Coord] {
        &self.points
    }
}
