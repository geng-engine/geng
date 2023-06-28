use async_recursion::*;
use futures::prelude::*;
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
    while let Some(event) = window.events().next().await {
        if let geng_window::Event::KeyDown { key } = event {
            match key {
                geng_window::Key::Escape => {
                    break;
                }
                geng_window::Key::Space => {
                    space_escape(depth + 1, window).await;
                }
                _ => {}
            }
        }
    }
    log::info!("Exiting depth {depth:?}");
}

fn main() {
    batbox_logger::init();
    let args: CliArgs = batbox_cli::parse();
    let window = geng_window::Window::new(&{
        let mut options = geng_window::Options::new("geng window demo").with_cli(&args.window);
        options.auto_close = args.auto_close.unwrap_or(false);
        options
    });
    window.clone().run(async move {
        let log_events = async {
            let mut events = window.events();
            while let Some(event) = events.next().await {
                log::info!("{event:?}");
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
    });
}
