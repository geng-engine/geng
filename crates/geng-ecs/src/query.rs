use super::*;

pub unsafe trait Query<'a>: Sized {
    type WorldBorrows;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows>;
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self>;
    type DirectBorrows;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows>;
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self;
}

unsafe impl<'a, T: Component> Query<'a> for &'a T {
    type WorldBorrows = component_storage::Borrow<'a, T>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow::<T>()
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self> {
        borrows.get(id)
    }
    type DirectBorrows = single_component_storage::Borrow<'a, T>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow::<T>()
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self {
        borrows.get()
    }
}

unsafe impl<'a, T: Component> Query<'a> for &'a mut T {
    type WorldBorrows = component_storage::BorrowMut<'a, T>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow_mut::<T>()
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self> {
        borrows.get(id)
    }
    type DirectBorrows = single_component_storage::BorrowMut<'a, T>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow_mut::<T>()
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self {
        borrows.get()
    }
}

unsafe impl<'a, Q: Query<'a>> Query<'a> for Option<Q> {
    type WorldBorrows = Option<Q::WorldBorrows>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        Some(Q::borrow_world(world))
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self> {
        borrows.as_ref().map(|borrows| Q::get_world(borrows, id))
    }
    type DirectBorrows = Option<Q::DirectBorrows>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(Q::borrow_direct(entity))
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self {
        borrows.as_ref().map(|borrows| Q::get(borrows))
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        unsafe impl<'a, $($name: Query<'a>),*> Query<'a> for ($($name,)*) {
            type WorldBorrows = ($($name::WorldBorrows,)*);
            unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
                $(let $name = $name::borrow_world(world)?;)*
                Some(($($name,)*))
            }
            unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<Self> {
                let ($($name,)*) = borrows;
                $(let $name = $name::get_world($name, id)?;)*
                Some(($($name,)*))
            }
            type DirectBorrows = ($($name::DirectBorrows,)*);
            unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
                $(let $name = $name::borrow_direct(entity)?;)*
                Some(($($name,)*))
            }
            unsafe fn get(borrows: &Self::DirectBorrows) -> Self {
                let ($($name,)*) = borrows;
                ($($name::get($name),)*)
            }
        }
    };
}

impl_tuples!(impl_for_tuple);
