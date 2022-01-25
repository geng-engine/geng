pub use ::anyhow;
#[doc(no_inline)]
pub use ::anyhow::{anyhow, Context as _};
#[doc(no_inline)]
pub use ::async_trait;
#[doc(no_inline)]
pub use async_trait::*;
#[doc(no_inline)]
pub use bincode;
#[doc(no_inline)]
pub use clap;
pub use derive_more;
#[doc(no_inline)]
pub use derive_more::{Constructor, Deref, DerefMut};
#[doc(no_inline)]
pub use dyn_clone::{clone_box, DynClone};
#[doc(no_inline)]
pub use futures::{self, prelude::*};
pub use itertools;
#[doc(no_inline)]
pub use itertools::izip;
#[doc(no_inline)]
pub use log::{self, debug, error, info, log_enabled, trace, warn};
#[doc(no_inline)]
pub use maplit::*;
#[doc(no_inline)]
pub use once_cell;
#[doc(no_inline)]
pub use pin_utils::pin_mut;
#[doc(no_inline)]
pub use serde::{self, Deserialize, Serialize};
#[doc(no_inline)]
pub use serde_json;
#[doc(no_inline)]
pub use std::{
    cell::{Cell, Ref, RefCell, RefMut},
    cmp::{max, min},
    collections::{HashMap, HashSet},
    convert::{TryFrom, TryInto},
    fmt::{self, Debug, Display},
    io::{BufRead, Read, Write},
    marker::PhantomData,
    mem,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
        RemAssign, Sub, SubAssign,
    },
    ops::{Bound, Range, RangeBounds, RangeInclusive},
    os::raw::{c_char, c_double, c_float, c_int, c_long, c_short, c_ulong, c_ushort, c_void},
    pin::Pin,
    rc::Rc,
    sync::{Arc, Mutex},
};
pub use thiserror;
#[doc(no_inline)]
pub use threadpool::ThreadPool;

#[cfg(target_arch = "wasm32")]
pub use js_sys;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::{self, prelude::*, JsCast as _};
#[cfg(target_arch = "wasm32")]
pub use web_sys;

pub use batbox_derive::*;

mod approx;
mod autosave;
mod collection;
mod color;
mod diff;
mod future_ext;
mod geom;
mod localization;
pub mod logger;
mod num;
pub mod program_args;
mod result_ext;
mod rng;
mod timer;
mod updater;

pub use approx::*;
pub use autosave::*;
pub use collection::*;
pub use color::*;
pub use diff::*;
pub use future_ext::ext::*;
pub use geom::*;
pub use localization::*;
pub use num::*;
pub use program_args::args as program_args;
pub use result_ext::ResultExt as _;
pub use rng::*;
pub use timer::*;
pub use updater::*;

pub fn default<T: Default>() -> T {
    T::default()
}

pub fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
    if a.cmp(&b) == std::cmp::Ordering::Less {
        (a, b)
    } else {
        (b, a)
    }
}

impl<T: PartialOrd> PartialOrdExt for T {}

pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    a.partial_min(b)
}

pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    a.partial_max(b)
}

pub fn partial_min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    a.partial_min_max(b)
}

pub trait PartialOrdExt: PartialOrd {
    fn partial_min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.partial_min_max(other).0
    }

    fn partial_max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.partial_min_max(other).1
    }

    fn partial_min_max(self, other: Self) -> (Self, Self)
    where
        Self: Sized,
    {
        if self.partial_cmp(&other).unwrap() == std::cmp::Ordering::Less {
            (self, other)
        } else {
            (other, self)
        }
    }

    /// Clamps a value in range.
    /// # Panics
    /// Panics if range is exclusive.
    /// # Examples
    /// ```
    /// # use batbox::*;
    /// assert_eq!(2.0.clamp_range(0.0..=1.0), 1.0);
    /// assert_eq!(2.0.clamp_range(3.0..), 3.0);
    /// assert_eq!(2.0.clamp_range(..=0.0), 0.0);
    /// ```
    fn clamp_range(mut self, range: impl RangeBounds<Self>) -> Self
    where
        Self: Clone,
    {
        match range.start_bound().cloned() {
            Bound::Included(start) => self = self.partial_max(start),
            Bound::Excluded(_) => panic!("Clamping with an exclusive range is undefined"),
            Bound::Unbounded => (),
        }
        match range.end_bound().cloned() {
            Bound::Included(end) => self = self.partial_min(end),
            Bound::Excluded(_) => panic!("Clamping with an exclusive range is undefined"),
            Bound::Unbounded => (),
        }
        self
    }

    fn clamp_abs(self, max: Self) -> Self
    where
        Self: Neg<Output = Self> + Copy,
    {
        self.clamp_range(-max..=max)
    }
}

pub fn index_range<R>(len: usize, range: R) -> Range<usize>
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
            Bound::Included(&i) => i - 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => len,
        },
    }
}

pub fn global_threadpool() -> &'static ThreadPool {
    static mut INSTANCE: Option<ThreadPool> = None;
    static mut INIT: std::sync::Once = std::sync::Once::new();
    unsafe {
        INIT.call_once(|| {
            mem::forget(mem::replace(&mut INSTANCE, Some(default())));
        });
        INSTANCE.as_ref().unwrap()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_file<F: FnOnce(&mut (dyn Write + Send)) -> std::io::Result<()>>(
    title: &str,
    default_path: &str,
    f: F,
) -> std::io::Result<()> {
    if let Some(path) = tinyfiledialogs::save_file_dialog(title, default_path) {
        f(&mut std::io::BufWriter::new(std::fs::File::create(path)?))?;
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn select_file(title: &str) -> Option<std::path::PathBuf> {
    tinyfiledialogs::open_file_dialog(title, "", None).map(|path| path.into())
}
