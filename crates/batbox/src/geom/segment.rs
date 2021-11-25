use super::*;

pub struct Segment<T> {
    pub start: Vec2<T>,
    pub end: Vec2<T>,
}

impl<T> Segment<T> {
    pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self { start, end }
    }
}
