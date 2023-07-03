use super::*;

use anyhow::Context as _;
use std::{ffi::c_void, ops::DerefMut};

pub struct Context {
    options: Options,
    window: RefCell<Option<winit::window::Window>>,
    gl_ctx: RefCell<Option<glutin::context::PossiblyCurrentContext>>,
    gl_surface: RefCell<Option<glutin::surface::Surface<glutin::surface::WindowSurface>>>,
    event_loop: RefCell<Option<winit::event_loop::EventLoop<()>>>,
    is_fullscreen: Cell<bool>,
    lock_cursor: Cell<bool>,
    cursor_pos: Cell<vec2<f64>>,
    ugli: Ugli,
    context_size: Cell<vec2<usize>>,
    edited_text: RefCell<Option<String>>,
}

fn create_window_builder(options: &Options) -> winit::window::WindowBuilder {
    let mut builder = winit::window::WindowBuilder::new();
    if let Some(size) = options.size {
        builder = builder.with_inner_size(winit::dpi::PhysicalSize {
            width: size.x as u32,
            height: size.y as u32,
        });
    }
    builder = builder.with_title(&options.title);
    builder = builder.with_transparent(options.transparency);
    builder = builder.with_visible(!options.start_hidden);
    builder
}

fn resume<T>(
    window_field: &mut Option<winit::window::Window>,
    window_target: Option<&winit::event_loop::EventLoopWindowTarget<T>>,
    options: &Options,
    gl_ctx_field: &mut Option<glutin::context::PossiblyCurrentContext>,
    gl_surface_field: &mut Option<glutin::surface::Surface<glutin::surface::WindowSurface>>,
) {
    let gl_ctx = gl_ctx_field.as_mut().unwrap();
    let gl_config = glutin::config::GetGlConfig::config(gl_ctx);
    let window = window_field.take().unwrap_or_else(|| {
        let window_builder = create_window_builder(options);
        ::glutin_winit::finalize_window(window_target.unwrap(), window_builder, &gl_config).unwrap()
    });

    let attrs = ::glutin_winit::GlWindow::build_surface_attributes(&window, <_>::default());
    let gl_surface = unsafe {
        glutin::prelude::GlDisplay::create_window_surface(
            &glutin::display::GetGlDisplay::display(&gl_config),
            &gl_config,
            &attrs,
        )
        .unwrap()
    };

    // Make it current.
    glutin::context::PossiblyCurrentGlContext::make_current(gl_ctx, &gl_surface).unwrap();

    // Try setting vsync.
    if let Err(res) = glutin::surface::GlSurface::set_swap_interval(
        &gl_surface,
        gl_ctx,
        if options.vsync {
            glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap())
        } else {
            glutin::surface::SwapInterval::DontWait
        },
    ) {
        log::error!("Error setting vsync: {res:?}");
    }

    window_field.replace(window);
    gl_surface_field.replace(gl_surface);
}

