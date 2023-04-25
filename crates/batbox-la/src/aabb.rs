use super::*;

/// 2d Axis aligned bounding box.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Aabb2<T> {
    /// Minimum coordinates
    pub min: vec2<T>,
    /// Maximum coordinates
    pub max: vec2<T>,
}

impl<T: UNum> Aabb2<T> {
    /// An [Aabb2] with both position and size equal to (0, 0).
    pub const ZERO: Self = Aabb2 {
        min: vec2::ZERO,
        max: vec2::ZERO,
    };

    /// Construct an [Aabb2] from two opposite corners. The two corners can be given in any order.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let aabb = Aabb2::from_corners(vec2(-5.0, -5.0), vec2(5.0, 5.0));
    /// let same = Aabb2::from_corners(vec2(5.0, -5.0), vec2(-5.0, 5.0));
    /// assert_eq!(aabb, same);
    /// ```
    pub fn from_corners(p1: vec2<T>, p2: vec2<T>) -> Self {
        let (min_x, max_x) = partial_min_max(p1.x, p2.x);
        let (min_y, max_y) = partial_min_max(p1.y, p2.y);
        Self {
            min: vec2(min_x, min_y),
            max: vec2(max_x, max_y),
        }
    }

    /// Create an [Aabb2] at given position of size (0, 0).
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// assert_eq!(Aabb2::<f32>::ZERO, Aabb2::point(vec2::ZERO));
    /// ```
    pub fn point(point: vec2<T>) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    /// Extend boundaries of the [Aabb2] by a given value in each direction.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let aabb = Aabb2::point(vec2(5, 5)).extend_uniform(10);
    /// assert_eq!(aabb, Aabb2::from_corners(vec2(-5, -5), vec2(15, 15)));
    /// ```
    pub fn extend_uniform(self, extend: T) -> Self {
        Self {
            min: self.min.map(|x| x - extend),
            max: self.max.map(|x| x + extend),
        }
    }

    /// Extend the boundaries equally right and left and equally up and down
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let aabb = Aabb2::ZERO.extend_symmetric(vec2(10, 5));
    /// let same = Aabb2::from_corners(vec2(-10, -5), vec2(10, 5));
    /// assert_eq!(aabb, same);
    /// ```
    pub fn extend_symmetric(self, extend: vec2<T>) -> Self {
        Self {
            min: self.min - extend,
            max: self.max + extend,
        }
    }

    /// Extend the boundaries to the right and up by the given values
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let aabb = Aabb2::point(vec2(-10.0, -5.0)).extend_positive(vec2(20.0, 10.0));
    /// let same = Aabb2::ZERO.extend_symmetric(vec2(10.0, 5.0));
    /// assert_eq!(aabb, same);
    /// ```
    pub fn extend_positive(self, extend: vec2<T>) -> Self {
        Self {
            min: self.min,
            max: self.max + extend,
        }
    }

    /// Extend the left edge of the [Aabb2] by a given value.
    pub fn extend_left(self, extend: T) -> Self {
        let mut res = self;
        res.min.x -= extend;
        res
    }

    /// Extend the right edge of the [Aabb2] by a given value.
    pub fn extend_right(self, extend: T) -> Self {
        let mut res = self;
        res.max.x += extend;
        res
    }

    /// Extend the top edge of the [Aabb2] by a given value.
    pub fn extend_up(self, extend: T) -> Self {
        let mut res = self;
        res.max.y += extend;
        res
    }

    /// Extend the bottom edge of the [Aabb2] by a given value.
    pub fn extend_down(self, extend: T) -> Self {
        let mut res = self;
        res.min.y -= extend;
        res
    }

    /// Ensure that the [Aabb2] has positive size
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let original = Aabb2::point(vec2(10.0, 5.0)).extend_positive(vec2(-20.0, -10.0));
    /// let normalized = Aabb2::ZERO.extend_symmetric(vec2(10.0, 5.0));
    /// assert_eq!(original.normalized(), normalized);
    /// ```
    pub fn normalized(self) -> Self {
        Self::from_corners(self.bottom_left(), self.top_right())
    }

    /// Get the bottom-left corner of the [Aabb2].
    pub fn bottom_left(&self) -> vec2<T> {
        self.min
    }

