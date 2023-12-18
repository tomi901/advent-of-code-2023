use std::cmp::{max, min};
use std::ops::{Add, Mul, Neg, Sub};
use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Vector2D(pub isize, pub isize);

impl Vector2D {
    pub const ZERO: Self = Self(0, 0);
    
    pub fn min_2d(&self, rhs: Self) -> Self {
        Self(min(self.0, rhs.0), min(self.1, rhs.1))
    }

    pub fn max_2d(&self, rhs: Self) -> Self {
        Self(max(self.0, rhs.0), max(self.1, rhs.1))
    }
}

impl From<Direction> for Vector2D {
    fn from(value: Direction) -> Self {
        Vector2D::from(&value)
    }
}

impl From<&Direction> for Vector2D {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => Self(0, -1),
            Direction::East => Self(1, 0),
            Direction::South => Self(0, 1),
            Direction::West => Self(-1, 0),
        }
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

// Can we make this generic?
impl Mul<usize> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs as isize, self.1 * rhs as isize)
    }
}
