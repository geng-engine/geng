pub mod prelude;

#[doc(no_inline)]
pub use batbox_derive::*;
#[doc(no_inline)]
pub use batbox_macros::*;

use prelude::*;

pub mod approx;
pub mod autosave;
pub mod collection;
pub mod color;
pub mod diff;
pub mod future_ext;
pub mod geom;
pub mod localization;
pub mod logger;
pub mod num;
pub mod program_args;
pub mod result_ext;
pub mod rng;
pub mod timer;
pub mod updater;
pub mod util;
