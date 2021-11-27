pub use egui;

use geng::prelude::*;

mod painter;

use painter::*;

/// Bindings for [egui](https://github.com/emilk/egui).
pub struct EguiGeng {
    geng: Geng,
    egui_ctx: egui::CtxRef,
    egui_input: egui::RawInput,
    painter: Painter,
    shapes: Option<Vec<egui::epaint::ClippedShape>>,
    screen_height: f32,
}

impl EguiGeng {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            egui_ctx: egui::CtxRef::default(),
            egui_input: egui::RawInput::default(),
            painter: Painter::new(geng),
            shapes: None,
            screen_height: 1.0,
        }
    }

    /// Use to call ui methods
    pub fn get_context(&self) -> &egui::CtxRef {
        &self.egui_ctx
    }

    /// Call at the beginning of the frame.
    /// Implement your ui logic inbetween [begin_frame] and [end_frame].
    pub fn begin_frame(&mut self) {
        self.gather_input();
        self.egui_ctx.begin_frame(self.egui_input.take());
    }

    /// Call at the end of the frame.
    /// Should be called after the ui logic.
    pub fn end_frame(&mut self) {
        let (output, shapes) = self.egui_ctx.end_frame();
        if self.shapes.is_some() {
            error!("Egui contents have not been drawn. Ensure to call `draw` after `end_frame`");
        }
        self.shapes = Some(shapes);

        // TODO: process output
    }

    /// Call after [end_frame] to draw the ui.
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        // Update screen size
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        self.screen_height = framebuffer_size.y;
        self.egui_input.screen_rect = Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::Vec2::new(framebuffer_size.x, framebuffer_size.y),
        ));

        // Render mesh
        if let Some(shapes) = self.shapes.take() {
            let paint_jobs = self.egui_ctx.tessellate(shapes);
            self.painter
                .paint(framebuffer, paint_jobs, &self.egui_ctx.texture());
        } else {
            error!("Failed to draw egui. Ensure to call `draw` after `end_frame`");
        }
    }

    /// Call every time you receive an event from the engine in [geng::State::handle_event].
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::Wheel { delta } => {
                if self.geng.window().is_key_pressed(geng::Key::LShift) {
                    self.egui_input.scroll_delta = egui::Vec2::new(delta as f32, 0.0);
                } else {
                    self.egui_input.scroll_delta = egui::Vec2::new(0.0, delta as f32);
                }
            }
            geng::Event::KeyDown { key } => {
                if let Some(key) = egui_key(key) {
                    let modifiers = self.get_modifiers();
                    self.egui_input.events.push(egui::Event::Key {
                        key,
                        modifiers,
                        pressed: true,
                    });
                    if let Some(mut symbol) = key_char(key) {
                        if modifiers.shift {
                            symbol = symbol.to_uppercase().next().unwrap();
                        }
                        self.egui_input
                            .events
                            .push(egui::Event::Text(symbol.to_string()));
                    }
                }
            }
            geng::Event::KeyUp { key } => {
                if let Some(key) = egui_key(key) {
                    self.egui_input.events.push(egui::Event::Key {
                        key,
                        modifiers: self.get_modifiers(),
                        pressed: false,
                    });
                }
            }
            geng::Event::MouseDown { position, button } => {
                let button = egui_button(button);
                self.egui_input.events.push(egui::Event::PointerButton {
                    pos: self.mouse_to_pos(position),
                    button,
                    pressed: true,
                    modifiers: self.get_modifiers(),
                });
            }
            geng::Event::MouseMove { position, .. } => {
                self.egui_input
                    .events
                    .push(egui::Event::PointerMoved(self.mouse_to_pos(position)));
            }
            geng::Event::MouseUp { position, button } => {
                let button = egui_button(button);
                self.egui_input.events.push(egui::Event::PointerButton {
                    pos: self.mouse_to_pos(position),
                    button,
                    pressed: false,
                    modifiers: self.get_modifiers(),
                });
            }
            _ => (),
        }
    }

    fn gather_input(&mut self) {
        self.egui_input.modifiers = self.get_modifiers();
    }

    fn get_modifiers(&self) -> egui::Modifiers {
        let window = self.geng.window();
        egui::Modifiers {
            alt: window.is_key_pressed(geng::Key::LAlt),
            ctrl: window.is_key_pressed(geng::Key::LCtrl),
            shift: window.is_key_pressed(geng::Key::LShift),
            ..default()
        }
    }

    fn mouse_to_pos(&self, mouse: Vec2<f64>) -> egui::Pos2 {
        egui::Pos2::new(mouse.x as f32, self.screen_height - mouse.y as f32)
    }
}

