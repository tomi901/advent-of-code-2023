use std::fmt::{Debug, Formatter};
use crate::direction::Direction;
use crate::vector2d::Vector2D;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coords2D(pub usize, pub usize);

impl Debug for Coords2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Coords2D {
    pub const ZERO: Self = Self(0, 0);

    pub fn try_move(&self, amount: Vector2D) -> Option<Self> {
        let result = checked_add(self.0, amount.0)
            .and_then(|x| checked_add(self.1, amount.1)
                .and_then(|y| Some(Self(x, y))));
        // println!("{:?} + {:?} = {:?}", self, amount, result);
        return result;

        fn checked_add(a: usize, b: isize) -> Option<usize> {
            if b >= 0 {
                Some(a + b as usize)
            } else {
                let b_abs = b.abs() as usize;
                a.checked_sub(b_abs)
            }
        }
    }

    pub fn try_move_one(&self, direction: Direction) -> Option<Self> {
        self.try_move(Vector2D::from(direction))
    }
    
    pub fn manhattan_distance_to(&self, other: Coords2D) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
