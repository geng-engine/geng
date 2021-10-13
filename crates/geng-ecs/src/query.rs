use super::*;

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
