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
        let music: geng::Sound = geng
            .asset_manager()
            .load(run_dir().join("assets/music.mp3"))
            .await
            .unwrap();
        let mut events = geng.window().events();
        let mut music_effect = None::<geng::SoundEffect>;
        let fade_duration = time::Duration::from_secs_f64(1.0);
        while let Some(event) = events.next().await {
            match event {
                geng::Event::Draw => {
                    geng.window().with_framebuffer(|framebuffer| {
                        let framebuffer_size = framebuffer.size();
                        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
                        if let Some(effect) = &music_effect {
                            geng.default_font().draw(
                                framebuffer,
                                &geng::PixelPerfectCamera,
                                &format!(
                                    "music: {:.2?}/{:.2?}",
                                    effect.playback_position(),
                                    music.duration()
                                ),
                                vec2::splat(geng::TextAlign::CENTER),
                                mat3::translate(framebuffer_size.map(|x| x as f32) / 2.0)
                                    * mat3::scale_uniform(32.0),
                                Rgba::WHITE,
                            );
                        }
                    });
                }
                geng::Event::KeyPress {
                    key: geng::Key::Space,
                } => {
                    hello.play();
                }
                geng::Event::KeyPress { key: geng::Key::M } => {
                    let mut effect = music.effect();
                    effect.set_looped(true);
                    effect.fade_in(fade_duration);
                    effect.play();
                    music_effect = Some(effect);
                }
                geng::Event::KeyRelease { key: geng::Key::M } => {
                    if let Some(mut effect) = music_effect.take() {
                        effect.fade_out(fade_duration);
                    }
                }
                _ => {}
            }
        }
    });
}
