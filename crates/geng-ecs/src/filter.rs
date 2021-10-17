use super::*;

pub trait Filter {
    type Fetch: for<'a> Fetch<'a>;
    fn fetch(self) -> Self::Fetch;
    fn get_world<'a>(
        fetch: &Self::Fetch,
        borrows: &'a mut <Self::Fetch as Fetch<'a>>::WorldBorrows,
        id: Id,
    ) -> bool;
    fn get<'a>(
        fetch: &Self::Fetch,
        borrows: &'a mut <Self::Fetch as Fetch<'a>>::DirectBorrows,
    ) -> bool;
}

pub trait FetchBool {
    type Fetch: for<'a> Fetch<'a, Output = bool>;
    fn fetch(self) -> Self::Fetch;
}

impl<T: FetchBool> Filter for T {
    type Fetch = <T as FetchBool>::Fetch;

    fn fetch(self) -> Self::Fetch {
        <T as FetchBool>::fetch(self)
    }

    fn get_world<'a>(
        fetch: &Self::Fetch,
        borrows: &'a mut <Self::Fetch as Fetch<'a>>::WorldBorrows,
        id: Id,
    ) -> bool {
        fetch.get_world(borrows, id).unwrap()
    }

    fn get<'a>(
        fetch: &Self::Fetch,
        borrows: &'a mut <Self::Fetch as Fetch<'a>>::DirectBorrows,
    ) -> bool {
        fetch.get(borrows)
    }
}

impl<F: for<'a> Fetch<'a, Output = bool>> FetchBool for F {
    type Fetch = Self;
    fn fetch(self) -> Self {
        self
    }
}

#[derive(Default)]
pub struct Is<F>(pub F);

impl<F: Filter> FetchBool for Is<F> {
    type Fetch = FilterFetch<F>;
    fn fetch(self) -> Self::Fetch {
        FilterFetch(self.0.fetch())
    }
}

#[derive(Default)]
pub struct And<A, B>(pub A, pub B);

impl<'a, A: Fetch<'a, Output = bool>, B: Fetch<'a, Output = bool>> Fetch<'a> for And<A, B> {
    type Output = bool;
    type WorldBorrows = (A::WorldBorrows, B::WorldBorrows);
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        Some((self.0.borrow_world(world)?, self.1.borrow_world(world)?))
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<Self::Output> {
        Some(self.0.get_world(&mut borrows.0, id)? && self.1.get_world(&mut borrows.1, id)?)
    }
    type DirectBorrows = (A::DirectBorrows, B::DirectBorrows);
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some((self.0.borrow_direct(entity)?, self.1.borrow_direct(entity)?))
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> Self::Output {
        self.0.get(&mut borrows.0) && self.1.get(&mut borrows.1)
    }
}

impl<A: Filter, B: Filter> std::ops::BitAnd<Is<B>> for Is<A> {
    type Output = Is<And<A, B>>;

    fn bitand(self, rhs: Is<B>) -> Self::Output {
        Is(And(self.0, rhs.0))
    }
}

#[derive(Default)]
pub struct Or<A, B>(pub A, pub B);

impl<'a, A: Fetch<'a, Output = bool>, B: Fetch<'a, Output = bool>> Fetch<'a> for Or<A, B> {
    type Output = bool;
    type WorldBorrows = (A::WorldBorrows, B::WorldBorrows);
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        Some((self.0.borrow_world(world)?, self.1.borrow_world(world)?))
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<Self::Output> {
        Some(self.0.get_world(&mut borrows.0, id)? || self.1.get_world(&mut borrows.1, id)?)
    }
    type DirectBorrows = (A::DirectBorrows, B::DirectBorrows);
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some((self.0.borrow_direct(entity)?, self.1.borrow_direct(entity)?))
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> Self::Output {
        self.0.get(&mut borrows.0) || self.1.get(&mut borrows.1)
    }
}

impl<A: Filter, B: Filter> std::ops::BitOr<Is<B>> for Is<A> {
    type Output = Is<Or<A, B>>;

