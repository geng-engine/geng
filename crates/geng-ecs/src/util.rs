macro_rules! impl_tuples {
    ($macro:ident) => {
        $macro!();
        $macro!(a0);
        $macro!(a0, a1);
        $macro!(a0, a1, a2);
        $macro!(a0, a1, a2, a3);
        $macro!(a0, a1, a2, a3, a4);
        $macro!(a0, a1, a2, a3, a4, a5);
        $macro!(a0, a1, a2, a3, a4, a5, a6);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7, a8);
        $macro!(a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
    };
}

pub trait ZipExt<T> {
    type Output;
    fn zip(self, rhs: T) -> Self::Output;
}

pub trait AsRefExt {
    type Output;
    fn as_ref(self) -> Self::Output;
}

pub trait AsMutExt {
    type Output;
    fn as_mut(self) -> Self::Output;
}

macro_rules! impl_zip_for_tuple {
    (($($a:ident),*), ($($b:ident),*)) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        impl<$($a,)* $($b),*> ZipExt<($($b,)*)> for ($($a,)*) {
            type Output = ($(($a, $b),)*);
            fn zip(self, ($($b,)*): ($($b,)*)) -> Self::Output {
                let ($($a,)*) = self;
                ($(($a, $b),)*)
            }
        }
    };
}

impl_zip_for_tuple!((), ());
impl_zip_for_tuple!((a0), (b0));
impl_zip_for_tuple!((a0, a1), (b0, b1));
impl_zip_for_tuple!((a0, a1, a2), (b0, b1, b2));
impl_zip_for_tuple!((a0, a1, a2, a3), (b0, b1, b2, b3));
impl_zip_for_tuple!((a0, a1, a2, a3, a4), (b0, b1, b2, b3, b4));
impl_zip_for_tuple!((a0, a1, a2, a3, a4, a5), (b0, b1, b2, b3, b4, b5));
impl_zip_for_tuple!((a0, a1, a2, a3, a4, a5, a6), (b0, b1, b2, b3, b4, b5, b6));
impl_zip_for_tuple!(
    (a0, a1, a2, a3, a4, a5, a6, a7),
    (b0, b1, b2, b3, b4, b5, b6, b7)
);
impl_zip_for_tuple!(
    (a0, a1, a2, a3, a4, a5, a6, a7, a8),
    (b0, b1, b2, b3, b4, b5, b6, b7, b8)
);
impl_zip_for_tuple!(
    (a0, a1, a2, a3, a4, a5, a6, a7, a8, a9),
    (b0, b1, b2, b3, b4, b5, b6, b7, b8, b9)
);

macro_rules! impl_asref_for_tuple {
    ($($a:ident),*) => {
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        impl<'a, $($a),*> AsRefExt for &'a ($($a,)*) {
            type Output = ($(&'a $a,)*);
            fn as_ref(self) -> Self::Output {
                let ($($a,)*) = self;
                ($($a,)*)
            }
        }
        #[allow(non_camel_case_types)]
        #[allow(unused_variables)]
        impl<'a, $($a),*> AsMutExt for &'a mut ($($a,)*) {
            type Output = ($(&'a mut $a,)*);
            fn as_mut(self) -> Self::Output {
                let ($($a,)*) = self;
                ($($a,)*)
            }
        }
    };
}

impl_tuples!(impl_asref_for_tuple);

pub(crate) use impl_tuples;
