use super::*;

/// 2d Axis aligned bounding box.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Aabb2<T> {
    pub x_min: T,
    pub x_max: T,
    pub y_min: T,
    pub y_max: T,
}

impl<T: UNum> Aabb2<T> {
    /// An AABB with both position and size equal to (0, 0).
    pub const ZERO: Self = Aabb2 {
        x_min: T::ZERO,
        x_max: T::ZERO,
        y_min: T::ZERO,
        y_max: T::ZERO,
    };

    /// Construct an AABB from two opposite corners. The two corners can be given in any order.
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// let aabb = AABB::from_corners(vec2(-5.0, -5.0), vec2(5.0, 5.0));
    /// let same = AABB::from_corners(vec2(5.0, -5.0), vec2(-5.0, 5.0));
    /// assert_eq!(aabb, same);
    /// ```
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

    /// Create an AABB at given position of size (0, 0).
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// assert_eq!(AABB::<f32>::ZERO, AABB::point(Vec2::ZERO));
    /// ```
    pub fn point(point: Vec2<T>) -> Self {
        Self {
            x_min: point.x,
            x_max: point.x,
            y_min: point.y,
            y_max: point.y,
        }
    }

    /// Extend boundaries of the AABB by a given value in each direction.
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// let aabb = AABB::point(vec2(5.0, 5.0)).extend_uniform(10.0);
    /// assert_eq!(aabb, AABB::from_corners(vec2(-5.0, -5.0), vec2(15.0, 15.0)));
    /// ```
    pub fn extend_uniform(self, extend: T) -> Self {
        Self {
            x_min: self.x_min - extend,
            x_max: self.x_max + extend,
            y_min: self.y_min - extend,
            y_max: self.y_max + extend,
        }
    }

    /// Extend the boundaries equally right and left and equally up and down
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// let aabb = AABB::ZERO.extend_symmetric(vec2(10.0, 5.0));
    /// let same = AABB::from_corners(vec2(-10.0, -5.0), vec2(10.0, 5.0));
    /// assert_eq!(aabb, same);
    /// ```
    pub fn extend_symmetric(self, extend: Vec2<T>) -> Self {
        Self {
            x_min: self.x_min - extend.x,
            x_max: self.x_max + extend.x,
            y_min: self.y_min - extend.y,
            y_max: self.y_max + extend.y,
        }
    }

    /// Extend the boundaries to the right and up by the given values
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// let aabb = AABB::point(vec2(-10.0, -5.0)).extend_positive(vec2(20.0, 10.0));
    /// let same = AABB::ZERO.extend_symmetric(vec2(10.0, 5.0));
    /// assert_eq!(aabb, same);
    /// ```
    pub fn extend_positive(self, extend: Vec2<T>) -> Self {
        Self {
            x_max: self.x_max + extend.x,
            y_max: self.y_max + extend.y,
            ..self
        }
    }

    /// Extend the left edge of the AABB by a given value.
    pub fn extend_left(self, extend: T) -> Self {
        Self {
            x_min: self.x_min - extend,
            ..self
        }
    }

    /// Extend the right edge of the AABB by a given value.
    pub fn extend_right(self, extend: T) -> Self {
        Self {
            x_max: self.x_max + extend,
            ..self
        }
    }

    /// Extend the top edge of the AABB by a given value.
    pub fn extend_up(self, extend: T) -> Self {
        Self {
            y_max: self.y_max + extend,
            ..self
        }
    }

    /// Extend the bottom edge of the AABB by a given value.
    pub fn extend_down(self, extend: T) -> Self {
        Self {
            y_min: self.y_min - extend,
            ..self
        }
    }

    /// Ensure that the AABB has positive size
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// let original = AABB::point(vec2(10.0, 5.0)).extend_positive(vec2(-20.0, -10.0));
    /// let normalized = AABB::ZERO.extend_symmetric(vec2(10.0, 5.0));
    /// assert_eq!(original.normalized(), normalized);
    /// ```
    pub fn normalized(self) -> Self {
        Self::from_corners(self.bottom_left(), self.top_right())
    }

