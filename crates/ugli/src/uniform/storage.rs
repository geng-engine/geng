use super::*;

pub trait Uniforms {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor;
}

impl Uniforms for () {
    fn walk_uniforms<C>(&self, _: &mut C)
    where
        C: UniformVisitor,
    {
    }
}

#[derive(Copy, Clone)]
pub struct SingleUniform<'a, U: Uniform> {
    name: &'a str,
    value: U,
}

impl<'a, U: Uniform> SingleUniform<'a, U> {
    pub fn new(name: &'a str, value: U) -> Self {
        Self { name, value }
    }
}

impl<'a, U: Uniform> Uniforms for SingleUniform<'a, U> {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        visitor.visit(self.name, &self.value);
    }
}

impl<'a, U: Uniforms> Uniforms for &'a U {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        (*self).walk_uniforms(visitor);
    }
}

impl<A: Uniforms, B: Uniforms> Uniforms for (A, B) {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        self.0.walk_uniforms(visitor);
        self.1.walk_uniforms(visitor);
    }
}

impl<A: Uniforms, B: Uniforms, C: Uniforms> Uniforms for (A, B, C) {
    fn walk_uniforms<V>(&self, visitor: &mut V)
    where
        V: UniformVisitor,
    {
        self.0.walk_uniforms(visitor);
        self.1.walk_uniforms(visitor);
        self.2.walk_uniforms(visitor);
    }
}

impl<A: Uniforms, B: Uniforms, C: Uniforms, D: Uniforms> Uniforms for (A, B, C, D) {
    fn walk_uniforms<V>(&self, visitor: &mut V)
    where
        V: UniformVisitor,
    {
        self.0.walk_uniforms(visitor);
        self.1.walk_uniforms(visitor);
        self.2.walk_uniforms(visitor);
        self.3.walk_uniforms(visitor);
    }
}

impl<'a, U: Uniforms> Uniforms for &'a [U] {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        for uniform in *self {
            uniform.walk_uniforms(visitor);
        }
    }
}

impl<U: Uniforms> Uniforms for Option<U> {
    fn walk_uniforms<C>(&self, visitor: &mut C)
    where
        C: UniformVisitor,
    {
        if let Some(u) = self {
            u.walk_uniforms(visitor);
        }
    }
}
