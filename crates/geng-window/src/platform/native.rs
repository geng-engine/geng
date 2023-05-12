use super::*;

use anyhow::Context as _;
use std::{ffi::c_void, ops::DerefMut};

pub struct Context {
    window: winit::window::Window,
    gl_ctx: glutin::context::PossiblyCurrentContext,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
    event_loop: RefCell<winit::event_loop::EventLoop<()>>,
    is_fullscreen: Cell<bool>,
    focused: Cell<bool>,
    lock_cursor: Cell<bool>,
    should_close: Cell<bool>,
    mouse_pos: Rc<Cell<vec2<f64>>>,
    ugli: Ugli,
    edited_text: RefCell<Option<String>>,
}

impl Context {
    pub fn new(options: &Options) -> Self {
        let event_loop = winit::event_loop::EventLoop::<()>::new();
        let (window, gl_config) = glutin_winit::DisplayBuilder::new()
            .with_window_builder(Some({
                let mut builder = winit::window::WindowBuilder::new();
                if let Some(size) = options.size {
                    builder = builder.with_inner_size(winit::dpi::PhysicalSize {
                        width: size.x as u32,
                        height: size.y as u32,
                    });
                }
                builder = builder.with_title(&options.title);
                builder
            }))
            .build(&event_loop, Default::default(), |configs| {
                if options.vsync {}
                if options.antialias {
                    configs
                        .into_iter()
                        .max_by_key(|config| glutin::config::GlConfig::num_samples(config))
                } else {
                    configs
                        .into_iter()
                        .min_by_key(|config| glutin::config::GlConfig::num_samples(config))
                }
                .expect("Could not find fitting config")
            })
            .unwrap();
        let window = window.unwrap();
        // .with_vsync(options.vsync)
        // .with_multisampling(if options.antialias { 8 } else { 0 })
        let raw_window_handle = raw_window_handle::HasRawWindowHandle::raw_window_handle(&window);

        let gl_display = glutin::display::GetGlDisplay::display(&gl_config);
        let context_attributes = glutin::context::ContextAttributesBuilder::new()
            // TODO
            // .with_profile(glutin::context::GlProfile::Core)
            // .with_context_api(glutin::context::ContextApi::OpenGl(Some(
            //     glutin::context::Version::new(3, 3),
            // )))
            .build(Some(raw_window_handle));

        let (gl_surface, gl_ctx) = {
            let attrs = glutin_winit::GlWindow::build_surface_attributes(&window, <_>::default());
            let surface = unsafe {
                glutin::display::GlDisplay::create_window_surface(&gl_display, &gl_config, &attrs)
                    .expect("Failed to create window surface")
            };
            let context = glutin::prelude::NotCurrentGlContextSurfaceAccessor::make_current(
                unsafe {
                    glutin::display::GlDisplay::create_context(
                        &gl_display,
                        &gl_config,
                        &context_attributes,
                    )
                    .expect("Failed to create context")
                },
                &surface,
            )
            .expect("Failed to make context current");
            (surface, context)
        };
        glutin::surface::GlSurface::set_swap_interval(
            &gl_surface,
            &gl_ctx,
            match options.vsync {
                true => glutin::surface::SwapInterval::Wait(1.try_into().unwrap()),
                false => glutin::surface::SwapInterval::DontWait,
            },
        )
        .expect("Failed to setup vsync");
        let ugli = Ugli::create_from_glutin(|symbol| {
            glutin::display::GlDisplay::get_proc_address(
                &gl_display,
                &std::ffi::CString::new(symbol).unwrap(),
            ) as *const c_void
        });
        Self {
            window,
            event_loop: RefCell::new(event_loop),
            gl_surface,
            gl_ctx,
            ugli,
            is_fullscreen: Cell::new(false),
            focused: Cell::new(false),
            lock_cursor: Cell::new(false),
            should_close: Cell::new(false),
            mouse_pos: Rc::new(Cell::new(vec2(0.0, 0.0))),
            edited_text: RefCell::new(None),
        }
    }

