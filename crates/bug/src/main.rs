#![allow(unused)]

use batbox_la::*;
use raw_window_handle::HasDisplayHandle;
use std::cell::{Cell, RefCell};
use std::ops::DerefMut;
use std::rc::Rc;

pub struct Context {
    window: RefCell<Option<winit::window::Window>>,
    gl_ctx: RefCell<Option<glutin::context::PossiblyCurrentContext>>,
    gl_surface: RefCell<Option<glutin::surface::Surface<glutin::surface::WindowSurface>>>,
    context_size: Cell<vec2<usize>>,
}

impl Context {
    fn check(&self) {
        unsafe {
            if gl::GetError() != gl::NO_ERROR {
                panic!("GL ERROR");
            }
        }
    }
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
        eprintln!("Error setting vsync: {res:?}");
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
                gl::load_with(|symbol| {
                    glutin::display::GlDisplay::get_proc_address(
                        &gl_display,
                        &std::ffi::CString::new(symbol).unwrap(),
                    )
                });
                let context = Rc::new(Context {
                    window: RefCell::new(window),
                    gl_surface: RefCell::new(gl_surface),
                    gl_ctx: RefCell::new(gl_ctx),
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

    fn handle_winit_window_event(
        &self,
        event: winit::event::WindowEvent,
        event_handler: &mut impl FnMut(Event),
    ) {
        match event {
            winit::event::WindowEvent::Resized(new_size) => {
                if new_size.width != 0 && new_size.height != 0 {
                    if let Some(gl_surface) = &*self.gl_surface.borrow() {
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
                    resume(
                        &mut self.window.borrow_mut(),
                        event_loop,
                        &mut self.gl_ctx.borrow_mut(),
                        &mut self.gl_surface.borrow_mut(),
                    );
                }
            }
            winit::event::Event::Suspended => {
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
    fn raw() {
        use std::collections::HashMap;

        struct Program {
            pub(crate) cache_key: u64,
            pub(crate) handle: gl::types::GLuint,
            pub(crate) attributes: HashMap<String, AttributeInfo>,
            pub(crate) uniforms: HashMap<String, UniformInfo>,
        }

        #[derive(Debug)]
        pub struct ActiveInfo {
            pub name: String,
            pub size: gl::types::GLint,
            pub typ: gl::types::GLenum,
        }

        #[derive(Debug)]
        pub struct AttributeInfo {
            pub(crate) location: gl::types::GLuint,
            #[allow(dead_code)]
            pub(crate) info: ActiveInfo,
        }

        #[derive(Debug, Clone)]
        pub struct UniformInfo {
            pub(crate) location: gl::types::GLint,
            pub(crate) name: String,
            // pub(crate) default: Option<UniformValue>,
        }

        impl Drop for Program {
            fn drop(&mut self) {
                unsafe {
                    gl::DeleteProgram(self.handle);
                }
            }
        }

        impl Program {
            pub fn new<'a>(shaders: impl IntoIterator<Item = &'a Shader>) -> Self {
                unsafe {
                    let shaders: Vec<&Shader> = shaders.into_iter().collect();
                    let mut program = Program {
                        cache_key: {
                            use std::sync::atomic::{AtomicU64, Ordering};
                            static NEXT: AtomicU64 = AtomicU64::new(0);
                            NEXT.fetch_add(1, Ordering::SeqCst)
                        },
                        // ugli: ugli.clone(),
                        handle: gl::CreateProgram(),
                        uniforms: HashMap::new(),
                        attributes: HashMap::new(),
                    };
                    assert!(program.handle != 0);
                    for shader in &shaders {
                        gl::AttachShader(program.handle, shader.handle);
                    }
                    gl::LinkProgram(program.handle);
                    for shader in &shaders {
                        gl::DetachShader(program.handle, shader.handle);
                    }

                    // Check for errors
                    let mut link_status = 0_i32;
                    gl::GetProgramiv(program.handle, gl::LINK_STATUS, &mut link_status as *mut _);
                    if link_status == gl::FALSE as _ {
                        panic!("link issue");
                    }

                    // Get attributes
                    // let attribute_count = gl
                    //     .get_program_parameter_int(&program.handle, raw::ACTIVE_ATTRIBUTES)
                    //     as usize;
                    // for index in 0..attribute_count {
                    //     let info = gl.get_active_attrib(&program.handle, index as raw::UInt);
                    //     let name = info.name.clone();
                    //     let location = gl.get_attrib_location(&program.handle, &name);
                    //     // TODO: why can't this be an assert?
                    //     if location >= 0 {
                    //         program.attributes.insert(
                    //             name,
                    //             AttributeInfo {
                    //                 location: location as raw::UInt,
                    //                 info,
                    //             },
                    //         );
                    //     }
                    // }

                    // Get uniforms
                    // let uniform_count = gl
                    //     .get_program_parameter_int(&program.handle, raw::ACTIVE_UNIFORMS)
                    //     as usize;
                    // for index in 0..uniform_count {
                    //     let info = gl.get_active_uniform(&program.handle, index as raw::UInt);
                    //     for index in 0..info.size {
                    //         let name = match info.size {
                    //             1 => info.name.clone(),
                    //             _ => format!("{}[{index}]", info.name.strip_suffix("[0]").unwrap()),
                    //         };
                    //         if let Some(location) = gl.get_uniform_location(&program.handle, &name)
                    //         {
                    //             let default = UniformValue::get_value(&program, &location, &info);
                    //             // info!("{:?}", name);
                    //             program.uniforms.insert(
                    //                 name.clone(),
                    //                 UniformInfo {
                    //                     location,
                    //                     name,
                    //                     // info,
                    //                     default,
                    //                 },
                    //             );
                    //         }
                    //     }
                    // }

                    // ugli.debug_check();
                    program
                }
            }
            pub fn uniform_info(&self, name: &str) -> Option<UniformInfo> {
                self.uniforms.get(name).cloned()
            }
            pub(crate) fn bind(&self) {
                unsafe {
                    gl::UseProgram(self.handle);
                }
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub enum ShaderType {
            Vertex,
            Fragment,
        }

        pub struct Shader {
            pub(crate) handle: gl::types::GLuint,
        }

        impl Drop for Shader {
            fn drop(&mut self) {
                unsafe { gl::DeleteShader(self.handle) }
            }
        }

        impl Shader {
            pub fn new(shader_type: ShaderType, source: &str) -> Self {
                unsafe {
                    let shader = Self {
                        handle: gl::CreateShader(match shader_type {
                            ShaderType::Vertex => gl::VERTEX_SHADER,
                            ShaderType::Fragment => gl::FRAGMENT_SHADER,
                        }),
                    };
                    assert!(shader.handle != 0);
                    gl::ShaderSource(
                        shader.handle,
                        1,
                        [source.as_ptr() as *const gl::types::GLchar].as_ptr(),
                        [source.as_bytes().len() as gl::types::GLint].as_ptr(),
                    );
                    gl::CompileShader(shader.handle);
                    let mut compile_status = 0_i32;
                    gl::GetShaderiv(
                        shader.handle,
                        gl::COMPILE_STATUS,
                        &mut compile_status as *mut _,
                    );
                    if compile_status == gl::FALSE as _ {
                        panic!("shader compile failed");
                    }
                    // ugli.debug_check();
                    shader
                }
            }
        }

        let program = Program::new([
            &Shader::new(
                ShaderType::Vertex,
                "void main() { gl_Position = vec4(0.0, 0.0, 0.0, 0.0); }",
            ),
            &Shader::new(ShaderType::Fragment, "void main() {}"),
        ]);
        program.bind();
    }

    run(move |window| {
        raw();
        window.check();
        move |_| {
            window.check();
            std::ops::ControlFlow::Continue(())
        }
    });
}
