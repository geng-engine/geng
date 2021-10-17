use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Debug,
    marker::PhantomData,
};

#[allow(unused_imports)]
use crate as ecs;

pub use geng_ecs_derive::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u32);

pub trait Component: Sized + 'static {}

impl<T: 'static> Component for T {}

mod entity;
mod fetch;
mod filter;
mod query;
mod storage;
pub mod util;
mod world;

pub use entity::*;
pub use fetch::*;
pub use filter::*;
pub use query::*;
use util::*;
pub use world::*;
