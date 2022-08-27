use super::*;

impl Context {
    pub fn blend_func(&self, src_factor: Enum, dst_factor: Enum) {
        self.inner.blend_func(src_factor, dst_factor);
    }

    pub fn blend_func_separate(
        &self,
        src_factor_rgb: Enum,
        dst_factor_rgb: Enum,
        src_factor_alpha: Enum,
        dst_factor_alpha: Enum,
    ) {
        self.inner.blend_func_separate(
            src_factor_rgb,
            dst_factor_rgb,
            src_factor_alpha,
            dst_factor_alpha,
        );
    }

    pub fn clear_color(
        &self,
        red: ClampedFloat,
        green: ClampedFloat,
        blue: ClampedFloat,
        alpha: ClampedFloat,
    ) {
        self.inner.clear_color(red, green, blue, alpha);
    }

    pub fn clear_depth(&self, depth: ClampedFloat) {
        self.inner.clear_depth(depth);
    }

    pub fn clear_stencil(&self, stencil: Int) {
        self.inner.clear_stencil(stencil);
    }

    pub fn color_mask(&self, red: Bool, green: Bool, blue: Bool, alpha: Bool) {
        self.inner.color_mask(red, green, blue, alpha);
    }

    pub fn cull_face(&self, mode: Enum) {
        self.inner.cull_face(mode);
    }

    pub fn depth_func(&self, func: Enum) {
        self.inner.depth_func(func);
    }

    pub fn depth_mask(&self, flag: Bool) {
        self.inner.depth_mask(flag);
    }

    pub fn disable(&self, cap: Enum) {
        self.inner.disable(cap);
    }

    pub fn enable(&self, cap: Enum) {
        self.inner.enable(cap);
    }

    pub fn get_error(&self) -> Enum {
        self.inner.get_error()
    }

    pub fn line_width(&self, width: Float) {
        self.inner.line_width(width);
    }

    pub fn get_version_string(&self) -> String {
        self.inner
            .get_parameter(VERSION)
            .unwrap()
            .as_string()
            .unwrap()
    }

    pub fn pixel_store(&self, pname: Enum, param: Int) {
        self.inner.pixel_storei(pname, param);
    }

    pub fn stencil_func_separate(&self, face: Enum, func: Enum, r#ref: Int, mask: UInt) {
        self.inner.stencil_func_separate(face, func, r#ref, mask);
    }

    pub fn stencil_mask_separate(&self, face: Enum, mask: UInt) {
        self.inner.stencil_mask_separate(face, mask);
    }

    pub fn stencil_op_separate(&self, face: Enum, fail: Enum, zfail: Enum, pass: Enum) {
        self.inner.stencil_op_separate(face, fail, zfail, pass);
    }
}
