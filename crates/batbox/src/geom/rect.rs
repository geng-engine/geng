use super::*;

/// A rect with sides parralel to x and y axis.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect<T> {
    pub bottom_left: Vec2<T>,
    pub top_right: Vec2<T>,
}

impl<T: UNum> Rect<T> {
    /// Construct a rect from corner points.
    pub fn from_corners(p1: Vec2<T>, p2: Vec2<T>) -> Self {
        let (min_x, max_x) = min_max(p1.x, p2.x);
        let (min_y, max_y) = min_max(p1.y, p2.y);
        Self {
            bottom_left: vec2(min_x, min_y),
            top_right: vec2(max_x, max_y),
        }
    }

    pub fn pos_size(pos: Vec2<T>, size: Vec2<T>) -> Self {
        Self {
            bottom_left: pos,
            top_right: pos + size,
        }
    }

    pub fn map<U: UNum, F: Fn(T) -> U>(self, f: F) -> Rect<U> {
        Rect::from_corners(self.bottom_left.map(&f), self.top_right.map(&f))
    }

    /// Get rect's width.
    pub fn width(&self) -> T {
        self.top_right.x - self.bottom_left.x
    }

    /// Get rect's height.
    pub fn height(&self) -> T {
        self.top_right.y - self.bottom_left.y
    }

    /// Get rect's size.
    pub fn size(&self) -> Vec2<T> {
        vec2(self.width(), self.height())
    }

    /// Check if a point is inside the rect.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
    /// let rect = Rect::from_corners(vec2(1, 2), vec2(3, 4));
    /// assert!(rect.contains(vec2(2, 3)));
    /// assert!(!rect.contains(vec2(5, 5)));
    /// ```
    pub fn contains(&self, point: Vec2<T>) -> bool {
        self.bottom_left.x <= point.x
            && point.x < self.top_right.x
            && self.bottom_left.y <= point.y
            && point.y < self.top_right.y
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.top_right.x > other.bottom_left.x
            && self.top_right.y > other.bottom_left.y
            && self.bottom_left.x < other.top_right.x
            && self.bottom_left.y < other.top_right.y
    }

    pub fn translate(self, v: Vec2<T>) -> Self {
        Self {
            bottom_left: self.bottom_left + v,
            top_right: self.top_right + v,
        }
    }
}
