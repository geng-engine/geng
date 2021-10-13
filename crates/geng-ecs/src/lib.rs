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

mod single_component_storage {
    use super::*;

    pub struct Storage {
        data: UnsafeCell<Box<dyn Any>>,
        borrows: Cell<usize>,
        borrowed_mutably: Cell<bool>,
    }

    impl Storage {
        pub fn new<T: Component>(value: T) -> Self {
            Self {
                data: UnsafeCell::new(Box::new(value)),
                borrows: Cell::new(0),
                borrowed_mutably: Cell::new(false),
            }
        }
        pub unsafe fn into_inner<T: Component>(self) -> T {
            assert_eq!(self.borrows.get(), 0, "Component is still borrowed");
            assert!(
                !self.borrowed_mutably.get(),
                "Component is still mutably borrowed"
            );
            *self.data.into_inner().downcast().unwrap()
        }
        pub unsafe fn borrow(&self) -> Borrow {
            if self.borrowed_mutably.get() {
                panic!("Failed to borrow, already mutably borrowed");
            }
            self.borrows.set(self.borrows.get() + 1);
            Borrow(self)
        }
        pub unsafe fn get<T: Component>(&self) -> &T {
            (*self.data.get()).downcast_ref().unwrap()
        }
        unsafe fn release(&self) {
            self.borrows.set(self.borrows.get() - 1);
        }
        pub unsafe fn borrow_mut(&self) -> BorrowMut {
            if self.borrows.get() != 0 {
                panic!("Failed to mutably borrow, already borrowed");
            }
            if self.borrowed_mutably.get() {
                panic!("Failed to mutably borrow, already mutably borrowed");
            }
            self.borrowed_mutably.set(true);
            BorrowMut(self)
        }
        pub unsafe fn get_mut<T: Component>(&self) -> &mut T {
            (*self.data.get()).downcast_mut().unwrap()
        }
        unsafe fn release_mut(&self) {
            self.borrowed_mutably.set(false);
        }
    }

    pub struct Borrow<'a>(&'a Storage);

    impl<'a> Drop for Borrow<'a> {
        fn drop(&mut self) {
            unsafe {
                self.0.release();
            }
        }
    }

    pub struct BorrowMut<'a>(&'a Storage);

    impl<'a> Drop for BorrowMut<'a> {
        fn drop(&mut self) {
            unsafe {
                self.0.release_mut();
            }
        }
    }
}

pub struct Entity {
    components: HashMap<TypeId, single_component_storage::Storage>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    pub fn add<T: Component>(&mut self, component: T) {
        self.components.insert(
            TypeId::of::<T>(),
            single_component_storage::Storage::new(component),
        );
    }
    pub fn has<T: Component>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }
    pub fn remove<T: Component>(&mut self) -> Option<T> {
        unsafe {
            self.components
                .remove(&TypeId::of::<T>())
                .map(|storage| storage.into_inner())
        }
    }
    pub fn query<'a, Q: Query<'a>>(&'a mut self) -> EntityQuery<'a, Q> {
        unsafe {
            let borrows = Q::borrow_direct(self);
            let item = if borrows.is_some() {
                Some(Q::get_direct(self).unwrap())
            } else {
                None
            };
            EntityQuery { borrows, item }
        }
    }
    unsafe fn borrow<T: Component>(&self) -> Option<single_component_storage::Borrow> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow())
    }
    unsafe fn get<T: Component>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.get())
    }
    unsafe fn borrow_mut<T: Component>(&self) -> Option<single_component_storage::BorrowMut> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow_mut())
    }
    unsafe fn get_mut<T: Component>(&self) -> Option<&mut T> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.get_mut())
    }
}

pub struct EntityQuery<'a, Q: Query<'a>> {
    #[allow(dead_code)]
    borrows: Option<Q::DirectBorrows>,
    item: Option<Q::Output>,
}

impl<'a, Q: Query<'a>> Debug for EntityQuery<'a, Q>
where
    Q::Output: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item)
    }
}

impl<'a, Q: Query<'a>> Deref for EntityQuery<'a, Q> {
    type Target = Option<Q::Output>;
    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, Q: Query<'a>> DerefMut for EntityQuery<'a, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

pub unsafe trait Query<'a>: Sized {
    type Output;
    type DirectBorrows;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows>;
    unsafe fn get_direct(entity: &'a Entity) -> Option<Self::Output>;
}

unsafe impl<'a, T: Component> Query<'a> for &'a T {
    type Output = Self;
    type DirectBorrows = single_component_storage::Borrow<'a>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow::<T>()
    }
    unsafe fn get_direct(entity: &'a Entity) -> Option<Self::Output> {
        entity.get::<T>()
    }
}

unsafe impl<'a, T: Component> Query<'a> for &'a mut T {
    type Output = Self;
    type DirectBorrows = single_component_storage::BorrowMut<'a>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow_mut::<T>()
    }
    unsafe fn get_direct(entity: &'a Entity) -> Option<Self::Output> {
        entity.get_mut::<T>()
    }
}

unsafe impl<'a, Q: Query<'a>> Query<'a> for Option<Q> {
    type Output = Option<Q::Output>;
    type DirectBorrows = Option<Q::DirectBorrows>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(Q::borrow_direct(entity))
    }
    unsafe fn get_direct(entity: &'a Entity) -> Option<Self::Output> {
        Some(Q::get_direct(entity))
    }
}

pub struct With<T>(PhantomData<T>);

unsafe impl<'a, T: Component> Query<'a> for With<T> {
    type Output = ();
    type DirectBorrows = ();
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        if entity.has::<T>() {
            Some(())
        } else {
            None
        }
    }
    unsafe fn get_direct(entity: &'a Entity) -> Option<Self::Output> {
        if entity.has::<T>() {
            Some(())
        } else {
            None
        }
    }
}

pub struct Without<T>(PhantomData<T>);

unsafe impl<'a, T: Component> Query<'a> for Without<T> {
    type Output = ();
    type DirectBorrows = ();
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        if entity.has::<T>() {
            None
        } else {
            Some(())
        }
    }
    unsafe fn get_direct(entity: &'a Entity) -> Option<Self::Output> {
        if entity.has::<T>() {
            None
        } else {
            Some(())
        }
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        unsafe impl<'a, $($name: Query<'a>),*> Query<'a> for ($($name,)*) {
            type Output = ($($name::Output,)*);
            type DirectBorrows = ($($name::DirectBorrows,)*);
            unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
                $(let $name = $name::borrow_direct(entity)?;)*
                Some(($($name,)*))
            }
            unsafe fn get_direct(entity: &'a Entity) -> Option<Self::Output> {
                Some(($($name::get_direct(entity).unwrap(),)*))
            }
        }
    };
}

impl_for_tuple!();
impl_for_tuple!(a);
impl_for_tuple!(a, b);
impl_for_tuple!(a, b, c);
impl_for_tuple!(a, b, c, d);
impl_for_tuple!(a, b, c, d, e);
impl_for_tuple!(a, b, c, d, e, f);
impl_for_tuple!(a, b, c, d, e, f, g);
impl_for_tuple!(a, b, c, d, e, f, g, h);
impl_for_tuple!(a, b, c, d, e, f, g, h, i);
impl_for_tuple!(a, b, c, d, e, f, g, h, i, j);
