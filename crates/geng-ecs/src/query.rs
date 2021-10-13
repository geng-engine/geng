use super::*;

pub unsafe trait Query<'a>: Sized {
    type Output;
    type DirectBorrows;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows>;
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output;
}

unsafe impl<'a, T: Component> Query<'a> for &'a T {
    type Output = Self;
    type DirectBorrows = single_component_storage::Borrow<'a, T>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow::<T>()
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output {
        borrows.get()
    }
}

unsafe impl<'a, T: Component> Query<'a> for &'a mut T {
    type Output = Self;
    type DirectBorrows = single_component_storage::BorrowMut<'a, T>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow_mut::<T>()
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output {
        borrows.get()
    }
}

unsafe impl<'a, Q: Query<'a>> Query<'a> for Option<Q> {
    type Output = Option<Q::Output>;
    type DirectBorrows = Option<Q::DirectBorrows>;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(Q::borrow_direct(entity))
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output {
        borrows.as_ref().map(|borrows| Q::get(borrows))
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
    unsafe fn get(_borrows: &()) {}
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
    unsafe fn get(_borrows: &()) {}
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
            unsafe fn get(borrows: &Self::DirectBorrows) -> Self::Output {
                let ($($name,)*) = borrows;
                ($($name::get($name),)*)
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