/// Converts [egui::Pos2] to [Vec2]. Moves the origin from top-left to bottom-left.
fn pos_to_vec(pos: egui::Pos2, height: f32) -> Vec2<f32> {
    vec2(pos.x, height - pos.y)
}

fn egui_button(geng_button: geng::MouseButton) -> egui::PointerButton {
    match geng_button {
        geng::MouseButton::Left => egui::PointerButton::Primary,
        geng::MouseButton::Middle => egui::PointerButton::Middle,
        geng::MouseButton::Right => egui::PointerButton::Secondary,
    }
}

fn egui_key(geng_key: geng::Key) -> Option<egui::Key> {
    use egui::Key::*;
    match geng_key {
        geng::Key::Num0 => Some(Num0),
        geng::Key::Num1 => Some(Num1),
        geng::Key::Num2 => Some(Num2),
        geng::Key::Num3 => Some(Num3),
        geng::Key::Num4 => Some(Num4),
        geng::Key::Num5 => Some(Num5),
        geng::Key::Num6 => Some(Num6),
        geng::Key::Num7 => Some(Num7),
        geng::Key::Num8 => Some(Num8),
        geng::Key::Num9 => Some(Num9),
        geng::Key::A => Some(A),
        geng::Key::B => Some(B),
        geng::Key::C => Some(C),
        geng::Key::D => Some(D),
        geng::Key::E => Some(E),
        geng::Key::F => Some(F),
        geng::Key::G => Some(G),
        geng::Key::H => Some(H),
        geng::Key::I => Some(I),
        geng::Key::J => Some(J),
        geng::Key::K => Some(K),
        geng::Key::L => Some(L),
        geng::Key::M => Some(M),
        geng::Key::N => Some(N),
        geng::Key::O => Some(O),
        geng::Key::P => Some(P),
        geng::Key::Q => Some(Q),
        geng::Key::R => Some(R),
        geng::Key::S => Some(S),
        geng::Key::T => Some(T),
        geng::Key::U => Some(U),
        geng::Key::V => Some(V),
        geng::Key::W => Some(W),
        geng::Key::X => Some(X),
        geng::Key::Y => Some(Y),
        geng::Key::Z => Some(Z),
        geng::Key::Escape => Some(Escape),
        geng::Key::Space => Some(Space),
        geng::Key::Enter => Some(Enter),
        geng::Key::Backspace => Some(Backspace),
        geng::Key::Left => Some(ArrowLeft),
        geng::Key::Right => Some(ArrowRight),
        geng::Key::Up => Some(ArrowUp),
        geng::Key::Down => Some(ArrowDown),
        geng::Key::PageUp => Some(PageUp),
        geng::Key::PageDown => Some(PageDown),
        _ => None,
    }
}

fn key_char(key: egui::Key) -> Option<char> {
    match key {
        egui::Key::A => Some('a'),
        egui::Key::B => Some('b'),
        egui::Key::C => Some('c'),
        egui::Key::D => Some('d'),
        egui::Key::E => Some('e'),
        egui::Key::F => Some('f'),
        egui::Key::G => Some('g'),
        egui::Key::H => Some('h'),
        egui::Key::I => Some('i'),
        egui::Key::J => Some('j'),
        egui::Key::K => Some('k'),
        egui::Key::L => Some('l'),
        egui::Key::M => Some('m'),
        egui::Key::N => Some('n'),
        egui::Key::O => Some('o'),
        egui::Key::P => Some('p'),
        egui::Key::Q => Some('q'),
        egui::Key::R => Some('r'),
        egui::Key::S => Some('s'),
        egui::Key::T => Some('t'),
        egui::Key::U => Some('u'),
        egui::Key::V => Some('v'),
        egui::Key::W => Some('w'),
        egui::Key::X => Some('x'),
        egui::Key::Y => Some('y'),
        egui::Key::Z => Some('z'),
        egui::Key::Num0 => Some('0'),
        egui::Key::Num1 => Some('1'),
        egui::Key::Num2 => Some('2'),
        egui::Key::Num3 => Some('3'),
        egui::Key::Num4 => Some('4'),
        egui::Key::Num5 => Some('5'),
        egui::Key::Num6 => Some('6'),
        egui::Key::Num7 => Some('7'),
        egui::Key::Num8 => Some('8'),
        egui::Key::Num9 => Some('9'),
        _ => None,
    }
}
