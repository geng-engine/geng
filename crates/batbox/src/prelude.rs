//! Items intended to always be available
//!
//! ```
//! use batbox::prelude::*;
//! ```

#[doc(no_inline)]
pub use crate::crates::*;

#[doc(no_inline)]
pub use crate::crates::anyhow::{anyhow, Context as _};
#[doc(no_inline)]
pub use crate::crates::async_trait::async_trait;
#[doc(no_inline)]
pub use crate::crates::derivative::Derivative;
#[doc(no_inline)]
pub use crate::crates::derive_more::{Constructor, Deref, DerefMut};
#[doc(no_inline)]
pub use crate::crates::dyn_clone::{clone_box, DynClone};
#[doc(no_inline)]
pub use crate::crates::futures::prelude::*;
#[doc(no_inline)]
pub use crate::crates::itertools::izip;
#[doc(no_inline)]
pub use crate::crates::maplit::{btreemap, btreeset, hashmap, hashset};
#[doc(no_inline)]
pub use crate::crates::pin_utils::pin_mut;
#[doc(no_inline)]
pub use crate::crates::serde::{de::DeserializeOwned, Deserialize, Serialize};
#[doc(no_inline)]
pub use crate::crates::serde_json;
#[cfg(not(target_arch = "wasm32"))]
#[doc(no_inline)]
pub use crate::crates::threadpool::ThreadPool;
#[doc(no_inline)]
pub use ::std::{
    cell::{Cell, Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display},
    hash::Hash,
    io::{BufRead, Read, Write},
    marker::PhantomData,
    mem,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
        RemAssign, Sub, SubAssign,
    },
    os::raw::{c_char, c_double, c_float, c_int, c_long, c_short, c_ulong, c_ushort, c_void},
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[cfg(target_arch = "wasm32")]
pub use crate::crates::wasm_bindgen::{prelude::*, JsCast as _};
#[cfg(target_arch = "wasm32")]
pub use crate::crates::wasm_bindgen_futures::{future_to_promise, JsFuture};

#[doc(no_inline)]
pub use ::batbox_derive::*;
#[doc(no_inline)]
pub use ::batbox_macros::*;

pub use crate::approx::prelude::*;
pub use crate::cmp::prelude::*;
pub use crate::collection::prelude::*;
pub use crate::color::prelude::*;
pub use crate::diff::prelude::*;
pub use crate::file::prelude::*;
pub use crate::geom::prelude::*;
pub use crate::i18n::prelude::*;
pub use crate::logger::prelude::*;
pub use crate::num::prelude::*;
pub use crate::preferences::prelude::*;
pub use crate::program_args::prelude::*;
pub use crate::range::prelude::*;
pub use crate::rng::prelude::*;
pub use crate::time::prelude::*;
pub use crate::util::prelude::*;
