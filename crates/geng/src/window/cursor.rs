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
        #[cfg(target_arch = "wasm32")]
        {
            let cursor_type = match cursor_type {
                CursorType::Default => "initial",
                CursorType::Pointer => "pointer",
                CursorType::Drag => "all-scroll",
                CursorType::None => "none",
            };
            // TODO: only canvas
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .style()
                .set_property("cursor", cursor_type)
                .unwrap();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use glutin::window::CursorIcon as GC;
            self.glutin_window
                .window()
                .set_cursor_icon(match cursor_type {
                    CursorType::Default => GC::Default,
                    CursorType::Pointer => GC::Hand,
                    CursorType::Drag => GC::AllScroll,
                    CursorType::None => GC::Default,
                });
            self.glutin_window
                .window()
                .set_cursor_visible(cursor_type != CursorType::None);
        };
    }

    pub fn set_cursor_position(&self, position: Vec2<f64>) {
        self.mouse_pos.set(position);
        let position = vec2(position.x, self.size().y as f64 - 1.0 - position.y); // TODO: WAT
        #[cfg(target_arch = "wasm32")]
        unimplemented!();
        #[cfg(not(target_arch = "wasm32"))]
        self.glutin_window
            .window()
            .set_cursor_position(glutin::dpi::PhysicalPosition::new(position.x, position.y))
            .expect("Failed to set cursor position");
    }

    pub fn cursor_position(&self) -> Vec2<f64> {
        self.mouse_pos.get()
    }

    pub fn cursor_locked(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        return web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .pointer_lock_element()
            .is_some();
        #[cfg(not(target_arch = "wasm32"))]
        return self.lock_cursor.get();
    }

    pub fn lock_cursor(&self) {
        self.lock_cursor.set(true);
        #[cfg(target_arch = "wasm32")]
        self.canvas.request_pointer_lock();
        #[cfg(not(target_arch = "wasm32"))]
        self.glutin_window.window().set_cursor_visible(false);
        // let _ = self.glutin_window.window().set_cursor_grab(true);
    }

    pub fn unlock_cursor(&self) {
        self.lock_cursor.set(false);
        #[cfg(not(target_arch = "wasm32"))]
        self.glutin_window.window().set_cursor_visible(true);
    }
}
