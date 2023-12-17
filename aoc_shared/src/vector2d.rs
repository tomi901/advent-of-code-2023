use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Vector2D(pub isize, pub isize);

impl From<Direction> for Vector2D {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self(0, -1),
            Direction::East => Self(1, 0),
            Direction::South => Self(0, 1),
            Direction::West => Self(-1, 0),
        }
    }
}

impl Vector2D {
    pub const ZERO: Self = Self(0, 0);
}
