use crate::direction::Direction;
use crate::vector2d::Vector2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coords2D(pub usize, pub usize);

impl Coords2D {
    pub const ZERO: Self = Self(0, 0);

    fn try_move(&self, amount: Vector2D) -> Option<Self> {
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

    fn try_move_one(&self, direction: Direction) -> Option<Self> {
        self.try_move(Vector2D::from(direction))
    }
}
