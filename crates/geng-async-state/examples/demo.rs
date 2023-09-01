use std::rc::Rc;

use batbox_cli as cli;
use batbox_color::*;
use batbox_la::*;
use batbox_time::*;
use futures::{future::LocalBoxFuture, prelude::*};
use geng_async_state as state;
use geng_window as window;
use rand::prelude::*;
use state::ActiveState;
use ugli::Ugli;

mod renderer {
    use super::*;

    #[derive(ugli::Vertex)]
    struct Vertex {
        a_pos: vec2<f32>,
    }

    pub struct Renderer {
        ugli: Ugli,
        program: ugli::Program,
        transition_program: ugli::Program,
        quad: ugli::VertexBuffer<Vertex>,
        data: ugli::VertexBuffer<Vertex>,
    }

    const PROGRAM_SOURCE: &str = r#"
    #ifdef VERTEX_SHADER
    attribute vec2 a_pos;
    uniform mat3 u_transform;
    void main() {
        gl_Position = mat4(u_transform) * vec4(a_pos, 0.0, 1.0);
    }
    #endif
    #ifdef FRAGMENT_SHADER
    uniform vec4 u_color;
    void main() {
        gl_FragColor = u_color;
    }
    #endif
    "#;

    const TRANSITION_SOURCE: &str = r#"
    varying vec2 v_uv;
    #ifdef VERTEX_SHADER
    attribute vec2 a_pos;
    uniform float u_swipe;
    void main() {
        v_uv = a_pos;
        gl_Position = vec4(a_pos * 2.0 - 1.0, 0.0, 1.0);
        gl_Position.x += (1.0 - u_swipe) * 2.0;
    }
    #endif
    #ifdef FRAGMENT_SHADER
    uniform sampler2D u_texture;
    uniform float u_a;
    void main() {
        gl_FragColor = texture2D(u_texture, v_uv);
        gl_FragColor.a *= u_a;
    }
    #endif
    "#;

    impl Renderer {
        pub fn new(ugli: &Ugli) -> Self {
            let shader_lib = geng_shader::Library::new(ugli, false, None);
            Self {
                ugli: ugli.clone(),
                program: shader_lib.compile(PROGRAM_SOURCE).unwrap(),
                transition_program: shader_lib.compile(TRANSITION_SOURCE).unwrap(),
                quad: ugli::VertexBuffer::new_static(
                    ugli,
                    vec![
                        Vertex {
                            a_pos: vec2(0.0, 0.0),
                        },
                        Vertex {
                            a_pos: vec2(1.0, 0.0),
                        },
                        Vertex {
                            a_pos: vec2(1.0, 1.0),
                        },
                        Vertex {
                            a_pos: vec2(0.0, 1.0),
                        },
                    ],
                ),
                data: ugli::VertexBuffer::new_static(
                    ugli,
                    vec![
                        Vertex {
                            a_pos: vec2(-0.5, -0.5),
                        },
                        Vertex {
                            a_pos: vec2(0.5, -0.5),
                        },
                        Vertex {
                            a_pos: vec2(0.0, 0.5),
                        },
                    ],
                ),
            }
        }
        pub fn draw(
            &self,
            framebuffer: &mut ugli::Framebuffer,
            transform: mat3<f32>,
            color: Rgba<f32>,
        ) {
            ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
            ugli::draw(
                framebuffer,
                &self.program,
                ugli::DrawMode::Triangles,
                &self.data,
                ugli::uniforms! {
                    u_transform: transform,
                    u_color: color,
                },
                ugli::DrawParameters::default(),
            );
        }

        pub fn draw_crossfade(
            &self,
            from: &mut dyn FnMut(&mut ugli::Framebuffer),
            to: &mut dyn FnMut(&mut ugli::Framebuffer),
            framebuffer: &mut ugli::Framebuffer,
            alpha: f32,
        ) {
            from(framebuffer);
            let mut texture = ugli::Texture::new_uninitialized(&self.ugli, framebuffer.size());
            {
                to(&mut ugli::Framebuffer::new_color(
                    &self.ugli,
                    ugli::ColorAttachment::Texture(&mut texture),
                ));
            }
            ugli::draw(
                framebuffer,
                &self.transition_program,
                ugli::DrawMode::TriangleFan,
                &self.quad,
                ugli::uniforms! {
                    u_texture: &texture,
                    u_swipe: 1.0,
                    u_a: alpha,
                },
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..Default::default()
                },
            );
        }

