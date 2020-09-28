use super::*;

/// Axis aligned bounding box.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AABB<T> {
    pub x_min: T,
    pub x_max: T,
    pub y_min: T,
    pub y_max: T,
}

impl<T: UNum> AABB<T> {
    pub fn bottom_left(&self) -> Vec2<T> {
        vec2(self.x_min, self.y_min)
    }
    pub fn bottom_right(&self) -> Vec2<T> {
        vec2(self.x_max, self.y_min)
    }
    pub fn top_left(&self) -> Vec2<T> {
        vec2(self.x_min, self.y_max)
    }
    pub fn top_right(&self) -> Vec2<T> {
        vec2(self.x_max, self.y_max)
    }
    pub fn center(&self) -> Vec2<T> {
        let two: T = T::ONE + T::ONE;
        vec2(
            (self.x_min + self.x_max) / two,
            (self.y_min + self.y_max) / two,
        )
    }
    pub fn from_corners(p1: Vec2<T>, p2: Vec2<T>) -> Self {
        let (x_min, x_max) = partial_min_max(p1.x, p2.x);
        let (y_min, y_max) = partial_min_max(p1.y, p2.y);
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn pos_size(pos: Vec2<T>, size: Vec2<T>) -> Self {
        Self {
            x_min: pos.x,
            y_min: pos.y,
            x_max: pos.x + size.x,
            y_max: pos.y + size.y,
        }
    }

    pub fn map<U: UNum, F: Fn(T) -> U>(self, f: F) -> AABB<U> {
        AABB {
            x_min: f(self.x_min),
            x_max: f(self.x_max),
            y_min: f(self.y_min),
            y_max: f(self.y_max),
        }
    }

    pub fn width(&self) -> T {
        self.x_max - self.x_min
    }

    /// Get rect's height.
    pub fn height(&self) -> T {
        self.y_max - self.y_min
    }

    /// Get rect's size.
    pub fn size(&self) -> Vec2<T> {
        vec2(self.width(), self.height())
    }

    /// Check if a point is inside the rect.
    ///
    /// # Examples
    /// ```
    /// use batbox::*;
    /// let rect = AABB::from_corners(vec2(1, 2), vec2(3, 4));
    /// assert!(rect.contains(vec2(2, 3)));
    /// assert!(!rect.contains(vec2(5, 5)));
    /// ```
    pub fn contains(&self, point: Vec2<T>) -> bool {
        self.x_min <= point.x
            && point.x < self.x_max
            && self.y_min <= point.y
            && point.y < self.y_max
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x_max > other.x_min
            && self.y_max > other.y_min
            && self.x_min < other.x_max
            && self.y_min < other.y_max
    }

    pub fn translate(self, v: Vec2<T>) -> Self {
        Self {
            x_min: self.x_min + v.x,
            x_max: self.x_max + v.x,
            y_min: self.y_min + v.y,
            y_max: self.y_max + v.y,
        }
    }

    pub fn add_padding(self, padding: T) -> Self {
        Self {
            x_min: self.x_min - padding,
            y_min: self.y_min - padding,
            x_max: self.x_max + padding,
            y_max: self.y_max + padding,
        }
    }

    pub fn points(&self) -> impl Iterator<Item = Vec2<T>> + '_
    where
        Range<T>: Iterator<Item = T>,
    {
        (self.x_min..self.x_max)
            .flat_map(move |x| (self.y_min..self.y_max).map(move |y| vec2(x, y)))
    }
}

impl<T: Float> AABB<T> {
    pub fn distance_to(&self, other: &Self) -> T {
        partial_max(
            partial_max(
                partial_max(self.x_min - other.x_max, other.x_min - self.x_max),
                partial_max(self.y_min - other.y_max, other.y_min - self.y_max),
            ),
            T::ZERO,
        )
    }
}
