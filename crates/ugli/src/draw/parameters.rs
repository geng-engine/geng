use super::*;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Condition {
    Never = raw::NEVER as _,
    Less = raw::LESS as _,
    Equal = raw::EQUAL as _,
    LessOrEqual = raw::LEQUAL as _,
    Greater = raw::GREATER as _,
    NotEqual = raw::NOTEQUAL as _,
    GreaterOrEqual = raw::GEQUAL as _,
    Always = raw::ALWAYS as _,
}

type StencilValue = u8;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StencilTest {
    pub condition: Condition,
    pub reference: StencilValue,
    pub mask: StencilValue,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StencilOpFunc {
    /// Keeps the current value.
    Keep = raw::KEEP as _,
    /// Sets the stencil buffer value to 0.
    Zero = raw::ZERO as _,
    /// Sets the stencil buffer value to the reference value as specified by [StencilTest].
    Replace = raw::REPLACE as _,
    /// Increments the current stencil buffer value. Clamps to the maximum representable unsigned value.
    Increment = raw::INCR as _,
    /// Increments the current stencil buffer value. Wraps stencil buffer value to zero when incrementing the maximum representable unsigned value.
    IncrementWrap = raw::INCR_WRAP as _,
    /// Decrements the current stencil buffer value. Clamps to 0.
    Decrement = raw::DECR as _,
    /// Decrements the current stencil buffer value. Wraps stencil buffer value to the maximum representable unsigned value when decrementing a stencil buffer value of 0.
    DecrementWrap = raw::DECR_WRAP as _,
    /// Inverts the current stencil buffer value bitwise.
    Invert = raw::INVERT as _,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StencilOp {
    pub fail: StencilOpFunc,
    pub zfail: StencilOpFunc,
    pub pass: StencilOpFunc,
}

impl StencilOp {
    pub fn always(func: StencilOpFunc) -> Self {
        Self {
            fail: func,
            zfail: func,
            pass: func,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FaceStencilMode {
    pub test: StencilTest,
    pub op: StencilOp,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StencilMode {
    pub back_face: FaceStencilMode,
    pub front_face: FaceStencilMode,
}

impl StencilMode {
    pub fn always(mode: FaceStencilMode) -> Self {
        Self {
            back_face: mode.clone(),
            front_face: mode.clone(),
        }
    }
    pub(crate) fn apply(mode: Option<&Self>, gl: &raw::Context) {
        if let Some(mode) = mode {
            gl.enable(raw::STENCIL_TEST);
            for (face, mode) in [(raw::BACK, &mode.back_face), (raw::FRONT, &mode.front_face)] {
                gl.stencil_func_separate(
                    face,
                    mode.test.condition as _,
                    mode.test.reference as _,
                    mode.test.mask as _,
                );
                gl.stencil_op_separate(
                    face,
                    mode.op.fail as _,
                    mode.op.zfail as _,
                    mode.op.pass as _,
                );
            }
        } else {
            gl.disable(raw::STENCIL_TEST);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DrawParameters {
    pub depth_func: Option<DepthFunc>,
    pub blend_mode: Option<BlendMode>,
    pub stencil_mode: Option<StencilMode>,
    pub cull_face: Option<CullFace>,
    pub viewport: Option<AABB<usize>>,
    pub write_color: bool,
    pub write_depth: bool,
    pub reset_uniforms: bool,
}

impl Default for DrawParameters {
    fn default() -> Self {
        Self {
            depth_func: None,
            blend_mode: None,
            stencil_mode: None,
            cull_face: None,
            viewport: None,
            reset_uniforms: true,
            write_color: true,
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
        StencilMode::apply(self.stencil_mode.as_ref(), gl);
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
        gl.color_mask(
            self.write_color as _,
            self.write_color as _,
            self.write_color as _,
            self.write_color as _,
        );
        gl.depth_mask(gl_bool(self.write_depth));
    }
}
