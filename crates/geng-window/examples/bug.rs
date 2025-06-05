#![allow(unused)]

use anyhow::Context as _;
use batbox_la::*;
use raw_window_handle::HasDisplayHandle;
use std::cell::{Cell, RefCell};
use std::ops::DerefMut;
use std::rc::Rc;
use ugli::Ugli;

pub struct Context {
    window: RefCell<Option<winit::window::Window>>,
    gl_ctx: RefCell<Option<glutin::context::PossiblyCurrentContext>>,
    gl_surface: RefCell<Option<glutin::surface::Surface<glutin::surface::WindowSurface>>>,
    ugli: Ugli,
    context_size: Cell<vec2<usize>>,
}

fn resume(
    window_field: &mut Option<winit::window::Window>,
    event_loop: &winit::event_loop::ActiveEventLoop,
    gl_ctx_field: &mut Option<glutin::context::PossiblyCurrentContext>,
    gl_surface_field: &mut Option<glutin::surface::Surface<glutin::surface::WindowSurface>>,
) {
    let gl_ctx = gl_ctx_field.as_mut().unwrap();
    let gl_config = glutin::config::GetGlConfig::config(gl_ctx);
    let window = window_field.take().unwrap_or_else(|| {
        let window_builder = winit::window::WindowAttributes::default();
        ::glutin_winit::finalize_window(event_loop, window_builder, &gl_config).unwrap()
    });

    let attrs =
        ::glutin_winit::GlWindow::build_surface_attributes(&window, <_>::default()).unwrap();
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
        match event_loop.display_handle().unwrap().as_raw() {
            raw_window_handle::RawDisplayHandle::Wayland(_) => {
                // on wayland need pre_present_notify
                glutin::surface::SwapInterval::DontWait
            }
            _ => glutin::surface::SwapInterval::Wait(1.try_into().unwrap()),
        },
    ) {
        log::error!("Error setting vsync: {res:?}");
    }

    window_field.replace(window);
    gl_surface_field.replace(gl_surface);
}

#[derive(Debug, Clone)]
enum Event {
    Draw,
}

fn run<EH>(once_ready: impl 'static + FnOnce(Rc<Context>) -> EH)
where
    EH: 'static + FnMut(Event) -> std::ops::ControlFlow<()>,
{
    let mut event_loop_builder = winit::event_loop::EventLoopBuilder::<()>::new();
    let event_loop = event_loop_builder.build().unwrap();

    type DynEH = Box<dyn FnMut(Event) -> std::ops::ControlFlow<()>>;

    struct App {
        window: Option<winit::window::Window>,
        context: Option<Rc<Context>>,
        gl_config: Option<glutin::config::Config>,
        once_ready: Option<Box<dyn FnOnce(Rc<Context>) -> DynEH>>,
        event_handler: Option<DynEH>,
    }

    impl App {
        fn handle(
            &mut self,
            event: winit::event::Event<()>,
            event_loop: &winit::event_loop::ActiveEventLoop,
        ) {
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

            if let winit::event::Event::Suspended = event {
                event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
            }
            if let Some(context) = &mut self.context {
                context.handle_winit_event(event, event_loop, &mut |event| {
                    if self.event_handler.as_mut().unwrap()(event).is_break() {
                        event_loop.exit();
                    }
                });
            } else if let winit::event::Event::Resumed = event {
                // First ever resume
                let (window, gl_config) = ::glutin_winit::DisplayBuilder::new()
                    .with_window_attributes(
                        // Only windows requires the window to be present before creating the display.
                        // Other platforms don't really need one.
                        //
                        // XXX if you don't care about running on android or so you can safely remove
                        // this condition and always pass the window builder.
                        if !cfg!(target_os = "android") {
                            Some(winit::window::WindowAttributes::default())
                        } else {
                            None
                        },
                    )
                    .build(
                        event_loop,
                        glutin::config::ConfigTemplateBuilder::new()
                            .with_transparency(false)
                            .prefer_hardware_accelerated(Some(true)),
                        |configs| {
                            let config = {
                                configs
                                    .into_iter()
                                    .max_by_key(glutin::config::GlConfig::num_samples)
                            }
                            .expect("Could not find fitting config");
                            log::debug!("{config:#?}");
                            config
                        },
                    )
                    .unwrap();

                let window_handle = window
                    .as_ref()
                    .map(raw_window_handle::HasWindowHandle::window_handle)
                    .transpose()
                    .unwrap();
                let gl_display = glutin::display::GetGlDisplay::display(&gl_config);
                let context_attributes = glutin::context::ContextAttributesBuilder::new()
                    .build(window_handle.map(|handle| handle.as_raw()));

                let gl_ctx = unsafe {
                    glutin::display::GlDisplay::create_context(
                        &gl_display,
                        &gl_config,
                        &context_attributes,
                    )
                    .expect("Failed to create context")
                };

                // Continuation of out android hack
                let mut window = window;
                let mut gl_ctx =
                    Some(glutin::prelude::NotCurrentGlContext::treat_as_possibly_current(gl_ctx));
                let mut gl_surface = None;

                resume(&mut window, event_loop, &mut gl_ctx, &mut gl_surface);
                window.as_ref().unwrap().request_redraw();
                let ugli = Ugli::create_from_glutin(|symbol| {
                    glutin::display::GlDisplay::get_proc_address(
                        &gl_display,
                        &std::ffi::CString::new(symbol).unwrap(),
                    )
                });
                let context = Rc::new(Context {
                    window: RefCell::new(window),
                    gl_surface: RefCell::new(gl_surface),
                    gl_ctx: RefCell::new(gl_ctx),
                    ugli,
                    context_size: Cell::new(vec2(1, 1)),
                });
                self.event_handler = Some((self.once_ready.take().unwrap())(context.clone()));
                self.context = Some(context);
            }
        }

        fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
            todo!()
        }
    }

    let mut app = App {
        window: None,
        gl_config: None,
        context: None,
        once_ready: Some(Box::new(|context| Box::new(once_ready(context)))),
        event_handler: None,
    };
    let mut state: Option<(Rc<Context>, EH)> = None;
    event_loop
        .run(move |event, event_loop| {
            app.handle(event, event_loop);
        })
        .unwrap();
}