        pub fn draw_swipe(
            &self,
            from: &mut dyn FnMut(&mut ugli::Framebuffer),
            to: &mut dyn FnMut(&mut ugli::Framebuffer),
            framebuffer: &mut ugli::Framebuffer,
            progress: f32,
        ) {
            from(framebuffer);
            let mut texture = ugli::Texture::new_uninitialized(&self.ugli, framebuffer.size());
            {
                to(&mut ugli::Framebuffer::new_color(
                    &self.ugli,
                    ugli::ColorAttachment::Texture(&mut texture),
                ));
            }
            ugli::draw(
                framebuffer,
                &self.transition_program,
                ugli::DrawMode::TriangleFan,
                &self.quad,
                ugli::uniforms! {
                    u_texture: &texture,
                    u_swipe: progress,
                    u_a: 1.0,
                },
                ugli::DrawParameters {
                    blend_mode: Some(ugli::BlendMode::straight_alpha()),
                    ..Default::default()
                },
            );
        }
    }
}

use renderer::Renderer;

struct Crossfade<'a> {
    timer: Timer,
    renderer: &'a Renderer,
}

impl<'a> Crossfade<'a> {
    pub fn new(renderer: &'a Renderer) -> Self {
        Self {
            timer: Timer::new(),
            renderer,
        }
    }
}

impl state::Transition for Crossfade<'_> {
    fn finished(&self) -> bool {
        self.timer.elapsed().as_secs_f64() > 1.0
    }

    fn draw(
        &mut self,
        from: &mut dyn FnMut(&mut ugli::Framebuffer),
        to: &mut dyn FnMut(&mut ugli::Framebuffer),
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.renderer.draw_crossfade(
            from,
            to,
            framebuffer,
            (self.timer.elapsed().as_secs_f64() as f32).min(1.0),
        )
    }
}

struct Swipe<'a> {
    timer: Timer,
    renderer: &'a Renderer,
}

impl<'a> Swipe<'a> {
    pub fn new(renderer: &'a Renderer) -> Self {
        Self {
            timer: Timer::new(),
            renderer,
        }
    }
}

impl state::Transition for Swipe<'_> {
    fn finished(&self) -> bool {
        self.timer.elapsed().as_secs_f64() > 1.0
    }

    fn draw(
        &mut self,
        from: &mut dyn FnMut(&mut ugli::Framebuffer),
        to: &mut dyn FnMut(&mut ugli::Framebuffer),
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.renderer.draw_swipe(
            from,
            to,
            framebuffer,
            (self.timer.elapsed().as_secs_f64() as f32).min(1.0),
        )
    }
}

#[derive(clap::Parser)]
struct CliArgs {
    #[clap(flatten)]
    window: window::CliArgs,
    #[clap(long)]
    auto_close: Option<bool>,
}

struct SpaceEscape {
    depth: usize,
    renderer: Rc<Renderer>,
    window: window::Window,
    timer: Timer,
    transform: mat3<f32>,
}

impl SpaceEscape {
    pub fn new(window: window::Window, renderer: Rc<Renderer>, depth: usize) -> Self {
        Self {
            window,
            renderer,
            depth,
            timer: Timer::new(),
            transform: mat3::rotate(thread_rng().gen())
                * mat3::scale_uniform((depth as f32 * 0.1).exp()),
        }
    }
}

