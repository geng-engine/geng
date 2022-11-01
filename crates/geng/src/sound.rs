use super::*;

pub(crate) struct AudioContext {
    // #[cfg(not(target_arch = "wasm32"))]
    // pub(crate) output_stream: rodio::OutputStream,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) output_stream_handle: Arc<rodio::OutputStreamHandle>,
}

impl AudioContext {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn new() -> Self {
        Self {}
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn new() -> Self {
        {
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
    }
}

pub struct Sound {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) output_stream_handle: Arc<rodio::OutputStreamHandle>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) inner: web_sys::HtmlAudioElement,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) data: Arc<[u8]>,
    pub looped: bool,
}

impl Sound {
    pub fn effect(&self) -> SoundEffect {
        SoundEffect {
            #[cfg(target_arch = "wasm32")]
            inner: {
                let effect = self
                    .inner
                    .clone_node()
                    .unwrap()
                    .dyn_into::<web_sys::HtmlAudioElement>()
                    .unwrap();
                effect.set_loop(self.looped);
                effect
            },
            #[cfg(not(target_arch = "wasm32"))]
            sink: Some({
                let sink = rodio::Sink::try_new(&self.output_stream_handle).unwrap();
                sink.pause();
                if self.looped {
                    sink.append(rodio::Source::repeat_infinite(
                        rodio::Decoder::new(std::io::Cursor::new(self.data.clone())).unwrap(),
                    ));
                } else {
                    sink.append(
                        rodio::Decoder::new(std::io::Cursor::new(self.data.clone())).unwrap(),
                    );
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
    #[cfg(target_arch = "wasm32")]
    inner: web_sys::HtmlAudioElement,
    #[cfg(not(target_arch = "wasm32"))]
    sink: Option<rodio::Sink>,
}

impl SoundEffect {
    pub fn set_volume(&mut self, volume: f64) {
        #[cfg(target_arch = "wasm32")]
        self.inner.set_volume(volume);
        #[cfg(not(target_arch = "wasm32"))]
        self.sink().set_volume(volume as f32);
    }
    pub fn play(&mut self) {
        #[cfg(target_arch = "wasm32")]
        let _ = self.inner.play().unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        self.sink().play();
    }
    pub fn stop(mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        self.sink().stop();
    }
    pub fn pause(&mut self) {
        #[cfg(target_arch = "wasm32")]
        self.inner.pause().unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        self.sink().pause();
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn sink(&mut self) -> &mut rodio::Sink {
        self.sink.as_mut().unwrap()
    }
}

impl Drop for SoundEffect {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let sink = self.sink.take().unwrap();
            if sink.volume() == 0.0 || sink.is_paused() {
                sink.stop();
            }
            sink.detach();
        }
    }
}
