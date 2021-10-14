use super::*;

pub unsafe trait Fetch<'a>: Sized {
    type Output;
    type WorldBorrows;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows>;
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self::Output>;
    type DirectBorrows;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows>;
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output;
}

impl<'a, T: Component> Query for &'a T {
    type Fetch = FetchRead<T>;
}

pub struct FetchRead<T: Component>(T);

unsafe impl<'a, T: Component> Fetch<'a> for FetchRead<T> {
    type Output = &'a T;
    type WorldBorrows = storage::world::Borrow<'a, T>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow::<T>()
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<&'a T> {
        borrows.get(id)
    }
    type DirectBorrows = storage::entity::Borrow<'a, T>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow::<T>()
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> &'a T {
        borrows.get()
    }
}

impl<'a, T: Component> Query for &'a mut T {
    type Fetch = FetchWrite<T>;
}

pub struct FetchWrite<T: Component>(T);

unsafe impl<'a, T: Component> Fetch<'a> for FetchWrite<T> {
    type Output = &'a mut T;
    type WorldBorrows = storage::world::BorrowMut<'a, T>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow_mut::<T>()
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<&'a mut T> {
        borrows.get(id)
    }
    type DirectBorrows = storage::entity::BorrowMut<'a, T>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow_mut::<T>()
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> &'a mut T {
        borrows.get()
    }
}

unsafe impl<'a, F: Fetch<'a>> Fetch<'a> for Option<F> {
    type Output = Option<F::Output>;
    type WorldBorrows = Option<F::WorldBorrows>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        Some(F::borrow_world(world))
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self::Output> {
        borrows.as_ref().map(|borrows| F::get_world(borrows, id))
    }
    type DirectBorrows = Option<F::DirectBorrows>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(F::borrow_direct(entity))
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output {
        borrows.as_ref().map(|borrows| F::get(borrows))
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        unsafe impl<'a, $($name: Fetch<'a>),*> Fetch<'a> for ($($name,)*) {
            type Output = ($($name::Output,)*);
            type WorldBorrows = ($($name::WorldBorrows,)*);
            unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
                $(let $name = $name::borrow_world(world)?;)*
                Some(($($name,)*))
            }
            unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self::Output> {
                let ($($name,)*) = borrows;
                $(let $name = $name::get_world($name, id)?;)*
                Some(($($name,)*))
            }
            type DirectBorrows = ($($name::DirectBorrows,)*);
            unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
                $(let $name = $name::borrow_direct(entity)?;)*
                Some(($($name,)*))
            }
            unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output {
                let ($($name,)*) = borrows;
                ($($name::get($name),)*)
            }
        }
    };
}

impl_tuples!(impl_for_tuple);
