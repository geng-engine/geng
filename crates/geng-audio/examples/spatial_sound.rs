use batbox_la::*;

fn main() {
    futures::executor::block_on(async {
        let audio = geng_audio::Audio::new().unwrap();
        let buffer = audio
            .decode(std::fs::read("examples/sound/assets/hello.wav").unwrap())
            .await
            .unwrap();
        audio
            .listener()
            .set_orientation(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0));
        let mut effect = buffer.effect();
        effect.set_position(vec3(0.0, 1.0, 0.0));
        effect.play();
        std::thread::sleep_ms(2000);
    });
}
