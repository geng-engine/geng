use super::*;

pub trait Query {
    type Fetch: for<'a> Fetch<'a>;
}

pub type QueryOutput<'a, Q> = <<Q as Query>::Fetch as Fetch<'a>>::Output;

impl<Q: Query> Query for Option<Q> {
    type Fetch = Option<Q::Fetch>;
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