    pub fn real_size(&self) -> vec2<usize> {
        let size = self.window.inner_size();
        let (width, height) = (size.width, size.height);
        vec2(width as usize, height as usize)
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.window.set_fullscreen(if fullscreen {
            Some(winit::window::Fullscreen::Borderless(None))
        } else {
            None
        });
        self.is_fullscreen.set(fullscreen);
    }

    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen.get()
    }

    pub fn set_icon(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let image = image::open(path).context(format!("Failed to load {path:?}"))?;
        let image = match image {
            image::DynamicImage::ImageRgba8(image) => image,
            _ => image.to_rgba8(),
        };
        let width = image.width();
        let height = image.height();
        let icon = winit::window::Icon::from_rgba(image.into_raw(), width, height)?;
        self.window.set_window_icon(Some(icon));
        Ok(())
    }

    pub fn swap_buffers(&self, event_handler: impl Fn(Event)) {
        glutin::surface::GlSurface::swap_buffers(&self.gl_surface, &self.gl_ctx).unwrap();
        for event in self.get_events() {
            if let Event::KeyDown { key: Key::Escape } = event {
                self.unlock_cursor();
            }
            event_handler(event);
        }
        if self.lock_cursor.get() && self.focused.get() {
            let pos = (self.real_size() / 2).map(|x| x as f64);
            self.set_cursor_position(pos);
        }
    }

    pub fn ugli(&self) -> &Ugli {
        &self.ugli
    }

    pub fn should_close(&self) -> bool {
        self.should_close.get()
    }

    pub fn mouse_pos(&self) -> vec2<f64> {
        self.mouse_pos.get()
    }

    pub fn cursor_locked(&self) -> bool {
        self.lock_cursor.get()
    }

    pub fn lock_cursor(&self) {
        self.lock_cursor.set(true);
        // TODO let _ = self.glutin_window.window().set_cursor_grab(true);
    }

    pub fn unlock_cursor(&self) {
        self.lock_cursor.set(false);
    }

    pub fn set_cursor_position(&self, position: vec2<f64>) {
        self.mouse_pos.set(position);
        let position = vec2(position.x, self.real_size().y as f64 - 1.0 - position.y); // TODO: WAT
        if let Err(e) = self
            .window
            .set_cursor_position(winit::dpi::PhysicalPosition::new(position.x, position.y))
        {
            log::error!("Failed to set cursor position: {:?}", e);
        }
    }

    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        use winit::window::CursorIcon as GC;
        self.window.set_cursor_icon(match cursor_type {
            CursorType::Default => GC::Default,
            CursorType::Pointer => GC::Hand,
            CursorType::Drag => GC::AllScroll,
            CursorType::None => GC::Default,
        });
        self.window
            .set_cursor_visible(cursor_type != CursorType::None);
    }

    fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        {
            let mut mouse_move = None;
            let mut handle_event = |e: winit::event::WindowEvent| match e {
                winit::event::WindowEvent::Focused(focus) => self.focused.set(focus),
                winit::event::WindowEvent::CloseRequested => self.should_close.set(true),
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    events.push(Event::Wheel {
                        delta: match delta {
                            winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y,
                            winit::event::MouseScrollDelta::LineDelta(_, dy) => dy as f64 * 51.0,
                        },
                    });
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let position = vec2(position.x, self.real_size().y as f64 - 1.0 - position.y);
                    mouse_move = Some(position);
                    self.mouse_pos.set(position);
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        winit::event::MouseButton::Left => Some(MouseButton::Left),
                        winit::event::MouseButton::Middle => Some(MouseButton::Middle),
                        winit::event::MouseButton::Right => Some(MouseButton::Right),
                        _ => None,
                    };
                    if let Some(button) = button {
                        let position = self.mouse_pos.get();
                        events.push(match state {
                            winit::event::ElementState::Pressed => {
                                Event::MouseDown { position, button }
                            }
                            winit::event::ElementState::Released => {
                                Event::MouseUp { position, button }
                            }
                        });
                    }
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    let mut edited_text = self.edited_text.borrow_mut();
                    if let Some(text) = edited_text.deref_mut() {
                        if input.state == winit::event::ElementState::Pressed
                            && input.virtual_keycode == Some(winit::event::VirtualKeyCode::Back)
                        {
                            text.pop();
                            events.push(Event::EditText(text.clone()));
                        }
                    } else if let Some(key) = input.virtual_keycode {
                        let key = from_glutin_key(key);
                        events.push(match input.state {
                            winit::event::ElementState::Pressed => Event::KeyDown { key },
                            winit::event::ElementState::Released => Event::KeyUp { key },
                        });
                    }
                }
                winit::event::WindowEvent::Resized(_new_size) => {
                    glutin_winit::GlWindow::resize_surface(
                        &self.window,
                        &self.gl_surface,
                        &self.gl_ctx,
                    );
                }
                winit::event::WindowEvent::ReceivedCharacter(c) => {
                    if !c.is_ascii_control() {
                        let mut edited_text = self.edited_text.borrow_mut();
                        if let Some(text) = edited_text.deref_mut() {
                            text.push(c);
                            events.push(Event::EditText(text.clone()));
                        }
                    }
                }
                _ => {}
            };
            use winit::platform::run_return::EventLoopExtRunReturn;
            let prev_mouse = self.mouse_pos.get();
            self.event_loop.borrow_mut().run_return(|e, _, flow| {
                if let winit::event::Event::WindowEvent { event: e, .. } = e {
                    handle_event(e)
                }
                *flow = winit::event_loop::ControlFlow::Exit;
            });
            if let Some(position) = mouse_move {
                // This is here because of weird delta
                events.push(Event::MouseMove {
                    position,
                    delta: position - prev_mouse,
                });
            }
        }
        events
    }

    pub fn start_text_edit(&self, text: &str) {
        *self.edited_text.borrow_mut() = Some(text.to_owned());
        // TODO: iOS/Android?
    }

    pub fn stop_text_edit(&self) {
        *self.edited_text.borrow_mut() = None;
        // TODO: iOS/Android?
    }
}

fn from_glutin_key(key: winit::event::VirtualKeyCode) -> Key {
    use winit::event::VirtualKeyCode as GKey;
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
        GKey::Tab => Key::Tab,

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
        GKey::Insert => Key::Insert,
        GKey::Delete => Key::Delete,
        GKey::Home => Key::Home,
        GKey::End => Key::End,

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
            log::warn!("Unrecognized key: {:?}", key);
            Key::Unknown
        }
    }
}
