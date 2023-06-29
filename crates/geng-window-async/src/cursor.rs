use super::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CursorType {
    Default,
    Pointer,
    Drag,
    None,
}

impl Window {
    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        self.inner.cursor_type.set(cursor_type);
        if self.cursor_locked() {
            return;
        }
        self.inner.platform.set_cursor_type(cursor_type);
    }

    pub fn cursor_position(&self) -> Option<vec2<f64>> {
        if self.cursor_locked() {
            return None;
        }
        self.inner.cursor_pos.get()
    }

    pub fn cursor_locked(&self) -> bool {
        self.inner.platform.cursor_locked()
    }

    pub fn lock_cursor(&self) {
        self.inner.platform.lock_cursor();
        self.inner.platform.set_cursor_type(CursorType::None);
    }

    pub fn unlock_cursor(&self) {
        self.inner.platform.unlock_cursor();
        self.inner
            .platform
            .set_cursor_type(self.inner.cursor_type.get());
    }
}
