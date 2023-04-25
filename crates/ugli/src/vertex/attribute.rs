use super::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AttributeType {
    Float = raw::FLOAT as _,
}

/// # Safety
/// Don't implement yourself
pub unsafe trait VertexAttributePrimitive {
    const SIZE: usize;
    const ROWS: usize;
    const TYPE: AttributeType;
}

pub trait VertexAttribute {
    type Primitive: VertexAttributePrimitive;
    fn primitive_offset() -> usize;
}

impl<T: VertexAttributePrimitive> VertexAttribute for T {
    type Primitive = Self;
    fn primitive_offset() -> usize {
        0
    }
}

unsafe impl VertexAttributePrimitive for f32 {
    const SIZE: usize = 1;
    const ROWS: usize = 1;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttributePrimitive for [f32; 2] {
    const SIZE: usize = 2;
    const ROWS: usize = 1;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttributePrimitive for [f32; 3] {
    const SIZE: usize = 3;
    const ROWS: usize = 1;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttributePrimitive for [f32; 4] {
    const SIZE: usize = 4;
    const ROWS: usize = 1;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttributePrimitive for [[f32; 3]; 3] {
    const SIZE: usize = 3;
    const ROWS: usize = 3;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttributePrimitive for [[f32; 4]; 4] {
    const SIZE: usize = 4;
    const ROWS: usize = 4;
    const TYPE: AttributeType = AttributeType::Float;
}

mod batbox {
    use super::*;

    impl VertexAttribute for vec2<f32> {
        type Primitive = [f32; 2];
        fn primitive_offset() -> usize {
            0
        }
    }

    impl VertexAttribute for vec3<f32> {
        type Primitive = [f32; 3];
        fn primitive_offset() -> usize {
            0
        }
    }

    impl VertexAttribute for vec4<f32> {
        type Primitive = [f32; 4];
        fn primitive_offset() -> usize {
            0
        }
    }

    impl VertexAttribute for Rgba<f32> {
        type Primitive = [f32; 4];
        fn primitive_offset() -> usize {
            0
        }
    }

    impl VertexAttribute for mat3<f32> {
        type Primitive = [[f32; 3]; 3];
        fn primitive_offset() -> usize {
            0
        }
    }

    impl VertexAttribute for mat4<f32> {
        type Primitive = [[f32; 4]; 4];
        fn primitive_offset() -> usize {
            0
        }
    }
}
