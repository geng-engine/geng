use super::*;

pub struct AudioContext {
    // output_stream: rodio::OutputStream,
    output_stream_handle: Arc<rodio::OutputStreamHandle>,
}

impl AudioContext {
    pub(crate) fn new() -> Self {
        // https://github.com/RustAudio/rodio/issues/214
        let stream_handle = std::thread::spawn(|| {
            let (stream, handle) = rodio::OutputStream::try_default().unwrap();
            mem::forget(stream);
            handle
        })
        .join()
        .unwrap();
        Self {
            output_stream_handle: Arc::new(stream_handle),
        }
    }

    pub fn set_listener_position(&self, pos: Vec3<f64>) {
        // TODO
    }

    pub fn set_listener_orientation(&self, forward: Vec3<f64>, up: Vec3<f64>) {
        // TODO
    }
}

pub struct Sound {
    geng: Geng,
    output_stream_handle: Arc<rodio::OutputStreamHandle>,
    source: rodio::source::Buffered<rodio::Decoder<std::io::Cursor<Vec<u8>>>>,
    pub looped: bool,
}

impl Sound {
    pub(crate) fn new(geng: &Geng, data: Vec<u8>) -> Self {
        Self {
            output_stream_handle: geng.inner.audio.output_stream_handle.clone(),
            geng: geng.clone(),
            source: rodio::Source::buffered(
                rodio::Decoder::new(std::io::Cursor::new(data)).expect("Failed to decode audio"),
            ),
            looped: false,
        }
    }
    pub fn effect(&self) -> SoundEffect {
        SoundEffect {
            geng: self.geng.clone(),
            sink: Some({
                let sink = rodio::Sink::try_new(&self.output_stream_handle).unwrap();
                sink.pause();
                if self.looped {
                    sink.append(rodio::Source::repeat_infinite(self.source.clone()));
                } else {
                    sink.append(self.source.clone());
                }
                sink
            }),
        }
    }
    pub fn play(&self) -> SoundEffect {
        let mut effect = self.effect();
        effect.play();
        effect
    }
}

pub struct SoundEffect {
    geng: Geng,
    sink: Option<rodio::Sink>,
}

impl SoundEffect {
    pub fn set_volume(&mut self, volume: f64) {
        self.sink().set_volume(volume as f32);
    }
    pub fn play(&mut self) {
        self.sink().play();
    }
    pub fn stop(mut self) {
        self.sink().stop();
    }
    pub fn pause(&mut self) {
        self.sink().pause();
    }
    fn sink(&mut self) -> &mut rodio::Sink {
        self.sink.as_mut().unwrap()
    }
    pub fn set_position(&mut self, position: Vec3<f64>) {
        // TODO
    }
    pub fn set_max_distance(&mut self, max_distance: f64) {
        // TODO
    }
}

impl Drop for SoundEffect {
    fn drop(&mut self) {
        let sink = self.sink.take().unwrap();
        if sink.volume() == 0.0 || sink.is_paused() {
            sink.stop();
        }
        sink.detach();
    }
}