impl SpaceEscape {
    fn run<'b, 'a: 'b, T>(
        self,
        state_task: &'b switch_resume::Task<'a, T>,
    ) -> LocalBoxFuture<'b, Box<dyn ActiveState>> {
        self.run_impl(state_task).boxed_local()
    }
    async fn run_impl<'a, T>(
        mut self,
        state_task: &switch_resume::Task<'a, T>,
    ) -> Box<dyn ActiveState> {
        log::info!("Entering depth {:?}", self.depth);
        while let Some(event) = self.window.events().next().await {
            match event {
                window::Event::KeyPress { key } => match key {
                    window::Key::Escape => {
                        break;
                    }
                    window::Key::Space => {
                        let renderer = self.renderer.clone();
                        let into = switch_resume::run({
                            let window = self.window.clone();
                            let renderer = self.renderer.clone();
                            let depth = self.depth;
                            move |state_task| async move {
                                SpaceEscape::new(window, renderer, depth + 1)
                                    .run(&state_task)
                                    .await
                            }
                        });

                        let mut state = geng_async_state::transition(
                            &self.window.clone(),
                            &mut self,
                            &mut Crossfade::new(&renderer),
                            into,
                        )
                        .await;
                        let window = self.window.clone();
                        state_task
                            .switch(move |resume| async move {
                                geng_async_state::transition(
                                    &window,
                                    &mut state,
                                    &mut Swipe::new(&renderer),
                                    resume(()),
                                )
                                .await
                            })
                            .await;
                    }
                    window::Key::F => {
                        self.window.toggle_fullscreen();
                    }
                    window::Key::M => {
                        if self.window.cursor_locked() {
                            log::info!("unlocking cursor");
                            self.window.unlock_cursor();
                        } else {
                            log::info!("locking cursor");
                            self.window.lock_cursor();
                        }
                    }
                    window::Key::T if self.window.is_key_pressed(window::Key::ControlLeft) => {
                        if self.window.is_editing_text() {
                            log::info!("stop editing text");
                            self.window.stop_text_edit();
                        } else {
                            log::info!("start editing text");
                            self.window.start_text_edit("text");
                        }
                    }
                    _ => {}
                },
                window::Event::Draw => {
                    geng_async_state::with_current_framebuffer(
                        &self.window.clone(),
                        |framebuffer| {
                            self.draw(framebuffer);
                        },
                    );
                }
                _ => {}
            }
        }
        log::info!("Exiting depth {:?}", self.depth);
        Box::new(self) as Box<dyn ActiveState>
    }
}

impl state::ActiveState for SpaceEscape {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let color = Hsla::new(self.timer.elapsed().as_secs_f64() as f32, 1.0, 0.5, 1.0).into();
        self.renderer.draw(framebuffer, self.transform, color);
    }
}

fn main() {
    batbox_logger::init();
    let args: CliArgs = cli::parse();
    window::run(
        &{
            let mut options = window::Options::new("geng window demo");
            options.with_cli(&args.window);
            options.auto_close = args.auto_close.unwrap_or(false);
            options.start_hidden = true;
            options
        },
        |window| {
            async move {
                // TODO sleep
                window.show();

                window.set_cursor_type(window::CursorType::Pointer);
                let log_events = async {
                    let mut events = window.events();
                    while let Some(event) = events.next().await {
                        if event != window::Event::Draw {
                            log::info!("{event:?}");
                        }
                    }
                };
                let close_requested = async {
                    window
                        .events()
                        .filter(|event| future::ready(*event == window::Event::CloseRequested))
                        .next()
                        .await;
                };
                let renderer = Rc::new(Renderer::new(window.ugli()));
                let window = window.clone();
                async fn run(
                    window: window::Window,
                    renderer: Rc<Renderer>,
                    state_task: switch_resume::Task<'_, ()>,
                ) {
                    SpaceEscape::new(window, renderer, 0).run(&state_task).await;
                }
                let space_escape =
                    switch_resume::run(|task| async move { run(window, renderer, task).await });
                futures::select! {
                    () = log_events.fuse() => {
                        unreachable!()
                    },
                    () = close_requested.fuse() => {
                        log::info!("Exiting because of request");
                    },
                    () = space_escape.fuse() => {
                        log::info!("Exiting because space_escape finished");
                    },
                }
            }
        },
    );
}
