use super::*;

impl Window {
    pub(crate) fn internal_get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        {
            let mut handle_event = |e: glutin::event::WindowEvent| match e {
                glutin::event::WindowEvent::CloseRequested => self.should_close.set(true),
                glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                    events.push(Event::Wheel {
                        delta: match delta {
                            glutin::event::MouseScrollDelta::PixelDelta(pos) => pos.y,
                            glutin::event::MouseScrollDelta::LineDelta(_, dy) => dy as f64 * 51.0,
                        },
                    });
                }
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    let position = vec2(position.x, self.size().y as f64 - 1.0 - position.y);
                    events.push(Event::MouseMove { position })
                }
                glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        glutin::event::MouseButton::Left => Some(MouseButton::Left),
                        glutin::event::MouseButton::Middle => Some(MouseButton::Middle),
                        glutin::event::MouseButton::Right => Some(MouseButton::Right),
                        _ => None,
                    };
                    if let Some(button) = button {
                        let position = self.mouse_pos.get();
                        events.push(match state {
                            glutin::event::ElementState::Pressed => {
                                Event::MouseDown { position, button }
                            }
                            glutin::event::ElementState::Released => {
                                Event::MouseUp { position, button }
                            }
                        });
                    }
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        let key = from_glutin_key(key);
                        events.push(match input.state {
                            glutin::event::ElementState::Pressed => Event::KeyDown { key: key },
                            glutin::event::ElementState::Released => Event::KeyUp { key: key },
                        });
                    }
                }
                glutin::event::WindowEvent::Resized(new_size) => {
                    self.glutin_window.resize(new_size);
                }
                _ => {}
            };
            use glutin::platform::desktop::EventLoopExtDesktop;
            self.glutin_event_loop
                .borrow_mut()
                .run_return(|e, _, flow| {
                    if let glutin::event::Event::WindowEvent { event: e, .. } = e {
                        handle_event(e)
                    }
                    *flow = glutin::event_loop::ControlFlow::Exit;
                });
        }
        events
    }
}

fn from_glutin_key(key: glutin::event::VirtualKeyCode) -> Key {
    use glutin::event::VirtualKeyCode as GKey;
    match key {
        GKey::Key0 => Key::Num0,
        GKey::Key1 => Key::Num1,
        GKey::Key2 => Key::Num2,
        GKey::Key3 => Key::Num3,
        GKey::Key4 => Key::Num4,
        GKey::Key5 => Key::Num5,
        GKey::Key6 => Key::Num6,
        GKey::Key7 => Key::Num7,
        GKey::Key8 => Key::Num8,
        GKey::Key9 => Key::Num9,

        GKey::A => Key::A,
        GKey::B => Key::B,
        GKey::C => Key::C,
        GKey::D => Key::D,
        GKey::E => Key::E,
        GKey::F => Key::F,
        GKey::G => Key::G,
        GKey::H => Key::H,
        GKey::I => Key::I,
        GKey::J => Key::J,
        GKey::K => Key::K,
        GKey::L => Key::L,
        GKey::M => Key::M,
        GKey::N => Key::N,
        GKey::O => Key::O,
        GKey::P => Key::P,
        GKey::Q => Key::Q,
        GKey::R => Key::R,
        GKey::S => Key::S,
        GKey::T => Key::T,
        GKey::U => Key::U,
        GKey::V => Key::V,
        GKey::W => Key::W,
        GKey::X => Key::X,
        GKey::Y => Key::Y,
        GKey::Z => Key::Z,

        GKey::Escape => Key::Escape,
        GKey::Space => Key::Space,
        GKey::Return => Key::Enter,
        GKey::Back => Key::Backspace,

        GKey::LShift => Key::LShift,
        GKey::RShift => Key::RShift,

        GKey::LControl => Key::LCtrl,
        GKey::RControl => Key::RCtrl,

        GKey::LAlt => Key::LAlt,
        GKey::RAlt => Key::RAlt,

        GKey::Left => Key::Left,
        GKey::Right => Key::Right,
        GKey::Up => Key::Up,
        GKey::Down => Key::Down,

        GKey::PageUp => Key::PageUp,
        GKey::PageDown => Key::PageDown,

        GKey::F1 => Key::F1,
        GKey::F2 => Key::F2,
        GKey::F3 => Key::F3,
        GKey::F4 => Key::F4,
        GKey::F5 => Key::F5,
        GKey::F6 => Key::F6,
        GKey::F7 => Key::F7,
        GKey::F8 => Key::F8,
        GKey::F9 => Key::F9,
        GKey::F10 => Key::F10,
        GKey::F11 => Key::F11,
        GKey::F12 => Key::F12,

        _ => {
            warn!("Unrecognized key: {:?}", key);
            Key::Unknown
        }
    }
}
