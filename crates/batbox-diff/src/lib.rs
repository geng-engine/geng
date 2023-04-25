//! Diffing structs

use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub use batbox_diff_derive::*;

/// A diffable type
///
/// [Can be derived](::batbox_derive::Diff)
///
/// For [Copy] types implementation just uses the type itself as delta.
///
/// Most of the trait bounds should not be here, but are because of
/// <https://github.com/rust-lang/rust/issues/20671>
pub trait Diff:
    Debug + Serialize + DeserializeOwned + Sync + Send + Clone + PartialEq + 'static + Unpin
{
    /// Object representing the difference between two states of Self
    type Delta: Debug + Serialize + DeserializeOwned + Sync + Send + Clone + 'static + Unpin;

    /// Calculate the difference between two states
    fn diff(&self, to: &Self) -> Self::Delta;

    /// Update the state using the delta
    ///
    /// ```
    /// # use batbox_diff::*;
    /// let a = 0_i32;
    /// let b = 1_i32;
    /// let delta = Diff::diff(&a, &b);
    ///
    /// let mut a = a;
    /// a.update(&delta);
    /// assert_eq!(a, b);
    /// ```
    fn update(&mut self, delta: &Self::Delta);
}

impl<
        T: Debug + Serialize + DeserializeOwned + Sync + Send + Copy + PartialEq + 'static + Unpin,
    > Diff for T
{
    type Delta = Self;
    fn diff(&self, to: &Self) -> Self {
        *to
    }
    fn update(&mut self, new_value: &Self) {
        *self = *new_value;
    }
}
