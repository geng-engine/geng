use super::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CursorType {
    Default,
    Pointer,
    Drag,
    None,
    Custom {
        image: image::RgbaImage,
        hotspot: vec2<u16>,
    },
}

impl Window {
    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        if self.cursor_locked() {
            return;
        }
        self.inner.backend.set_cursor_type(&cursor_type);
        self.inner.cursor_type.replace(cursor_type);
    }

    pub fn cursor_position(&self) -> Option<vec2<f64>> {
        if self.cursor_locked() {
            return None;
        }
        self.inner.cursor_pos.get()
    }

    pub fn cursor_locked(&self) -> bool {
        self.inner.backend.cursor_locked()
    }

    pub fn lock_cursor(&self) {
        self.inner.backend.lock_cursor();
        self.inner.backend.set_cursor_type(&CursorType::None);
    }

    pub fn unlock_cursor(&self) {
        self.inner.backend.unlock_cursor();
        self.inner
            .backend
            .set_cursor_type(&self.inner.cursor_type.borrow());
    }
}