    /// Get the bottom-left corner of the AABB.
    pub fn bottom_left(&self) -> Vec2<T> {
        vec2(self.x_min, self.y_min)
    }

    /// Get the bottom-right corner of the AABB.
    pub fn bottom_right(&self) -> Vec2<T> {
        vec2(self.x_max, self.y_min)
    }

    /// Get the top-left corner of the AABB.
    pub fn top_left(&self) -> Vec2<T> {
        vec2(self.x_min, self.y_max)
    }

    /// Get the top-right corner of the AABB.
    pub fn top_right(&self) -> Vec2<T> {
        vec2(self.x_max, self.y_max)
    }

    /// Get the center position of the AABB.
    pub fn center(&self) -> Vec2<T> {
        let two: T = T::ONE + T::ONE;
        vec2(
            (self.x_min + self.x_max) / two,
            (self.y_min + self.y_max) / two,
        )
    }

    pub fn corners(&self) -> [Vec2<T>; 4] {
        [
            self.bottom_left(),
            self.bottom_right(),
            self.top_right(),
            self.top_left(),
        ]
    }

    /// Map every value (coordinate) of the AABB.
    pub fn map<U: UNum, F: Fn(T) -> U>(self, f: F) -> Aabb2<U> {
        Aabb2 {
            x_min: f(self.x_min),
            x_max: f(self.x_max),
            y_min: f(self.y_min),
            y_max: f(self.y_max),
        }
    }

    /// Returns the width of the AABB.
    pub fn width(&self) -> T {
        self.x_max - self.x_min
    }

    /// Returns the height of the AABB.
    pub fn height(&self) -> T {
        self.y_max - self.y_min
    }

    /// Return the size of the AABB.
    pub fn size(&self) -> Vec2<T> {
        vec2(self.width(), self.height())
    }

    /// Check if a point is inside the AABB.
    ///
    /// # Examples
    /// ```
    /// use batbox::prelude::*;
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

    /// Checks whether two AABB's intersect.
    pub fn intersects(&self, other: &Self) -> bool {
        self.x_max > other.x_min
            && self.y_max > other.y_min
            && self.x_min < other.x_max
            && self.y_min < other.y_max
    }

    /// Moves the AABB by a given vector.
    pub fn translate(self, v: Vec2<T>) -> Self {
        Self {
            x_min: self.x_min + v.x,
            x_max: self.x_max + v.x,
            y_min: self.y_min + v.y,
            y_max: self.y_max + v.y,
        }
    }

    /// Returns an iterator over points inside the AABB.
    pub fn points(self) -> impl Iterator<Item = Vec2<T>>
    where
        Range<T>: Iterator<Item = T>,
    {
        (self.x_min..self.x_max)
            .flat_map(move |x| (self.y_min..self.y_max).map(move |y| vec2(x, y)))
    }

    /// Returns the smallest possible AABB such that it contains all the points.
    pub fn points_bounding_box(points: impl IntoIterator<Item = Vec2<T>>) -> Self {
        let mut points = points.into_iter();
        let Vec2 {
            x: mut x_min,
            y: mut y_min,
        } = points.next().expect("At least one point expected");
        let mut x_max = x_min;
        let mut y_max = y_min;
        for Vec2 { x, y } in points {
            // TODO: disallow partials?
            x_min = partial_min(x_min, x);
            y_min = partial_min(y_min, y);
            x_max = partial_max(x_max, x);
            y_max = partial_max(y_max, y);
        }
        Aabb2 {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

impl<T: Float> Aabb2<T> {
    /// Returns the distance between two AABB's.
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

impl<T: Float> FitTarget2d<T> for Aabb2<T> {
    fn make_fit(&self, object: &mut impl Transform2d<T>) {
        let current_aabb = object.bounding_box();
        let current_width = current_aabb.width();
        let current_height = current_aabb.height();
        if current_width == T::ZERO || current_height == T::ZERO {
            return;
        }
        let scale = partial_min(self.height() / current_height, self.width() / current_width);
        object.apply_transform(
            Mat3::translate(self.center())
                * Mat3::scale_uniform(scale)
                * Mat3::translate(-current_aabb.center()),
        );
    }
}
