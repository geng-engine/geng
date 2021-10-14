use std::{
    any::{Any, TypeId},
    cell::{Cell, UnsafeCell},
    collections::{HashMap, HashSet},
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[allow(unused_imports)]
use crate as ecs;

pub use geng_ecs_derive::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u32);

pub trait Component: Sized + 'static {}

impl<T: 'static> Component for T {}

macro_rules! impl_tuples {
    ($macro:ident) => {
        $macro!();
        $macro!(a);
        $macro!(a, b);
        $macro!(a, b, c);
        $macro!(a, b, c, d);
        $macro!(a, b, c, d, e);
        $macro!(a, b, c, d, e, f);
        $macro!(a, b, c, d, e, f, g);
        $macro!(a, b, c, d, e, f, g, h);
        $macro!(a, b, c, d, e, f, g, h, i);
        $macro!(a, b, c, d, e, f, g, h, i, j);
    };
}

mod entity;
mod fetch;
mod filter;
mod query;
mod storage;
mod world;

pub use entity::*;
pub use fetch::*;
pub use filter::*;
pub use query::*;
pub use world::*;
