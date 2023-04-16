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
        self.platform.set_cursor_type(cursor_type);
    }

    /// TODO should not expose?
    pub fn set_cursor_position(&self, position: vec2<f64>) {
        self.platform.set_cursor_position(position);
    }

    pub fn cursor_position(&self) -> vec2<f64> {
        self.platform.mouse_pos()
    }

    pub fn cursor_locked(&self) -> bool {
        self.platform.cursor_locked()
    }

    pub fn lock_cursor(&self) {
        self.platform.lock_cursor();
    }

    pub fn unlock_cursor(&self) {
        self.platform.unlock_cursor();
    }
}
