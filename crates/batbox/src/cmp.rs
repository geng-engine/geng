//! Extra utilities for [Ord] and [PartialOrd] types
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    #[doc(no_inline)]
    pub use super::*;

    #[doc(no_inline)]
    pub use std::cmp::{max, min};
}

#[allow(unused_imports)]
use prelude::*;

/// Extension trait for getting minimum/maximum of values grouped together
pub trait MinMax: Sized {
    /// Type of a single value
    type T;

    /// Find (min, max)
    fn min_max(self) -> (Self::T, Self::T);

    /// Find min
    fn min(self) -> Self::T {
        self.min_max().0
    }

    /// Find max
    fn max(self) -> Self::T {
        self.min_max().1
    }
}

impl<T: Ord> MinMax for (T, T) {
    type T = T;
    fn min_max(self) -> (T, T) {
        let (a, b) = self;
        if a.cmp(&b) == std::cmp::Ordering::Less {
            (a, b)
        } else {
            (b, a)
        }
    }
}

/// Compares arguments and returns (min, max)
pub fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
    (a, b).min_max()
}

/// Extension trait for getting minimum/maximum of values grouped together
pub trait PartialMinMax: Sized {
    /// Type of a single value
    type T;

    /// Find (min, max)
    fn partial_min_max(self) -> (Self::T, Self::T);

    /// Find min
    fn partial_min(self) -> Self::T {
        self.partial_min_max().0
    }

    /// Find max
    fn partial_max(self) -> Self::T {
        self.partial_min_max().1
    }
}

impl<T: PartialOrd> PartialMinMax for (T, T) {
    type T = T;
    fn partial_min_max(self) -> (T, T) {
        let (a, b) = self;
        if a.partial_cmp(&b) == Some(std::cmp::Ordering::Less) {
            (a, b)
        } else {
            (b, a)
        }
    }
}

/// Compares and returns the minimum of two values
pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    (a, b).partial_min()
}

/// Compares and returns the maximum of two values
pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    (a, b).partial_max()
}

/// Compares arguments and returns (min, max)
pub fn partial_min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    (a, b).partial_min_max()
}

/// Provides methods for clamping values
pub trait Clamp: PartialOrd {
    /// Clamps a value in range.
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// assert_eq!(2.0.clamp_range(0.0..=1.0), 1.0);
    /// assert_eq!(2.0.clamp_range(3.0..), 3.0);
    /// assert_eq!(2.0.clamp_range(..=0.0), 0.0);
    /// ```
    fn clamp_range(mut self, range: impl FixedRangeBounds<Self>) -> Self
    where
        Self: Clone,
    {
        match range.start_bound() {
            FixedBound::Included(start) => self = partial_max(self, start.clone()),
            FixedBound::Unbounded => (),
        }
        match range.end_bound() {
            FixedBound::Included(end) => self = partial_min(self, end.clone()),
            FixedBound::Unbounded => (),
        }
        self
    }

    /// Clamp the absolute value. Same as self.clamp_range(-max..=max)
    fn clamp_abs(self, max: Self) -> Self
    where
        Self: Neg<Output = Self> + Copy,
    {
        self.clamp_range(-max..=max)
    }
}

impl<T: PartialOrd> Clamp for T {}
