use super::*;

pub trait Fetch<'a>: Sized + 'a {
    type Output;
    type WorldBorrows;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows>;
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<Self::Output>;
    type DirectBorrows;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows>;
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> Self::Output;
}

pub struct FetchRead<T>(PhantomData<T>);

impl<T> Default for FetchRead<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: Component> Fetch<'a> for FetchRead<T> {
    type Output = &'a T;
    type WorldBorrows = storage::world::Borrow<'a, T>;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow::<T>()
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<&'a T> {
        borrows.get(id)
    }
    type DirectBorrows = storage::entity::Borrow<'a, T>;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow::<T>()
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> &'a T {
        &**borrows
    }
}

pub struct FetchWrite<T>(PhantomData<T>);

impl<T> Default for FetchWrite<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: Component> Fetch<'a> for FetchWrite<T> {
    type Output = &'a mut T;
    type WorldBorrows = storage::world::BorrowMut<'a, T>;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        world.borrow_mut::<T>()
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<&'a mut T> {
        borrows.get_mut(id)
    }
    type DirectBorrows = storage::entity::BorrowMut<'a, T>;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        entity.borrow_mut::<T>()
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> &'a mut T {
        &mut **borrows
    }
}

#[derive(Default)]
pub struct OptionFetch<T>(T);

impl<'a, F: Fetch<'a>> Fetch<'a> for OptionFetch<F> {
    type Output = Option<F::Output>;
    type WorldBorrows = Option<F::WorldBorrows>;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        Some(F::borrow_world(&self.0, world))
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<Self::Output> {
        borrows
            .as_mut()
            .map(|borrows| F::get_world(&self.0, borrows, id))
    }
    type DirectBorrows = Option<F::DirectBorrows>;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(F::borrow_direct(&self.0, entity))
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> Self::Output {
        borrows.as_mut().map(|borrows| F::get(&self.0, borrows))
    }
}

pub struct FilterFetch<F: Filter>(pub F::Fetch);

impl<F: Filter + Default> Default for FilterFetch<F> {
    fn default() -> Self {
        Self(F::default().fetch())
    }
}

impl<'a, F: Filter + 'a> Fetch<'a> for FilterFetch<F> {
    type Output = bool;
    type WorldBorrows = <F::Fetch as Fetch<'a>>::WorldBorrows;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        self.0.borrow_world(world)
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<bool> {
        Some(F::get_world(&self.0, borrows, id))
    }
    type DirectBorrows = <F::Fetch as Fetch<'a>>::DirectBorrows;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        self.0.borrow_direct(entity)
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> bool {
        F::get(&self.0, borrows)
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
         impl<'a, $($name: Fetch<'a>),*> Fetch<'a> for ($($name,)*) {
            type Output = ($($name::Output,)*);
            type WorldBorrows = ($($name::WorldBorrows,)*);
            fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
                let ($($name,)*) = self;
                $(let $name = $name::borrow_world($name, world)?;)*
                Some(($($name,)*))
            }
            fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<Self::Output> {
                let ($($name,)*) = ZipExt::zip(self.as_ref(), borrows.as_mut());
                $(
                    let (fetch, borrows) = $name;
                    let $name = $name::get_world(fetch, borrows, id)?;
                )*
                Some(($($name,)*))
            }
            type DirectBorrows = ($($name::DirectBorrows,)*);
            fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
                let ($($name,)*) = self;
                $(let $name = $name::borrow_direct($name, entity)?;)*
                Some(($($name,)*))
            }
            fn get(&self, borrows: &'a mut Self::DirectBorrows) -> Self::Output {
                let ($($name,)*) = ZipExt::zip(self.as_ref(), borrows.as_mut());
                ($({
                    let (fetch, borrows) = $name;
                    $name::get(fetch, borrows)
                },)*)
            }
        }
    };
}

impl_tuples!(impl_for_tuple);
