use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

const fn directions() -> (Point, Point, Point, Point) {
    (
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: 0 },
    )
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn in_bounds(&self, extents: Point) -> bool {
        (self.x >= 0 && self.x < extents.x) && (self.y >= 0 && self.y < extents.y)
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let (up, right, down, left) = directions();
        vec![*self + up, *self + right, *self + down, *self + left]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
