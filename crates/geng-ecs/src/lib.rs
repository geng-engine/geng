use std::{
    any::{Any, TypeId},
    cell::{Cell, UnsafeCell},
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[allow(unused_imports)]
use crate as ecs;

pub use geng_ecs_derive::*;

pub trait Component: Sized + 'static {}

impl<T: 'static> Component for T {}

mod entity;
mod query;
mod single_component_storage;

pub use entity::*;
pub use query::*;
