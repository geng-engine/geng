use super::*;

mod attribute;
mod buffer;

pub use attribute::*;
pub use buffer::*;

pub trait VertexAttributeVisitor {
    fn visit<A: VertexAttribute>(&mut self, name: &str, offset: usize);
}

/// # Safety
/// Don't implement yourself, use derive macro
pub unsafe trait Vertex {
    fn walk_attributes(visitor: impl VertexAttributeVisitor);
}

pub trait VertexDataVisitor {
    fn visit<'a, T: Vertex + 'a, D: IntoVertexBufferSlice<'a, T>>(
        &mut self,
        data: D,
        divisor: Option<usize>,
    );
}

pub trait VertexDataSource {
    fn walk_data<C>(&self, visitor: C)
    where
        C: VertexDataVisitor;
}

impl<'a, S: VertexDataSource> VertexDataSource for &'a S {
    fn walk_data<C>(&self, visitor: C)
    where
        C: VertexDataVisitor,
    {
        (*self).walk_data(visitor);
    }
}

impl<'a, T: Vertex + 'a> VertexDataSource for &'a VertexBuffer<T> {
    fn walk_data<C>(&self, mut visitor: C)
    where
        C: VertexDataVisitor,
    {
        visitor.visit(*self, None);
    }
}

impl<'a, T: Vertex + 'a> VertexDataSource for VertexBufferSlice<'a, T> {
    fn walk_data<C>(&self, mut visitor: C)
    where
        C: VertexDataVisitor,
    {
        visitor.visit(self, None);
    }
}

pub struct InstancedVertexDataSource<'a, V: Vertex + 'a, I: Vertex + 'a> {
    vertices: VertexBufferSlice<'a, V>,
    instances: VertexBufferSlice<'a, I>,
}

impl<'a, V, I> VertexDataSource for InstancedVertexDataSource<'a, V, I>
where
    V: Vertex + 'a,
    I: Vertex + 'a,
{
    fn walk_data<C>(&self, mut visitor: C)
    where
        C: VertexDataVisitor,
    {
        visitor.visit(&self.vertices, None);
        visitor.visit(&self.instances, Some(1));
    }
}

pub fn instanced<'a, V, I, VS, IS>(
    vertices: VS,
    instances: IS,
) -> InstancedVertexDataSource<'a, V, I>
where
    V: Vertex + 'a,
    I: Vertex + 'a,
    VS: IntoVertexBufferSlice<'a, V>,
    IS: IntoVertexBufferSlice<'a, I>,
{
    InstancedVertexDataSource {
        vertices: vertices.into_slice(),
        instances: instances.into_slice(),
    }
}
