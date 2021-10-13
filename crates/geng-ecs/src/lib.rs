use std::{
    any::{Any, TypeId},
    cell::{Cell, UnsafeCell},
    collections::HashMap,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub type Id = usize;

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
    pub fn remove<T: Component>(&mut self) -> Option<T> {
        unsafe {
            self.components
                .remove(&TypeId::of::<T>())
                .map(|storage| storage.into_inner())
        }
    }
    pub fn query<'a, Q: Query>(&'a mut self) -> EntityQuery<'a, Q> {
        unsafe {
            let borrows = Q::Fetch::borrow_direct(self);
            let item = if borrows.is_some() {
                Some(Q::Fetch::get_direct(self))
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
    unsafe fn get<T: Component>(&self) -> &T {
        self.components.get(&TypeId::of::<T>()).unwrap().get()
    }
    unsafe fn borrow_mut<T: Component>(&self) -> Option<single_component_storage::BorrowMut> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow_mut())
    }
    unsafe fn get_mut<T: Component>(&self) -> &mut T {
        self.components.get(&TypeId::of::<T>()).unwrap().get_mut()
    }
}

pub struct EntityQuery<'a, Q: Query> {
    #[allow(dead_code)]
    borrows: Option<<Q::Fetch as Fetch<'a>>::DirectBorrows>,
    item: Option<<Q::Fetch as Fetch<'a>>::Item>,
}

impl<'a, Q: Query> Debug for EntityQuery<'a, Q>
where
    <Q::Fetch as Fetch<'a>>::Item: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item)
    }
}

impl<'a, Q: Query> Deref for EntityQuery<'a, Q> {
    type Target = Option<<Q::Fetch as Fetch<'a>>::Item>;
    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, Q: Query> DerefMut for EntityQuery<'a, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

pub trait Query {
    type Fetch: for<'a> Fetch<'a>;
}

pub unsafe trait Fetch<'a> {
    type Item;
    type DirectBorrows;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows>;
    unsafe fn get_direct(entity: &'a Entity) -> Self::Item;
}

impl<'a, T: Component> Query for &'a T {
    type Fetch = FetchRead<T>;
}

pub struct FetchRead<T> {
    phantom_data: PhantomData<T>,
}

unsafe impl<'a, T: Component> Fetch<'a> for FetchRead<T> {
    type Item = &'a T;
    type DirectBorrows = single_component_storage::Borrow<'a>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow::<T>()
    }
    unsafe fn get_direct(entity: &'a Entity) -> Self::Item {
        entity.get::<T>()
    }
}

impl<'a, T: Component> Query for &'a mut T {
    type Fetch = FetchWrite<T>;
}

pub struct FetchWrite<T> {
    phantom_data: PhantomData<T>,
}

unsafe impl<'a, T: Component> Fetch<'a> for FetchWrite<T> {
    type Item = &'a mut T;
    type DirectBorrows = single_component_storage::BorrowMut<'a>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow_mut::<T>()
    }
    unsafe fn get_direct(entity: &'a Entity) -> Self::Item {
        entity.get_mut::<T>()
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        impl<$($name: Query),*> Query for ($($name,)*) {
            type Fetch = ($($name::Fetch,)*);
        }

        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        unsafe impl<'a, $($name: Fetch<'a>),*> Fetch<'a> for ($($name,)*) {
            type Item = ($($name::Item,)*);
            type DirectBorrows = ($($name::DirectBorrows,)*);
            unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
                $(let $name = $name::borrow_direct(entity)?;)*
                Some(($($name,)*))
            }
            unsafe fn get_direct(entity: &'a Entity) -> Self::Item {
                ($($name::get_direct(entity),)*)
            }
        }
    };
}

impl_for_tuple!();
impl_for_tuple!(a);
impl_for_tuple!(a, b);
impl_for_tuple!(a, b, c);

#[test]
fn test() {
    let mut entity = Entity::new();
    entity.add(123i32);
    entity.add("Hello, world!");
    assert_eq!(*entity.query::<&i32>(), Some(&123));
    assert_eq!(
        *entity.query::<(&mut i32, &&str)>(),
        Some((&mut 123, &"Hello, world!"))
    );
}

#[test]
fn test_double_borrow() {
    let mut entity = Entity::new();
    entity.add(123i32);
    assert_eq!(*entity.query::<(&i32, &i32)>(), Some((&123, &123)));
}

#[test]
#[should_panic]
fn test_double_mutable_borrow() {
    let mut entity = Entity::new();
    entity.add(123i32);
    assert_eq!(
        *entity.query::<(&mut i32, &mut i32)>(),
        Some((&mut 123, &mut 123))
    );
}
