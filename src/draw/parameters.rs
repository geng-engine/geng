use crate::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DepthFunc {
    Less = raw::LESS as _,
    LessOrEqual = raw::LEQUAL as _,
    Greater = raw::GREATER as _,
}

impl Default for DepthFunc {
    fn default() -> DepthFunc {
        DepthFunc::Less
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BlendMode {
    Alpha,
}

impl Default for BlendMode {
    fn default() -> BlendMode {
        BlendMode::Alpha
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CullFace {
    Back = raw::BACK as _,
    Front = raw::FRONT as _,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DrawParameters {
    pub depth_func: Option<DepthFunc>,
    pub blend_mode: Option<BlendMode>,
    pub cull_face: Option<CullFace>,
    pub viewport: Option<AABB<usize>>,
    pub write_depth: bool,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            depth_func: None,
            blend_mode: None,
            cull_face: None,
            viewport: None,
            write_depth: true,
        }
    }
}

impl DrawParameters {
    pub(crate) fn apply(&self, gl: &raw::Context, framebuffer_size: Vec2<usize>) {
        match self.depth_func {
            Some(depth_test) => gl.depth_func(depth_test as _),
            None => gl.depth_func(raw::ALWAYS),
        }
        match self.blend_mode {
            Some(blend_mode) => {
                gl.enable(raw::BLEND);
                match blend_mode {
                    BlendMode::Alpha => gl.blend_func(raw::SRC_ALPHA, raw::ONE_MINUS_SRC_ALPHA),
                }
            }
            None => gl.disable(raw::BLEND),
        }
        match self.cull_face {
            Some(cull_face) => {
                gl.enable(raw::CULL_FACE);
                gl.cull_face(cull_face as raw::Enum);
            }
            None => gl.disable(raw::CULL_FACE),
        }
        if let Some(rect) = self.viewport {
            gl.viewport(
                rect.x_min as _,
                rect.y_min as _,
                rect.width() as _,
                rect.height() as _,
            );
        } else {
            gl.viewport(0, 0, framebuffer_size.x as _, framebuffer_size.y as _);
        }
        gl.depth_mask(gl_bool(self.write_depth));
    }
}
