use batbox::prelude::*;
use geng_window_async as geng_window;
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
    void main() {
        gl_Position = vec4(a_pos, 0.0, 1.0);
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
        pub fn draw(&self, framebuffer: &mut ugli::Framebuffer, color: Rgba<f32>) {
            ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
            ugli::draw(
                framebuffer,
                &self.program,
                ugli::DrawMode::Triangles,
                &self.data,
                ugli::uniforms! {
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
    window: geng_window::CliArgs,
    #[clap(long)]
    auto_close: Option<bool>,
}

#[async_recursion(?Send)]
async fn space_escape(depth: usize, renderer: &Renderer, window: &geng_window::Window) {
    log::info!("Entering depth {depth:?}");
    let color: Rgba<f32> = Hsla::new(thread_rng().gen(), 1.0, 0.5, 1.0).into();
    let mut g = 0.0;
    let mut timer = Timer::new();
    while let Some(event) = window.events().next().await {
        match event {
            geng_window::Event::KeyDown { key } => match key {
                geng_window::Key::Escape => {
                    break;
                }
                geng_window::Key::Space => {
                    space_escape(depth + 1, renderer, window).await;
                }
                _ => {}
            },
            geng_window::Event::Draw => {
                g = (g + timer.tick().as_secs_f64() as f32).fract();
                window.with_framebuffer(|framebuffer| {
                    renderer.draw(framebuffer, color * Rgba::new(g, g, g, 1.0));
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
    geng_window::run(
        {
            let mut options = geng_window::Options::new("geng window demo").with_cli(&args.window);
            options.auto_close = args.auto_close.unwrap_or(false);
            options
        },
        |window| async move {
            let log_events = async {
                let mut events = window.events();
                while let Some(event) = events.next().await {
                    if event != geng_window::Event::Draw {
                        log::info!("{event:?}");
                    }
                }
            };
            let close_requested = async {
                window
                    .events()
                    .filter(|event| future::ready(*event == geng_window::Event::CloseRequested))
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
