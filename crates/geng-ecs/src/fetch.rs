use super::*;

pub unsafe trait Fetch<'a>: Sized {
    type Output;
    type WorldBorrows;
    unsafe fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows>;
    unsafe fn get_world(&self, borrows: &Self::WorldBorrows, id: Id) -> Option<Self::Output>;
    type DirectBorrows;
    unsafe fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows>;
    unsafe fn get(&self, borrows: &Self::DirectBorrows) -> Self::Output;
}

pub struct FetchRead<T>(PhantomData<T>);

impl<T> Default for FetchRead<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

unsafe impl<'a, T: Component> Fetch<'a> for FetchRead<T> {
    type Output = &'a T;
    type WorldBorrows = storage::world::Borrow<'a, T>;
    unsafe fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow::<T>()
    }
    unsafe fn get_world(&self, borrows: &Self::WorldBorrows, id: Id) -> Option<&'a T> {
        borrows.get(id)
    }
    type DirectBorrows = storage::entity::Borrow<'a, T>;
    unsafe fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow::<T>()
    }
    unsafe fn get(&self, borrows: &Self::DirectBorrows) -> &'a T {
        borrows.get()
    }
}

pub struct FetchWrite<T>(PhantomData<T>);

impl<T> Default for FetchWrite<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

unsafe impl<'a, T: Component> Fetch<'a> for FetchWrite<T> {
    type Output = &'a mut T;
    type WorldBorrows = storage::world::BorrowMut<'a, T>;
    unsafe fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow_mut::<T>()
    }
    unsafe fn get_world(&self, borrows: &Self::WorldBorrows, id: Id) -> Option<&'a mut T> {
        borrows.get(id)
    }
    type DirectBorrows = storage::entity::BorrowMut<'a, T>;
    unsafe fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow_mut::<T>()
    }
    unsafe fn get(&self, borrows: &Self::DirectBorrows) -> &'a mut T {
        borrows.get()
    }
}

#[derive(Default)]
pub struct OptionFetch<T>(T);

unsafe impl<'a, F: Fetch<'a>> Fetch<'a> for OptionFetch<F> {
    type Output = Option<F::Output>;
    type WorldBorrows = Option<F::WorldBorrows>;
    unsafe fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        Some(F::borrow_world(&self.0, world))
    }
    unsafe fn get_world(&self, borrows: &Self::WorldBorrows, id: Id) -> Option<Self::Output> {
        borrows
            .as_ref()
            .map(|borrows| F::get_world(&self.0, borrows, id))
    }
    type DirectBorrows = Option<F::DirectBorrows>;
    unsafe fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(F::borrow_direct(&self.0, entity))
    }
    unsafe fn get(&self, borrows: &Self::DirectBorrows) -> Self::Output {
        borrows.as_ref().map(|borrows| F::get(&self.0, borrows))
    }
}

pub struct FilterFetch<F: Filter>(pub F::Fetch);

impl<F: Filter + Default> Default for FilterFetch<F> {
    fn default() -> Self {
        Self(F::default().fetch())
    }
}

unsafe impl<'a, F: Filter> Fetch<'a> for FilterFetch<F> {
    type Output = bool;
    type WorldBorrows = <F::Fetch as Fetch<'a>>::WorldBorrows;
    unsafe fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        self.0.borrow_world(world)
    }
    unsafe fn get_world(&self, borrows: &Self::WorldBorrows, id: Id) -> Option<Self::Output> {
        Some(F::get_world(&self.0, borrows, id))
    }
    type DirectBorrows = <F::Fetch as Fetch<'a>>::DirectBorrows;
    unsafe fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        self.0.borrow_direct(entity)
    }
    unsafe fn get(&self, borrows: &Self::DirectBorrows) -> Self::Output {
        F::get(&self.0, borrows)
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        #[allow(clippy::unused_unit)]
        unsafe impl<'a, $($name: Fetch<'a>),*> Fetch<'a> for ($($name,)*) {
            type Output = ($($name::Output,)*);
            type WorldBorrows = ($($name::WorldBorrows,)*);
            unsafe fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
                let ($($name,)*) = self;
                $(let $name = $name::borrow_world($name, world)?;)*
                Some(($($name,)*))
            }
            unsafe fn get_world(&self, borrows: &Self::WorldBorrows, id: Id) -> Option<Self::Output> {
                let ($($name,)*) = ZipExt::<<&Self::WorldBorrows as AsRefExt>::Output>::zip(self.as_ref(), borrows.as_ref());
                $(
                    let (fetch, borrows) = $name;
                    let $name = $name::get_world(fetch, borrows, id)?;
                )*
                Some(($($name,)*))
            }
            type DirectBorrows = ($($name::DirectBorrows,)*);
            unsafe fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
                let ($($name,)*) = self;
                $(let $name = $name::borrow_direct($name, entity)?;)*
                Some(($($name,)*))
            }
            unsafe fn get(&self, borrows: &Self::DirectBorrows) -> Self::Output {
                let ($($name,)*) = ZipExt::zip(self.as_ref(), borrows.as_ref());
                ($({
                    let (fetch, borrows) = $name;
                    $name::get(fetch, borrows)
                },)*)
            }
        }
    };
}

impl_tuples!(impl_for_tuple);
