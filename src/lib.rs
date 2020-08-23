pub use ::anyhow;
#[doc(no_inline)]
pub use ::anyhow::{anyhow, Context as _};
#[doc(no_inline)]
pub use bincode;
#[doc(no_inline)]
pub use dyn_clone::{clone_box, DynClone};
#[doc(no_inline)]
pub use futures::{self, prelude::*};
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
#[doc(no_inline)]
pub use structopt::{self, StructOpt};
pub use thiserror;
#[doc(no_inline)]
pub use thiserror::*;
#[doc(no_inline)]
pub use threadpool::ThreadPool;
pub use trans;
#[doc(no_inline)]
pub use trans::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use js_sys;
#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen::{self, prelude::*, JsCast as _};
#[cfg(target_arch = "wasm32")]
pub use web_sys;

pub use batbox_derive::*;

mod approx;
mod autosave;
mod color;
mod diff;
mod future_ext;
mod geom;
mod localization;
pub mod logger;
pub mod microtask;
mod num;
pub mod program_args;
mod rng;
mod timer;

pub use approx::*;
pub use autosave::*;
pub use color::*;
pub use diff::*;
pub use future_ext::ext::*;
pub use geom::*;
pub use localization::*;
pub use num::*;
pub use rng::*;
pub use timer::*;

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

pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    partial_min_max(a, b).0
}

pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    partial_min_max(a, b).1
}

pub fn partial_min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a.partial_cmp(&b).unwrap() == std::cmp::Ordering::Less {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn clamp<T: PartialOrd>(x: T, range: RangeInclusive<T>) -> T {
    let (min, max) = range.into_inner();
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn clamp_abs<T>(x: T, max: T) -> T
where
    T: PartialOrd + Neg<Output = T> + Copy,
{
    clamp(x, -max..=max)
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

pub fn save_file<F: FnOnce(Box<dyn Write>) -> std::io::Result<()>>(
    title: &str,
    default_path: &str,
    f: F,
) -> std::io::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = tinyfiledialogs::save_file_dialog(title, default_path) {
            f(Box::new(std::io::BufWriter::new(std::fs::File::create(
                path,
            )?)))?;
        }
    }
    Ok(())
}

pub fn select_file(title: &str) -> Option<std::path::PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        tinyfiledialogs::open_file_dialog(title, "", None).map(|path| path.into())
    }
    #[cfg(target_arch = "wasm32")]
    {
        panic!("Not supported on wasm");
    }
}
