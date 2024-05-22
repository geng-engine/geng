use batbox_la::*;

fn main() {
    futures::executor::block_on(async {
        let audio = geng_audio::Audio::new().unwrap();
        let sound = audio
            .decode(
                batbox_file::load_bytes("examples/sound/assets/hello.wav")
                    .await
                    .unwrap(),
            )
            .await
            .unwrap();
        audio
            .listener()
            .set_orientation(vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, 1.0));
        let mut effect = sound.effect();
        effect.set_position(vec3(0.0, 1.0, 0.0));
        effect.play();
        batbox_time::sleep(batbox_time::Duration::from_secs_f64(2.0)).await;
    });
}
