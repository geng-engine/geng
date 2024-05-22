use geng_web_audio_api::{self as audio, AudioNode as _};

fn main() {
    futures::executor::block_on(async {
        let audio = audio::AudioContext::new().unwrap();
        let bytes = std::fs::read("examples/sound/assets/hello.wav").unwrap(); // Dont use std::fs on the web
        let buffer = audio.decode(bytes).await.unwrap();
        let mut source = audio::AudioBufferSourceNode::new(&audio);
        source.set_buffer(buffer);
        let panner = audio::PannerNode::new(&audio);
        source.connect(&panner);
        panner.connect(&audio.destination());

        // No sound, but is sound if these two lines are commented
        panner.disconnect();
        panner.connect(&audio.destination());

        source.start_with_offset(0.0);

        std::thread::sleep(std::time::Duration::from_secs(1)); // Dont sleep the thread on the web
    });
}
