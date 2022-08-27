use super::*;

impl Context {
    pub fn blend_func(&self, src_factor: Enum, dst_factor: Enum) {
        unsafe {
            gl::BlendFunc(src_factor, dst_factor);
        }
    }

    pub fn blend_func_separate(
        &self,
        src_factor_rgb: Enum,
        dst_factor_rgb: Enum,
        src_factor_alpha: Enum,
        dst_factor_alpha: Enum,
    ) {
        unsafe {
            gl::BlendFuncSeparate(
                src_factor_rgb,
                dst_factor_rgb,
                src_factor_alpha,
                dst_factor_alpha,
            );
        }
    }

    pub fn clear_color(
        &self,
        red: ClampedFloat,
        green: ClampedFloat,
        blue: ClampedFloat,
        alpha: ClampedFloat,
    ) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear_depth(&self, depth: ClampedFloat) {
        unsafe {
            gl::ClearDepth(depth.into());
        }
    }

    pub fn clear_stencil(&self, stencil: Int) {
        unsafe {
            gl::ClearStencil(stencil);
        }
    }

    pub fn color_mask(&self, red: Bool, green: Bool, blue: Bool, alpha: Bool) {
        unsafe {
            gl::ColorMask(red, green, blue, alpha);
        }
    }

    pub fn cull_face(&self, mode: Enum) {
        unsafe {
            gl::CullFace(mode);
        }
    }

    pub fn depth_func(&self, func: Enum) {
        unsafe {
            gl::DepthFunc(func);
        }
    }

    pub fn depth_mask(&self, flag: Bool) {
        unsafe {
            gl::DepthMask(flag);
        }
    }

    pub fn disable(&self, cap: Enum) {
        unsafe {
            gl::Disable(cap);
        }
    }

    pub fn enable(&self, cap: Enum) {
        unsafe {
            gl::Enable(cap);
        }
    }

    pub fn get_error(&self) -> Enum {
        unsafe { gl::GetError() }
    }

    pub fn line_width(&self, width: Float) {
        unsafe {
            gl::LineWidth(width);
        }
    }

    pub fn get_version_string(&self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as _)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }

    pub fn pixel_store(&self, pname: Enum, param: Int) {
        unsafe {
            gl::PixelStorei(pname, param);
        }
    }

    pub fn stencil_func_separate(&self, face: Enum, func: Enum, r#ref: Int, mask: UInt) {
        unsafe {
            gl::StencilFuncSeparate(face, func, r#ref, mask);
        }
    }

    pub fn stencil_mask_separate(&self, face: Enum, mask: UInt) {
        unsafe {
            gl::StencilMaskSeparate(face, mask);
        }
    }

    pub fn stencil_op_separate(&self, face: Enum, fail: Enum, zfail: Enum, pass: Enum) {
        unsafe {
            gl::StencilOpSeparate(face, fail, zfail, pass);
        }
    }
}