impl Context {
    pub fn real_size(&self) -> vec2<usize> {
        let size = match &*self.window.borrow() {
            Some(window) => window.inner_size(),
            None => return vec2::ZERO,
        };
        let (width, height) = (size.width, size.height);
        vec2(width as usize, height as usize)
    }

    pub fn set_icon(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let Some(window) = &*self.window.borrow() else {
            return Ok(());
        };
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

    pub fn with_framebuffer<T>(&self, f: impl FnOnce(&mut ugli::Framebuffer) -> T) -> T {
        f(&mut ugli::Framebuffer::default(
            &self.ugli,
            self.context_size.get(),
        ))
    }

    fn handle_winit_window_event(
        &self,
        event: winit::event::WindowEvent,
        event_handler: &mut impl FnMut(Event),
    ) {
        match event {
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
            winit::event::WindowEvent::RedrawRequested => {
                if let Some(gl_surface) = &*self.gl_surface.borrow() {
                    event_handler(Event::Draw);
                    if let Some(window) = self.window.borrow().as_ref() {
                        window.pre_present_notify();
                    }
                    glutin::surface::GlSurface::swap_buffers(
                        gl_surface,
                        self.gl_ctx.borrow().as_ref().unwrap(),
                    )
                    .unwrap();
                }
                if let Some(window) = self.window.borrow().as_ref() {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn handle_winit_event(
        &self,
        event: winit::event::Event<()>,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event_handler: &mut impl FnMut(Event),
    ) {
        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                self.handle_winit_window_event(event, event_handler)
            }
            winit::event::Event::Resumed => {
                if self.gl_surface.borrow().is_none() {
                    log::debug!("Resumed!");
                    resume(
                        &mut self.window.borrow_mut(),
                        event_loop,
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
            _ => {}
        }
    }
}
fn main() {
    use batbox_la::*;
    use futures::prelude::*;
    use ugli::Ugli;

    run(move |window| {
        let ugli = window.ugli();
        let program = ugli::Program::new(
            ugli,
            [
                &ugli::Shader::new(
                    ugli,
                    ugli::ShaderType::Vertex,
                    "void main() { gl_Position = vec4(0.0, 0.0, 0.0, 0.0); }",
                )
                .unwrap(),
                &ugli::Shader::new(ugli, ugli::ShaderType::Fragment, "void main() {}").unwrap(),
            ],
        )
        .unwrap();
        ugli.raw().use_program(program.raw());
        ugli.check();
        move |_| {
            let ugli = window.ugli();
            if ugli.try_check().is_err() {
                panic!("WTF");
            }
            std::ops::ControlFlow::Continue(())
        }
    });
}
