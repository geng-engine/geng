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

mod component_storage;
mod entity;
mod query;
mod single_component_storage;
mod world;

pub use entity::*;
pub use query::*;
pub use world::*;
