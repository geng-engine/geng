//! Provides macros to work with tuples
#![warn(missing_docs)]

/// calls a macro provided as argument for tuples of all sizes
///
/// # Example
/// ```
/// trait Foo {}
/// macro_rules! impl_for_tuple {
///     ($($a:ident),*) => {
///         impl<$($a),*> Foo for ($($a,)*) {}
///     }
/// }
/// batbox_tuple_macros::call_for_tuples!(impl_for_tuple);
/// ```
#[macro_export]
macro_rules! call_for_tuples {
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
