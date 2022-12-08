use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, AddAssign},
};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, PartialOrd)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Point> for Point {
    type Output = i64;

    fn mul(self, rhs: Point) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Point {
    const ORIGIN: Point = Point { x: 0, y: 0, z: 0 };
    const I: Point = Point { x: 1, y: 0, z: 0 };
    const J: Point = Point { x: 0, y: 1, z: 0 };
    const K: Point = Point { x: 0, y: 0, z: 1 };

    pub fn origin() -> &'static Self {
        &Self::ORIGIN
    }

    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl Direction {
    const PLANE: [Direction; 4] = [
        Direction::PosX,
        Direction::NegX,
        Direction::PosY,
        Direction::NegY,
    ];

    pub fn to_point(&self) -> Point {
        match self {
            Direction::PosX => Point::I,
            Direction::NegX => -Point::I,
            Direction::PosY => Point::J,
            Direction::NegY => -Point::J,
            Direction::PosZ => Point::K,
            Direction::NegZ => -Point::K,
        }
    }

    pub fn plane() -> &'static [Direction; 4] {
        &Self::PLANE
    }
}
