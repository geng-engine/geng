use geng::prelude::*;

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("Hello, World!", |geng| async move {
        let hello: geng::Sound = geng
            .asset_manager()
            .load(run_dir().join("assets/hello.wav"))
            .await
            .unwrap();
        let mut events = geng.window().events();
        let mut hello_effect = None;
        while let Some(event) = events.next().await {
            match event {
                geng::Event::Draw => {
                    geng.window().with_framebuffer(|framebuffer| {
                        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
                    });
                }
                geng::Event::KeyPress {
                    key: geng::Key::Space,
                } => {
                    hello_effect = Some(hello.play());
                }
                _ => {}
            }
        }
    });
}
