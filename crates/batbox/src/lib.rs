//! Battery box, a library containing common stuff
//!
//! Check out [prelude] which is supposed to be used like `use batbox::prelude::*`
//! A lot of reexports of std and other [crates].

#![warn(missing_docs)]

#[allow(unused_imports)]
use crate as batbox;

pub mod crates {
    //! External crates

    pub use ::anyhow;
    pub use ::async_trait;
    pub use ::bincode;
    pub use ::derivative;
    pub use ::derive_more;
    pub use ::dyn_clone;
    pub use ::futures;
    pub use ::itertools;
    pub use ::log;
    pub use ::maplit;
    pub use ::once_cell;
    pub use ::pin_utils;
    pub use ::serde;
    pub use ::serde_json;
    pub use ::thiserror;
    pub use ::toml;

    pub use ::batbox_approx as approx;
    pub use ::batbox_cli as cli;
    pub use ::batbox_cmp as cmp;
    pub use ::batbox_collection as collection;
    pub use ::batbox_color as color;
    pub use ::batbox_diff as diff;
    pub use ::batbox_file as file;
    pub use ::batbox_file_dialog as file_dialog;
    pub use ::batbox_i18n as i18n;
    pub use ::batbox_la as la;
    pub use ::batbox_lapp as lapp;
    pub use ::batbox_logger as logger;
    pub use ::batbox_num as num;
    pub use ::batbox_preferences as preferences;
    pub use ::batbox_range as range;
    pub use ::batbox_time as time;
    pub use ::batbox_tuple_macros as tuple_macros;
}

#[doc(no_inline)]
pub use crates::*;

pub mod prelude {
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
            Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg,
            Rem, RemAssign, Sub, SubAssign,
        },
        os::raw::{c_char, c_double, c_float, c_int, c_long, c_short, c_ulong, c_ushort, c_void},
        pin::Pin,
        rc::Rc,
        sync::{Arc, Mutex},
    };

    #[doc(no_inline)]
    pub use crate::crates::approx::*;
    #[doc(no_inline)]
    pub use crate::crates::cli::prelude::*;
    #[doc(no_inline)]
    pub use crate::crates::cmp::*;
    #[doc(no_inline)]
    pub use crate::crates::collection::*;
    #[doc(no_inline)]
    pub use crate::crates::color::*;
    #[doc(no_inline)]
    pub use crate::crates::diff::*;
    #[doc(no_inline)]
    pub use crate::crates::la::*;
    #[doc(no_inline)]
    pub use crate::crates::lapp::*;
    #[doc(no_inline)]
    pub use crate::crates::num::*;
    #[doc(no_inline)]
    pub use crate::crates::range::*;
    #[doc(no_inline)]
    pub use crate::crates::time::Timer;
    #[doc(no_inline)]
    pub use crate::crates::tuple_macros::*;

    #[doc(no_inline)]
    pub use crate::rng::*;
    #[doc(no_inline)]
    pub use crate::util::*;
}

use prelude::*;

// Unsorted into own crates
pub mod rng;
pub mod util;
