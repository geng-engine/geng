use super::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum Error {
    InvalidEnum = raw::INVALID_ENUM,
    InvalidValue = raw::INVALID_VALUE,
    InvalidOperation = raw::INVALID_OPERATION,
    OutOfMemory = raw::OUT_OF_MEMORY,
    InvalidFramebufferOperation = raw::INVALID_FRAMEBUFFER_OPERATION,
    #[cfg(target_arch = "wasm32")]
    ContextLost = raw::CONTEXT_LOST,
    Unknown,
}

impl Error {
    fn from_raw(raw: raw::Enum) -> Option<Self> {
        if raw == raw::NO_ERROR {
            return None;
        }
        Some(raw.try_into().unwrap_or(Self::Unknown))
    }
}

impl Ugli {
    pub fn try_check(&self) -> Result<(), Error> {
        match Error::from_raw(self.inner.raw.get_error()) {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
    pub fn check(&self) {
        self.try_check().expect("GL error");
    }
    pub fn debug_check(&self) {
        #[cfg(debug_assertions)]
        self.check();
    }
}
