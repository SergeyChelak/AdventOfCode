#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Self; 4] {
        use Direction::*;
        [Up, Down, Left, Right]
    }

    pub fn circular_directions() -> Vec<Vec<Direction>> {
        vec![
            vec![Self::Up],
            vec![Self::Up, Self::Right],
            vec![Self::Right],
            vec![Self::Right, Self::Down],
            vec![Self::Down],
            vec![Self::Down, Self::Left],
            vec![Self::Left],
            vec![Self::Left, Self::Up],
        ]
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Down | Self::Up)
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }

    pub fn is_reversed(&self, other: &Self) -> bool {
        self.reverse() == *other
    }
}
