use super::*;

pub trait Filter {
    type Fetch: for<'a> Fetch<'a>;
    unsafe fn get_world<'a>(borrows: &<Self::Fetch as Fetch<'a>>::WorldBorrows, id: Id) -> bool;
    unsafe fn get<'a>(borrows: &<Self::Fetch as Fetch<'a>>::DirectBorrows) -> bool;
}

impl<F: for<'a> Fetch<'a, Output = bool>> Filter for F {
    type Fetch = Self;
    unsafe fn get_world<'a>(borrows: &<Self as Fetch<'a>>::WorldBorrows, id: Id) -> bool {
        F::get_world(borrows, id).unwrap()
    }
    unsafe fn get<'a>(borrows: &<Self as Fetch<'a>>::DirectBorrows) -> bool {
        F::get(borrows)
    }
}

pub struct With<T>(PhantomData<T>);

impl<T: Component> Query for With<T> {
    type Fetch = Self;
}

unsafe impl<'a, T: Component> Fetch<'a> for With<T> {
    type Output = bool;
    type WorldBorrows = Option<storage::world::Borrow<'a, T>>;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        Some(world.borrow::<T>())
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<bool> {
        if let Some(borrows) = borrows {
            Some(borrows.get(id).is_some())
        } else {
            Some(false)
        }
    }
    type DirectBorrows = &'a Entity;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(entity)
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> bool {
        borrows.has::<T>()
    }
}

pub struct Inverse<F>(F);

impl<F: Filter> Query for Inverse<F> {
    type Fetch = Self;
}

unsafe impl<'a, F: Filter> Fetch<'a> for Inverse<F> {
    type Output = bool;
    type WorldBorrows = <F::Fetch as Fetch<'a>>::WorldBorrows;
    unsafe fn borrow_world(world: &'a World) -> Option<Self::WorldBorrows> {
        F::Fetch::borrow_world(world)
    }
    unsafe fn get_world(borrows: &Self::WorldBorrows, id: Id) -> Option<bool> {
        Some(!<F as Filter>::get_world(borrows, id))
    }
    type DirectBorrows = <F::Fetch as Fetch<'a>>::DirectBorrows;
    unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
        F::Fetch::borrow_direct(entity)
    }
    unsafe fn get(borrows: &Self::DirectBorrows) -> bool {
        !<F as Filter>::get(borrows)
    }
}

pub type Without<T> = Inverse<With<T>>;

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        impl<$($name: Filter),*> Filter for ($($name,)*) {
            type Fetch = ($($name::Fetch,)*);
            unsafe fn get_world<'a>(borrows: &<Self::Fetch as Fetch<'a>>::WorldBorrows, id: Id) -> bool {
                let ($($name,)*) = borrows;
                $(
                    if !<$name as Filter>::get_world($name, id) {
                        return false;
                    }
                )*
                true
            }
            unsafe fn get<'a>(borrows: &<Self::Fetch as Fetch<'a>>::DirectBorrows) -> bool {
                let ($($name,)*) = borrows;
                $(
                    if !<$name as Filter>::get($name) {
                        return false;
                    }
                )*
                true
            }
        }
    };
}

impl_tuples!(impl_for_tuple);
