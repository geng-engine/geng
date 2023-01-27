//! Geometry & linear algebra
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    #[doc(no_inline)]
    pub use crate::geom;
    #[doc(no_inline)]
    pub use crate::geom::*;
}

mod aabb;
mod chain;
mod curve;
mod ellipse;
mod mat;
mod quad;
mod quat;
mod segment;
mod transform;
mod vec;

pub use aabb::*;
pub use chain::*;
pub use curve::*;
pub use ellipse::*;
pub use mat::*;
pub use quad::*;
pub use quat::*;
pub use segment::*;
pub use transform::*;
pub use vec::*;
