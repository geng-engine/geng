use super::*;

pub trait Uniforms {
    type ProgramInfoCacheKey: 'static;
    type ProgramInfo: 'static;
    fn get_program_info(program: &Program) -> Self::ProgramInfo;
    fn apply_uniforms(&self, program: &Program, info: &Self::ProgramInfo);
}

impl<'a, U: Uniforms> Uniforms for &'a U {
    type ProgramInfoCacheKey = U::ProgramInfoCacheKey;
    type ProgramInfo = U::ProgramInfo;
    fn get_program_info(program: &Program) -> Self::ProgramInfo {
        U::get_program_info(program)
    }
    fn apply_uniforms(&self, program: &Program, info: &Self::ProgramInfo) {
        U::apply_uniforms(self, program, info)
    }
}

macro_rules! impl_for_tuple {
    ($($a:ident),*; $($b:ident),*) => {
        impl<$($a: Uniforms,)*> Uniforms for ($($a,)*) {
            type ProgramInfoCacheKey = ($($a::ProgramInfoCacheKey,)*);
            type ProgramInfo = ($($a::ProgramInfo,)*);
            fn get_program_info(program: &Program) -> Self::ProgramInfo {
                #![allow(clippy::unused_unit, unused_variables)]
                ($($a::get_program_info(program),)*)
            }
            fn apply_uniforms(&self, program: &Program, info: &Self::ProgramInfo) {
                #![allow(unused_parens, unused_variables)]
                let ($($a,)*) = self;
                let ($($b,)*) = info;
                $(
                    $a.apply_uniforms(program, $b);
                )*
            }
        }
    };
}

batbox_tuple_macros::call_for_tuples_2!(impl_for_tuple);

impl<U: Uniforms> Uniforms for Option<U> {
    type ProgramInfoCacheKey = U::ProgramInfoCacheKey;
    type ProgramInfo = U::ProgramInfo;
    fn get_program_info(program: &Program) -> Self::ProgramInfo {
        U::get_program_info(program)
    }
    fn apply_uniforms(&self, program: &Program, info: &Self::ProgramInfo) {
        if let Some(value) = self {
            value.apply_uniforms(program, info);
        }
        // TODO else default???
    }
}
