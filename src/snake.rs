use bevy::prelude::*;

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
pub const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub(crate) fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
pub struct SnakeSegments(Vec<Entity>);