impl Context {
    pub fn new(options: &Options) -> Self {
        #[cfg(target_os = "android")]
        let event_loop = {
            use winit::platform::android::EventLoopBuilderExtAndroid;
            winit::event_loop::EventLoopBuilder::new()
                .with_android_app(batbox_android::app().clone())
                .build()
        };
        #[cfg(not(target_os = "android"))]
        let event_loop = winit::event_loop::EventLoop::<()>::new();

        let (window, gl_config) = ::glutin_winit::DisplayBuilder::new()
            .with_window_builder(
                // Only windows requires the window to be present before creating the display.
                // Other platforms don't really need one.
                //
                // XXX if you don't care about running on android or so you can safely remove
                // this condition and always pass the window builder.
                if !cfg!(target_os = "android") {
                    Some(create_window_builder(options))
                } else {
                    None
                },
            )
            .build(
                &event_loop,
                glutin::config::ConfigTemplateBuilder::new()
                    .with_transparency(options.transparency),
                |configs| {
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
        let raw_window_handle = window
            .as_ref()
            .map(|window| raw_window_handle::HasRawWindowHandle::raw_window_handle(window));
        let gl_display = glutin::display::GetGlDisplay::display(&gl_config);
        let context_attributes =
            glutin::context::ContextAttributesBuilder::new().build(raw_window_handle);

        let gl_ctx = unsafe {
            glutin::display::GlDisplay::create_context(&gl_display, &gl_config, &context_attributes)
                .expect("Failed to create context")
        };

        // Continuation of out android hack
        let mut window = window;
        let mut gl_ctx =
            Some(glutin::prelude::NotCurrentGlContext::treat_as_possibly_current(gl_ctx));
        let mut gl_surface = None;
        let mut event_loop = event_loop;
        if cfg!(target_os = "android") {
            use winit::platform::pump_events::EventLoopExtPumpEvents;
            let mut resumed = false;
            while !resumed {
                event_loop.pump_events(|e, window_target, _flow| {
                    if let winit::event::Event::Resumed = e {
                        resume(
                            &mut window,
                            Some(window_target),
                            options,
                            &mut gl_ctx,
                            &mut gl_surface,
                        );
                        resumed = true;
                        // *flow = winit::event_loop::ControlFlow::Exit;
                    }
                });
            }
        } else {
            resume::<()>(&mut window, None, options, &mut gl_ctx, &mut gl_surface);
        }
        assert!(gl_surface.is_some());

        let ugli = Ugli::create_from_glutin(|symbol| {
            glutin::display::GlDisplay::get_proc_address(
                &gl_display,
                &std::ffi::CString::new(symbol).unwrap(),
            ) as *const c_void
        });
        Self {
            options: options.clone(),
            window: RefCell::new(window),
            event_loop: RefCell::new(Some(event_loop)),
            gl_surface: RefCell::new(gl_surface),
            gl_ctx: RefCell::new(gl_ctx),
            ugli,
            is_fullscreen: Cell::new(false),
            lock_cursor: Cell::new(false),
            cursor_pos: Cell::new(vec2(0.0, 0.0)),
            context_size: Cell::new(vec2(1, 1)),
            edited_text: RefCell::new(None),
        }
    }

    pub fn real_size(&self) -> vec2<usize> {
        let size = match &*self.window.borrow() {
            Some(window) => window.inner_size(),
            None => return vec2::ZERO,
        };
        let (width, height) = (size.width, size.height);
        vec2(width as usize, height as usize)
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        let Some(window) = &*self.window.borrow() else { return };
        window.set_fullscreen(if fullscreen {
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
        let Some(window) = &*self.window.borrow() else { return Ok(()) };
        let image = image::open(path).context(format!("Failed to load {path:?}"))?;
        let image = match image {
            image::DynamicImage::ImageRgba8(image) => image,
            _ => image.to_rgba8(),
        };
        let width = image.width();
        let height = image.height();
        let icon = winit::window::Icon::from_rgba(image.into_raw(), width, height)?;
        window.set_window_icon(Some(icon));
        Ok(())
    }

    pub fn ugli(&self) -> &Ugli {
        &self.ugli
    }

    pub fn with_framebuffer(&self, f: impl FnOnce(&mut ugli::Framebuffer)) {
        f(&mut ugli::Framebuffer::default(
            &self.ugli,
            self.context_size.get(),
        ));
    }

    pub fn cursor_locked(&self) -> bool {
        self.lock_cursor.get()
    }

    pub fn lock_cursor(&self) {
        let Some(window) = &*self.window.borrow() else { return };
        if let Err(lock_e) = window.set_cursor_grab(winit::window::CursorGrabMode::Locked) {
            if let Err(confine_e) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined)
            {
                log::error!("Failed to lock cursor: {lock_e}, {confine_e}");
            }
        }
        self.lock_cursor.set(true);
    }

    pub fn unlock_cursor(&self) {
        self.lock_cursor.set(false);
        let Some(window) = &*self.window.borrow() else { return };
        if let Err(e) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
            log::error!("Failed to unlock cursor: {e}");
        }
    }

    pub fn set_cursor_type(&self, cursor_type: CursorType) {
        let Some(window) = &*self.window.borrow() else { return };
        use winit::window::CursorIcon as GC;
        window.set_cursor_icon(match cursor_type {
            CursorType::Default => GC::Default,
            CursorType::Pointer => GC::Pointer,
            CursorType::Drag => GC::AllScroll,
            CursorType::None => GC::Default,
        });
        window.set_cursor_visible(cursor_type != CursorType::None);
    }

    fn handle_winit_window_event(
        &self,
        event: winit::event::WindowEvent<'_>,
        event_handler: &mut impl FnMut(Event),
    ) {
        let screen_pos = |position: winit::dpi::PhysicalPosition<f64>| -> vec2<f64> {
            vec2(position.x, self.real_size().y as f64 - 1.0 - position.y)
        };
        match event {
            winit::event::WindowEvent::Focused(focus) => {
                event_handler(Event::Focused(focus));
            }
            winit::event::WindowEvent::CloseRequested => {
                event_handler(Event::CloseRequested);
            }
            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                event_handler(Event::Wheel {
                    delta: match delta {
                        winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y,
                        winit::event::MouseScrollDelta::LineDelta(_, dy) => dy as f64 * 51.0,
                    },
                });
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                let position = screen_pos(position);
                self.cursor_pos.set(position);
                event_handler(Event::CursorMove { position });
            }
            winit::event::WindowEvent::MouseInput { state, button, .. } => {
                let button = match button {
                    winit::event::MouseButton::Left => Some(MouseButton::Left),
                    winit::event::MouseButton::Middle => Some(MouseButton::Middle),
                    winit::event::MouseButton::Right => Some(MouseButton::Right),
                    _ => None,
                };
                if let Some(button) = button {
                    event_handler(match state {
                        winit::event::ElementState::Pressed => Event::MousePress { button },
                        winit::event::ElementState::Released => Event::MouseRelease { button },
                    });
                }
            }
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                {
                    let mut edited_text_ref = self.edited_text.borrow_mut();
                    if let Some(edited_text) = edited_text_ref.deref_mut() {
                        if event.state == winit::event::ElementState::Pressed {
                            if event.physical_key == winit::keyboard::KeyCode::Backspace {
                                edited_text.pop();
                                let event = Event::EditText(edited_text.clone());
                                std::mem::drop(edited_text_ref);
                                event_handler(event);
                            } else {
                                #[cfg(not(target_os = "android"))]
                                {
                                    use winit::platform::modifier_supplement::KeyEventExtModifierSupplement;
                                    if let Some(text) =
                                        KeyEventExtModifierSupplement::text_with_all_modifiers(
                                            &event,
                                        )
                                    {
                                        for c in text.chars().filter(|c| !char::is_ascii_control(c))
                                        {
                                            edited_text.push(c);
                                        }
                                        let event = Event::EditText(edited_text.clone());
                                        std::mem::drop(edited_text_ref);
                                        event_handler(event);
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(key) = from_winit_key(event.physical_key) {
                    event_handler(match event.state {
                        winit::event::ElementState::Pressed => Event::KeyPress { key },
                        winit::event::ElementState::Released => Event::KeyRelease { key },
                    });
                }
            }
            winit::event::WindowEvent::Resized(new_size) => {
                if new_size.width != 0 && new_size.height != 0 {
                    if let Some(gl_surface) = &*self.gl_surface.borrow() {
                        log::debug!("Resizing to {new_size:?}");
                        glutin::surface::GlSurface::resize(
                            gl_surface,
                            self.gl_ctx.borrow().as_ref().unwrap(),
                            new_size.width.try_into().unwrap(),
                            new_size.height.try_into().unwrap(),
                        );
                        self.context_size
                            .set(vec2(new_size.width, new_size.height).map(|x| x as usize));
                    }
                }
            }
            winit::event::WindowEvent::Touch(touch) => {
                let geng_touch = Touch {
                    id: touch.id,
                    position: screen_pos(touch.location),
                };
                event_handler(match touch.phase {
                    winit::event::TouchPhase::Started => Event::TouchStart(geng_touch),
                    winit::event::TouchPhase::Moved => Event::TouchMove(geng_touch),
                    winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled => {
                        Event::TouchEnd(geng_touch)
                    }
                });
            }
            _ => {}
        }
    }

    fn handle_winit_event(
        &self,
        event: winit::event::Event<'_, ()>,
        window_target: &winit::event_loop::EventLoopWindowTarget<()>,
        event_handler: &mut impl FnMut(Event),
    ) {
        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                self.handle_winit_window_event(event, event_handler)
            }
            winit::event::Event::RedrawEventsCleared => {
                if let Some(gl_surface) = &*self.gl_surface.borrow() {
                    event_handler(Event::Draw);
                    glutin::surface::GlSurface::swap_buffers(
                        gl_surface,
                        self.gl_ctx.borrow().as_ref().unwrap(),
                    )
                    .unwrap();
                }
            }
            winit::event::Event::Resumed => {
                if self.gl_surface.borrow().is_none() {
                    log::debug!("Resumed!");
                    resume(
                        &mut self.window.borrow_mut(),
                        Some(window_target),
                        &self.options,
                        &mut self.gl_ctx.borrow_mut(),
                        &mut self.gl_surface.borrow_mut(),
                    );
                }
            }
            winit::event::Event::Suspended => {
                log::debug!("Suspended!");
                self.window.take();
                if let Some(_gl_surface) = self.gl_surface.take() {
                    self.gl_ctx.replace(Some(
                        glutin::prelude::NotCurrentGlContext::treat_as_possibly_current(
                            glutin::prelude::PossiblyCurrentGlContext::make_not_current(
                                self.gl_ctx.take().unwrap(),
                            )
                            .unwrap(),
                        ),
                    ));
                }
            }
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion {
                    delta: (delta_x, delta_y),
                } => event_handler(Event::RawMouseMove {
                    delta: vec2(delta_x, -delta_y),
                }),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn run(
        self: Rc<Self>,
        mut event_handler: impl FnMut(Event) -> std::ops::ControlFlow<()> + 'static,
    ) {
        self.clone()
            .event_loop
            .borrow_mut()
            .take()
            .expect("Event loop already was started")
            .run(move |event, window_target, control_flow| {
                control_flow.set_wait();
                self.handle_winit_event(event, window_target, &mut |event| {
                    if let Event::KeyPress { key: Key::Escape } = event {
                        self.unlock_cursor();
                    }
                    if event_handler(event).is_break() {
                        control_flow.set_exit();
                    }
                });
            })
            .expect("Event loop error");
    }

    pub fn start_text_edit(&self, text: &str) {
        *self.edited_text.borrow_mut() = Some(text.to_owned());
        #[cfg(target_os = "android")]
        batbox_android::app().show_soft_input(true);
    }

    pub fn stop_text_edit(&self) {
        *self.edited_text.borrow_mut() = None;
        #[cfg(target_os = "android")]
        batbox_android::app().hide_soft_input(false);
    }

    pub fn is_editing_text(&self) -> bool {
        self.edited_text.borrow().is_some()
    }

    pub fn show(&self) {
        if let Some(window) = &mut *self.window.borrow_mut() {
            window.set_visible(true);
        }
    }
}

fn from_winit_key(key: winit::keyboard::KeyCode) -> Option<Key> {
    use winit::keyboard::KeyCode as GKey;
    Some(match key {
        GKey::Backquote => Key::Backquote,
        GKey::Backslash => Key::Backslash,
        GKey::BracketLeft => Key::BracketLeft,
        GKey::BracketRight => Key::BracketRight,
        GKey::Comma => Key::Comma,
        GKey::Digit0 => Key::Digit0,
        GKey::Digit1 => Key::Digit1,
        GKey::Digit2 => Key::Digit2,
        GKey::Digit3 => Key::Digit3,
        GKey::Digit4 => Key::Digit4,
        GKey::Digit5 => Key::Digit5,
        GKey::Digit6 => Key::Digit6,
        GKey::Digit7 => Key::Digit7,
        GKey::Digit8 => Key::Digit8,
        GKey::Digit9 => Key::Digit9,
        GKey::Equal => Key::Equal,
        GKey::IntlBackslash => Key::IntlBackslash,
        GKey::IntlRo => Key::IntlRo,
        GKey::IntlYen => Key::IntlYen,
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
        GKey::Minus => Key::Minus,
        GKey::Period => Key::Period,
        GKey::Quote => Key::Quote,
        GKey::Semicolon => Key::Semicolon,
        GKey::Slash => Key::Slash,
        GKey::AltLeft => Key::AltLeft,
        GKey::AltRight => Key::AltRight,
        GKey::Backspace => Key::Backspace,
        GKey::CapsLock => Key::CapsLock,
        GKey::ContextMenu => Key::ContextMenu,
        GKey::ControlLeft => Key::ControlLeft,
        GKey::ControlRight => Key::ControlRight,
        GKey::Enter => Key::Enter,
        GKey::SuperLeft => Key::SuperLeft,
        GKey::SuperRight => Key::SuperRight,
        GKey::ShiftLeft => Key::ShiftLeft,
        GKey::ShiftRight => Key::ShiftRight,
        GKey::Space => Key::Space,
        GKey::Tab => Key::Tab,
        GKey::Delete => Key::Delete,
        GKey::End => Key::End,
        GKey::Help => Key::Help,
        GKey::Home => Key::Home,
        GKey::Insert => Key::Insert,
        GKey::PageDown => Key::PageDown,
        GKey::PageUp => Key::PageUp,
        GKey::ArrowDown => Key::ArrowDown,
        GKey::ArrowLeft => Key::ArrowLeft,
        GKey::ArrowRight => Key::ArrowRight,
        GKey::ArrowUp => Key::ArrowUp,
        GKey::NumLock => Key::NumLock,
        GKey::Numpad0 => Key::Numpad0,
        GKey::Numpad1 => Key::Numpad1,
        GKey::Numpad2 => Key::Numpad2,
        GKey::Numpad3 => Key::Numpad3,
        GKey::Numpad4 => Key::Numpad4,
        GKey::Numpad5 => Key::Numpad5,
        GKey::Numpad6 => Key::Numpad6,
        GKey::Numpad7 => Key::Numpad7,
        GKey::Numpad8 => Key::Numpad8,
        GKey::Numpad9 => Key::Numpad9,
        GKey::NumpadAdd => Key::NumpadAdd,
        GKey::NumpadBackspace => Key::NumpadBackspace,
        GKey::NumpadClear => Key::NumpadClear,
        GKey::NumpadClearEntry => Key::NumpadClearEntry,
        GKey::NumpadComma => Key::NumpadComma,
        GKey::NumpadDecimal => Key::NumpadDecimal,
        GKey::NumpadDivide => Key::NumpadDivide,
        GKey::NumpadEnter => Key::NumpadEnter,
        GKey::NumpadEqual => Key::NumpadEqual,
        GKey::NumpadHash => Key::NumpadHash,
        GKey::NumpadMemoryAdd => Key::NumpadMemoryAdd,
        GKey::NumpadMemoryClear => Key::NumpadMemoryClear,
        GKey::NumpadMemoryRecall => Key::NumpadMemoryRecall,
        GKey::NumpadMemoryStore => Key::NumpadMemoryStore,
        GKey::NumpadMemorySubtract => Key::NumpadMemorySubtract,
        GKey::NumpadMultiply => Key::NumpadMultiply,
        GKey::NumpadParenLeft => Key::NumpadParenLeft,
        GKey::NumpadParenRight => Key::NumpadParenRight,
        GKey::NumpadStar => Key::NumpadStar,
        GKey::NumpadSubtract => Key::NumpadSubtract,
        GKey::Escape => Key::Escape,
        GKey::BrowserBack => Key::Back,
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
            log::trace!("Unrecognized key: {:?}", key);
            return None;
        }
    })
}
