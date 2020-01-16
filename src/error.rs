use crate::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum Error {
    InvalidEnum = ugl::INVALID_ENUM,
    InvalidValue = ugl::INVALID_VALUE,
    InvalidOperation = ugl::INVALID_OPERATION,
    OutOfMemory = ugl::OUT_OF_MEMORY,
    InvalidFramebufferOperation = ugl::INVALID_FRAMEBUFFER_OPERATION,
    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    ContextLost = ugl::CONTEXT_LOST,
    Unknown,
}

impl Ugli {
    pub fn try_check(&self) -> Result<(), Error> {
        let error = self.inner.get_error();
        if error == ugl::NO_ERROR {
            return Ok(());
        }
        Err(error.try_into().unwrap_or(Error::Unknown))
    }
    pub fn check(&self) {
        self.try_check().expect("GL error");
    }
    pub fn debug_check(&self) {
        #[cfg(debug_assertions)]
        self.check();
    }
}
