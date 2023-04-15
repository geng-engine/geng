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
