use batbox::prelude::*;
use geng_window_async as window;
use ugli::Ugli;

mod renderer {
    use super::*;

    #[derive(ugli::Vertex)]
    struct Vertex {
        a_pos: vec2<f32>,
    }

    pub struct Renderer {
        program: ugli::Program,
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

    impl Renderer {
        pub fn new(ugli: &Ugli) -> Self {
            Self {
                program: geng_shader::Library::new(ugli, false, None)
                    .compile(PROGRAM_SOURCE)
                    .unwrap(),
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
    }
}

use renderer::Renderer;

#[derive(clap::Parser)]
struct CliArgs {
    #[clap(flatten)]
    window: window::CliArgs,
    #[clap(long)]
    auto_close: Option<bool>,
}

#[async_recursion(?Send)]
async fn space_escape(depth: usize, renderer: &Renderer, window: &window::Window) {
    log::info!("Entering depth {depth:?}");
    let timer = Timer::new();
    let transform =
        mat3::rotate(thread_rng().gen()) * mat3::scale_uniform((depth as f32 * 0.1).exp());
    while let Some(event) = window.events().next().await {
        match event {
            window::Event::KeyPress { key } => match key {
                window::Key::Escape => {
                    break;
                }
                window::Key::Space => {
                    space_escape(depth + 1, renderer, window).await;
                }
                window::Key::F => {
                    window.toggle_fullscreen();
                }
                window::Key::M => {
                    if window.cursor_locked() {
                        log::info!("unlocking cursor");
                        window.unlock_cursor();
                    } else {
                        log::info!("locking cursor");
                        window.lock_cursor();
                    }
                }
                window::Key::T if window.is_key_pressed(window::Key::ControlLeft) => {
                    if window.is_editing_text() {
                        log::info!("stop editing text");
                        window.stop_text_edit();
                    } else {
                        log::info!("start editing text");
                        window.start_text_edit("text");
                    }
                }
                _ => {}
            },
            window::Event::Draw => {
                let color = Hsla::new(timer.elapsed().as_secs_f64() as f32, 1.0, 0.5, 1.0).into();
                window.with_framebuffer(|framebuffer| {
                    renderer.draw(framebuffer, transform, color);
                });
            }
            _ => {}
        }
    }
    log::info!("Exiting depth {depth:?}");
}

fn main() {
    logger::init();
    let args: CliArgs = cli::parse();
    window::run(
        {
            let mut options = window::Options::new("geng window demo").with_cli(&args.window);
            options.auto_close = args.auto_close.unwrap_or(false);
            options
        },
        |window| async move {
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
            let renderer = Renderer::new(window.ugli());
            let space_escape = space_escape(0, &renderer, &window);
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
        },
    );
}
