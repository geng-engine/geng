use super::*;

pub unsafe trait Filter<'a> {
    type WorldBorrows;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows>;
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> bool;
    type DirectBorrows;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows>;
}

pub struct With<T>(PhantomData<T>);

unsafe impl<'a, T: Component> Filter<'a> for With<T> {
    type WorldBorrows = component_storage::Borrow<'a, T>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow::<T>()
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> bool {
        borrows.get(id).is_some()
    }
    type DirectBorrows = ();
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        if entity.has::<T>() {
            Some(())
        } else {
            None
        }
    }
}

pub struct Without<T>(PhantomData<T>);

unsafe impl<'a, T: Component> Filter<'a> for Without<T> {
    type WorldBorrows = component_storage::Borrow<'a, T>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow::<T>()
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> bool {
        borrows.get(id).is_none()
    }
    type DirectBorrows = ();
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
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
        unsafe impl<'a, $($name: Filter<'a>),*> Filter<'a> for ($($name,)*) {
            type WorldBorrows = ($($name::WorldBorrows,)*);
            unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
                $(let $name = $name::borrow_world(world)?;)*
                Some(($($name,)*))
            }
            unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> bool {
                let ($($name,)*) = borrows;
                $(
                    if !$name::get_world($name, id) {
                        return false;
                    }
                )*
                true
            }
            type DirectBorrows = ($($name::DirectBorrows,)*);
            unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
                $(let $name = $name::borrow_direct(entity)?;)*
                Some(($($name,)*))
            }
        }
    };
}

impl_tuples!(impl_for_tuple);
