use super::*;

impl Context {
    pub fn blend_func(&self, sfactor: Enum, dfactor: Enum) {
        self.inner.blend_func(sfactor, dfactor);
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
        use stdweb::unstable::TryInto;
        return js! {
            var gl = @{&self.inner};
            return gl.getParameter(gl.VERSION);
        }
        .try_into()
        .unwrap();
    }

    pub fn pixel_store(&self, pname: Enum, param: Int) {
        self.inner.pixel_storei(pname, param);
    }
}
