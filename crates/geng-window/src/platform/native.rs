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
        #[cfg(target_os = "android")]
        let event_loop = {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            // TODO
            // android_logger::init_once(
            //     android_logger::Config::default().with_min_level(log::Level::Trace),
            // );
            let mut event_loop = winit::event_loop::EventLoopBuilder::new()
                .with_android_app(batbox_android::app().clone())
                .build();
            use winit::platform::run_return::EventLoopExtRunReturn;
            event_loop.run_return(|e, _, flow| {
                if let winit::event::Event::Resumed = e {
                    *flow = winit::event_loop::ControlFlow::Exit;
                }
            });
            event_loop
        };
        #[cfg(not(target_os = "android"))]
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
                builder = builder.with_transparent(options.transparency);
                builder
            }))
            .build(
                &event_loop,
                glutin::config::ConfigTemplateBuilder::new()
                    .with_transparency(options.transparency),
                |mut configs| {
                    if options.vsync {}
                    let config = if options.antialias {
                        configs
                            .into_iter()
                            .max_by_key(|config| glutin::config::GlConfig::num_samples(config))
                    } else {
                        configs
                            .into_iter()
                            .min_by_key(|config| glutin::config::GlConfig::num_samples(config))
                    }
                    .expect("Could not find fitting config");
                    log::debug!("{config:#?}");
                    config
                },
            )
            .unwrap();
        let window = window.unwrap();
        // .with_vsync(options.vsync)
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
            CursorType::Pointer => GC::Pointer,
            CursorType::Drag => GC::AllScroll,
            CursorType::None => GC::Default,
        });
        self.window
            .set_cursor_visible(cursor_type != CursorType::None);
    }

    fn get_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        {
            let window_pos = |position: winit::dpi::PhysicalPosition<f64>| -> vec2<f64> {
                vec2(position.x, self.real_size().y as f64 - 1.0 - position.y)
            };
            let mut mouse_move = None;
            let mut handle_event = |e: winit::event::WindowEvent| match e {
                winit::event::WindowEvent::Focused(focus) => self.focused.set(focus),
                winit::event::WindowEvent::CloseRequested => {
                    log::debug!("Close requested");
                    self.should_close.set(true);
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    events.push(Event::Wheel {
                        delta: match delta {
                            winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y,
                            winit::event::MouseScrollDelta::LineDelta(_, dy) => dy as f64 * 51.0,
                        },
                    });
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let position = window_pos(position);
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
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    let mut edited_text = self.edited_text.borrow_mut();
                    if let Some(edited_text) = edited_text.deref_mut() {
                        if event.state == winit::event::ElementState::Pressed {
                            if event.physical_key == winit::keyboard::KeyCode::Backspace {
                                edited_text.pop();
                                events.push(Event::EditText(edited_text.clone()));
                            }
                            #[cfg(not(target_os = "android"))]
                            {
                                use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
                                if let Some(text) =
                                    KeyEventExtModifierSupplement::text_with_all_modifiers(&event)
                                {
                                    for c in text.chars().filter(|c| !char::is_ascii_control(c)) {
                                        edited_text.push(c);
                                    }
                                    events.push(Event::EditText(edited_text.clone()));
                                }
                            }
                        }
                    }
                    let key = from_winit_key(event.physical_key);
                    events.push(match event.state {
                        winit::event::ElementState::Pressed => Event::KeyDown { key },
                        winit::event::ElementState::Released => Event::KeyUp { key },
                    });
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    if new_size.width != 0 && new_size.height != 0 {
                        log::debug!("Resizing to {new_size:?}");
                        glutin::surface::GlSurface::resize(
                            &self.gl_surface,
                            &self.gl_ctx,
                            new_size.width.try_into().unwrap(),
                            new_size.height.try_into().unwrap(),
                        );
                    }
                    // glutin_winit::GlWindow::resize_surface(
                    //     &self.window,
                    //     &self.gl_surface,
                    //     &self.gl_ctx,
                    // );
                }
                winit::event::WindowEvent::Touch(touch) => {
                    let geng_touch = Touch {
                        id: touch.id,
                        position: window_pos(touch.location),
                    };
                    events.push(match touch.phase {
                        winit::event::TouchPhase::Started => Event::TouchStart(geng_touch),
                        winit::event::TouchPhase::Moved => Event::TouchMove(geng_touch),
                        winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled => {
                            Event::TouchEnd(geng_touch)
                        }
                    });
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
        #[cfg(target_os = "android")]
        batbox_android::app().show_soft_input(false);
    }

    pub fn stop_text_edit(&self) {
        *self.edited_text.borrow_mut() = None;
        #[cfg(target_os = "android")]
        batbox_android::app().hide_soft_input(false);
    }
}

fn from_winit_key(key: winit::keyboard::KeyCode) -> Key {
    use winit::keyboard::KeyCode as GKey;
    match key {
        GKey::Digit0 => Key::Num0,
        GKey::Digit1 => Key::Num1,
        GKey::Digit2 => Key::Num2,
        GKey::Digit3 => Key::Num3,
        GKey::Digit4 => Key::Num4,
        GKey::Digit5 => Key::Num5,
        GKey::Digit6 => Key::Num6,
        GKey::Digit7 => Key::Num7,
        GKey::Digit8 => Key::Num8,
        GKey::Digit9 => Key::Num9,

        GKey::KeyA => Key::A,
        GKey::KeyB => Key::B,
        GKey::KeyC => Key::C,
        GKey::KeyD => Key::D,
        GKey::KeyE => Key::E,
        GKey::KeyF => Key::F,
        GKey::KeyG => Key::G,
        GKey::KeyH => Key::H,
        GKey::KeyI => Key::I,
        GKey::KeyJ => Key::J,
        GKey::KeyK => Key::K,
        GKey::KeyL => Key::L,
        GKey::KeyM => Key::M,
        GKey::KeyN => Key::N,
        GKey::KeyO => Key::O,
        GKey::KeyP => Key::P,
        GKey::KeyQ => Key::Q,
        GKey::KeyR => Key::R,
        GKey::KeyS => Key::S,
        GKey::KeyT => Key::T,
        GKey::KeyU => Key::U,
        GKey::KeyV => Key::V,
        GKey::KeyW => Key::W,
        GKey::KeyX => Key::X,
        GKey::KeyY => Key::Y,
        GKey::KeyZ => Key::Z,

        GKey::Escape => Key::Escape,
        GKey::Space => Key::Space,
        GKey::Enter => Key::Enter,
        GKey::Backspace => Key::Backspace,
        GKey::Tab => Key::Tab,

        GKey::ShiftLeft => Key::LShift,
        GKey::ShiftRight => Key::RShift,

        GKey::ControlLeft => Key::LCtrl,
        GKey::ControlRight => Key::RCtrl,

        GKey::AltLeft => Key::LAlt,
        GKey::AltRight => Key::RAlt,

        GKey::ArrowLeft => Key::Left,
        GKey::ArrowRight => Key::Right,
        GKey::ArrowUp => Key::Up,
        GKey::ArrowDown => Key::Down,

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
