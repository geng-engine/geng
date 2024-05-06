use web_audio_api::{
    context::{AudioContext, BaseAudioContext},
    node::{AudioBufferSourceNode, AudioNode, PannerNode},
};

fn main() {
    let audio = AudioContext::new(Default::default());
    let buffer = audio
        .decode_audio_data_sync(std::io::Cursor::new(
            std::fs::read("examples/sound/assets/hello.wav").unwrap(),
        ))
        .unwrap();
    let mut source = AudioBufferSourceNode::new(&audio, Default::default());
    source.set_buffer(buffer);
    let panner = PannerNode::new(&audio, Default::default());
    source.connect(&panner);
    panner.connect(&audio.destination());

    // No sound, but is sound if these two lines are commented
    panner.disconnect();
    panner.connect(&audio.destination());

    source.start_at_with_offset(0.0, 0.0);
    std::thread::sleep(std::time::Duration::from_secs(1));
}
