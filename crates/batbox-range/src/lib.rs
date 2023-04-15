//! Extra utilities for working with ranges

#[doc(no_inline)]
pub use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// Same as [RangeBounds] but without exclusive bounds
pub trait FixedRangeBounds<T: ?Sized> {
    /// Start index bound
    fn start_bound(&self) -> FixedBound<&T>;
    /// End index bound
    fn end_bound(&self) -> FixedBound<&T>;
}

/// Same as [Bound] but without exclusive bounds
pub enum FixedBound<T> {
    /// An inclusive bound
    Included(T),
    /// An infinite endpoint. Indicates that there is no bound in this direction
    Unbounded,
}

impl<T> FixedRangeBounds<T> for RangeInclusive<T> {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(self.start())
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(self.end())
    }
}

impl<T> FixedRangeBounds<T> for RangeFull {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
}

impl<T> FixedRangeBounds<T> for RangeFrom<T> {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(&self.start)
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
}

impl<T> FixedRangeBounds<T> for RangeToInclusive<T> {
    fn start_bound(&self) -> FixedBound<&T> {
        FixedBound::Unbounded
    }
    fn end_bound(&self) -> FixedBound<&T> {
        FixedBound::Included(&self.end)
    }
}

pub trait IndexRangeExt {
    /// Convert any range into a `start..end` [Range] as if used for slicing of a container of length equal to self
    fn index_range<R>(self, range: R) -> Range<usize>
    where
        R: RangeBounds<usize>;
}

impl IndexRangeExt for usize {
    fn index_range<R>(self, range: R) -> Range<usize>
    where
        R: RangeBounds<usize>,
    {
        Range {
            start: match range.start_bound() {
                Bound::Included(&i) => i,
                Bound::Excluded(&i) => i + 1,
                Bound::Unbounded => 0,
            },
            end: match range.end_bound() {
                Bound::Included(&i) => i + 1,
                Bound::Excluded(&i) => i,
                Bound::Unbounded => self,
            },
        }
    }
}
