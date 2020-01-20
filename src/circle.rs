#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: isize,
}

impl Circle {
    pub const fn new(x: isize, y: isize, radius: isize) -> Self {
        Self {
            center: Point { x, y },
            radius,
        }
    }

    pub fn include(self, x: isize, y: isize) -> bool {
        (x - self.center.x).pow(2) + (y - self.center.y).pow(2) <= self.radius.pow(2)
    }
}
