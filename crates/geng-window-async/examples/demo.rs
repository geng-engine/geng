use batbox::prelude::*;
use geng_window_async as geng_window;

#[derive(clap::Parser)]
struct CliArgs {
    #[clap(flatten)]
    window: geng_window::CliArgs,
    #[clap(long)]
    auto_close: Option<bool>,
}

#[async_recursion(?Send)]
async fn space_escape(depth: usize, window: &geng_window::Window) {
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
                    space_escape(depth + 1, window).await;
                }
                _ => {}
            },
            geng_window::Event::Draw => {
                g = (g + timer.tick().as_secs_f64() as f32).fract();
                window.with_framebuffer(|framebuffer| {
                    ugli::clear(
                        framebuffer,
                        Some(color * Rgba::new(g, g, g, 1.0)),
                        None,
                        None,
                    );
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
            futures::select! {
                () = log_events.fuse() => {
                    unreachable!()
                },
                () = close_requested.fuse() => {
                    log::info!("Exiting because of request");
                },
                () = space_escape(0, &window).fuse() => {
                    log::info!("Exiting because space_escape finished");
                },
            }
        },
    );
}
