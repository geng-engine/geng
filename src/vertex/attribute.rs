use crate::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AttributeType {
    Float = ugl::FLOAT as _,
}

pub unsafe trait VertexAttribute {
    const SIZE: usize;
    const TYPE: AttributeType;
}

unsafe impl VertexAttribute for f32 {
    const SIZE: usize = 1;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttribute for [f32; 2] {
    const SIZE: usize = 2;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttribute for Vec2<f32> {
    const SIZE: usize = 2;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttribute for [f32; 3] {
    const SIZE: usize = 3;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttribute for Vec3<f32> {
    const SIZE: usize = 3;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttribute for [f32; 4] {
    const SIZE: usize = 4;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttribute for Vec4<f32> {
    const SIZE: usize = 4;
    const TYPE: AttributeType = AttributeType::Float;
}

unsafe impl VertexAttribute for Color<f32> {
    const SIZE: usize = 4;
    const TYPE: AttributeType = AttributeType::Float;
}
