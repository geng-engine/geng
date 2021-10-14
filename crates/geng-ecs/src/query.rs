use super::*;

pub trait Query {
    type Fetch: for<'a> Fetch<'a> + Default;
}

pub type QueryOutput<'a, Q> = <<Q as Query>::Fetch as Fetch<'a>>::Output;

impl<'a, T: Component> Query for &'a T {
    type Fetch = FetchRead<T>;
}

impl<'a, T: Component> Query for &'a mut T {
    type Fetch = FetchWrite<T>;
}

impl<Q: Query> Query for Option<Q> {
    type Fetch = OptionFetch<Q::Fetch>;
}

impl<F: Filter + Default> Query for Is<F> {
    type Fetch = FilterFetch<F>;
}

macro_rules! impl_for_tuple {
    ($($name:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        impl<$($name: Query),*> Query for ($($name,)*) {
            type Fetch = ($($name::Fetch,)*);
        }
    };
}

impl_tuples!(impl_for_tuple);