    fn bitor(self, rhs: Is<B>) -> Self::Output {
        Is(Or(self.0, rhs.0))
    }
}

pub struct With<T>(PhantomData<T>);

pub fn with<T: Component>() -> Is<With<T>> {
    Is(With::default())
}

impl<T> Default for With<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T: Component> Fetch<'a> for With<T> {
    type Output = bool;
    type WorldBorrows = Option<storage::world::Borrow<'a, T>>;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        Some(world.borrow::<T>())
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<bool> {
        if let Some(borrows) = borrows {
            Some(borrows.get(id).is_some())
        } else {
            Some(false)
        }
    }
    type DirectBorrows = &'a Entity;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        Some(entity)
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> bool {
        borrows.has::<T>()
    }
}

#[derive(Default)]
pub struct Not<F>(F);

impl<'a, F: Fetch<'a, Output = bool>> Fetch<'a> for Not<F> {
    type Output = bool;
    type WorldBorrows = F::WorldBorrows;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        self.0.borrow_world(world)
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<bool> {
        Some(!self.0.get_world(borrows, id)?)
    }
    type DirectBorrows = F::DirectBorrows;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        self.0.borrow_direct(entity)
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> bool {
        !self.0.get(borrows)
    }
}

impl<A: Filter> std::ops::Not for Is<A> {
    type Output = Is<Not<A>>;
    fn not(self) -> Self::Output {
        Is(Not(self.0))
    }
}

pub type Without<T> = Not<With<T>>;

pub fn without<T: Component>() -> Is<Without<T>> {
    Is(Without::default())
}

pub struct Equal<T>(pub T);

pub fn equal<T>(value: T) -> Is<Equal<T>> {
    Is(Equal(value))
}

impl<'a, T: Component + PartialEq> Fetch<'a> for Equal<T> {
    type Output = bool;
    type WorldBorrows = <FetchRead<T> as Fetch<'a>>::WorldBorrows;
    fn borrow_world(&self, world: &'a World) -> Option<Self::WorldBorrows> {
        <FetchRead<T> as Fetch<'a>>::borrow_world(&FetchRead::default(), world)
    }
    fn get_world(&self, borrows: &'a mut Self::WorldBorrows, id: Id) -> Option<Self::Output> {
        borrows.get(id).map(|value| *value == self.0)
    }
    type DirectBorrows = <FetchRead<T> as Fetch<'a>>::DirectBorrows;
    fn borrow_direct(&self, entity: &'a Entity) -> Option<Self::DirectBorrows> {
        <FetchRead<T> as Fetch<'a>>::borrow_direct(&FetchRead::default(), entity)
    }
    fn get(&self, borrows: &'a mut Self::DirectBorrows) -> Self::Output {
        **borrows == self.0
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        impl<$($name: Filter),*> Filter for ($($name,)*) {
            type Fetch = ($($name::Fetch,)*);
            fn fetch(self) -> Self::Fetch {
                let ($($name,)*) = self;
                ($($name.fetch(),)*)
            }
             fn get_world<'a>(fetch: &Self::Fetch, borrows: &'a mut <Self::Fetch as Fetch<'a>>::WorldBorrows, id: Id) -> bool {
                let ($($name,)*) = ZipExt::zip(fetch.as_ref(), borrows.as_mut());
                $(
                    let (fetch, borrows) = $name;
                    if !<$name as Filter>::get_world(fetch, borrows, id) {
                        return false;
                    }
                )*
                true
            }
             fn get<'a>(fetch: &Self::Fetch, borrows: &'a mut <Self::Fetch as Fetch<'a>>::DirectBorrows) -> bool {
                let ($($name,)*) = ZipExt::zip(fetch.as_ref(), borrows.as_mut());
                $(
                    let (fetch, borrows) = $name;
                    if !<$name as Filter>::get(fetch, borrows) {
                        return false;
                    }
                )*
                true
            }
        }
    };
}

impl_tuples!(impl_for_tuple);
