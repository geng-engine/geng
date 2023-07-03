use geng::prelude::*;

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("Hello, World!", |geng| async move {
        let mut events = geng.window().events();
        while let Some(event) = events.next().await {
            if let geng::Event::Draw = event {
                geng.window().with_framebuffer(|framebuffer| {
                    ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

                    // Draw text using default font
                    geng.default_font().draw(
                        framebuffer,
                        &geng::PixelPerfectCamera, // using pixel coordinates
                        "Hello, World!",
                        vec2::splat(geng::TextAlign::CENTER), // center-aligned
                        mat3::translate(framebuffer.size().map(|x| x as f32 / 2.0))
                            * mat3::scale_uniform(32.0), // in the middle of the screen 32 pixels high
                        Rgba::WHITE,
                    );
                });
            }
        }
    });
}