    /// Get the bottom-right corner of the [Aabb2].
    pub fn bottom_right(&self) -> vec2<T> {
        vec2(self.max.x, self.min.y)
    }

    /// Get the top-left corner of the [Aabb2].
    pub fn top_left(&self) -> vec2<T> {
        vec2(self.min.x, self.max.y)
    }

    /// Get the top-right corner of the [Aabb2].
    pub fn top_right(&self) -> vec2<T> {
        vec2(self.max.x, self.max.y)
    }

    /// Get the center position of the [Aabb2].
    pub fn center(&self) -> vec2<T> {
        let two: T = T::ONE + T::ONE;
        vec2(
            (self.min.x + self.max.x) / two,
            (self.min.y + self.max.y) / two,
        )
    }

    /// Get an array of all four corner points
    pub fn corners(&self) -> [vec2<T>; 4] {
        [
            self.bottom_left(),
            self.bottom_right(),
            self.top_right(),
            self.top_left(),
        ]
    }

    /// Map every value (coordinate) of the [Aabb2].
    pub fn map<U: UNum, F: Fn(T) -> U>(self, f: F) -> Aabb2<U> {
        Aabb2 {
            min: self.min.map(&f),
            max: self.max.map(&f),
        }
    }

    /// Map bounds (min & max vectors)
    pub fn map_bounds<U: UNum, F: Fn(vec2<T>) -> vec2<U>>(self, f: F) -> Aabb2<U> {
        Aabb2 {
            min: f(self.min),
            max: f(self.max),
        }
    }

    /// Returns the width of the [Aabb2].
    pub fn width(&self) -> T {
        self.max.x - self.min.x
    }

    /// Returns the height of the [Aabb2].
    pub fn height(&self) -> T {
        self.max.y - self.min.y
    }

    /// Return the size of the [Aabb2].
    pub fn size(&self) -> vec2<T> {
        vec2(self.width(), self.height())
    }

    /// Check if a point is inside the [Aabb2].
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let rect = Aabb2::from_corners(vec2(1, 2), vec2(3, 4));
    /// assert!(rect.contains(vec2(2, 3)));
    /// assert!(!rect.contains(vec2(5, 5)));
    /// ```
    pub fn contains(&self, point: vec2<T>) -> bool {
        self.min.x <= point.x
            && point.x < self.max.x
            && self.min.y <= point.y
            && point.y < self.max.y
    }

    /// Checks whether two [Aabb2]'s intersect.
    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x > other.min.x
            && self.max.y > other.min.y
            && self.min.x < other.max.x
            && self.min.y < other.max.y
    }

    /// Moves the [Aabb2] by a given vector.
    pub fn translate(self, v: vec2<T>) -> Self {
        Self {
            min: self.min + v,
            max: self.max + v,
        }
    }

    /// Returns an iterator over points inside the [Aabb2].
    pub fn points(self) -> impl Iterator<Item = vec2<T>>
    where
        Range<T>: Iterator<Item = T>,
    {
        (self.min.x..self.max.x)
            .flat_map(move |x| (self.min.y..self.max.y).map(move |y| vec2(x, y)))
    }

    /// Returns the smallest possible [Aabb2] such that it contains all the points.
    ///
    /// # Examples
    /// ```
    /// # use batbox_la::*;
    /// let aabb = Aabb2::points_bounding_box([
    ///     vec2(3, 0),
    ///     vec2(0, -2),
    ///     vec2(1, 4),
    ///     vec2(-1, -1),
    ///     vec2(0, 3),
    /// ]);
    /// assert_eq!(aabb, Aabb2 { min: vec2(-1, -2), max: vec2(3, 4) });
    /// ```
    pub fn points_bounding_box(points: impl IntoIterator<Item = vec2<T>>) -> Self {
        let mut points = points.into_iter();
        let vec2(mut min_x, mut min_y) = points.next().expect("At least one point expected");
        let mut max_x = min_x;
        let mut max_y = min_y;
        for vec2(x, y) in points {
            // TODO: disallow partials?
            min_x = partial_min(min_x, x);
            min_y = partial_min(min_y, y);
            max_x = partial_max(max_x, x);
            max_y = partial_max(max_y, y);
        }
        Aabb2 {
            min: vec2(min_x, min_y),
            max: vec2(max_x, max_y),
        }
    }
}
