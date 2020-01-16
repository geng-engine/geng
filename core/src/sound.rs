use crate::*;

pub struct Sound {
    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    pub(crate) inner: stdweb::Reference,
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    pub(crate) data: Arc<[u8]>,
    pub looped: bool,
}

impl Sound {
    pub fn effect(&self) -> SoundEffect {
        SoundEffect {
            #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
            inner: stdweb::unstable::TryInto::try_into(js! {
                var effect = @{&self.inner}.cloneNode();
                effect.loop = @{self.looped};
                return effect;
            })
            .unwrap(),
            #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
            sink: Some({
                let sink = rodio::Sink::new(&rodio::default_output_device().unwrap());
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
    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    inner: stdweb::Reference,
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    sink: Option<rodio::Sink>,
}

impl SoundEffect {
    pub fn set_volume(&mut self, volume: f64) {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        js! {
            @(no_return)
            @{&self.inner}.volume = @{volume};
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        self.sink().set_volume(volume as f32);
    }
    pub fn play(&mut self) {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        js! {
            @(no_return)
            @{&self.inner}.play();
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        self.sink().play();
    }
    pub fn pause(&mut self) {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        js! {
            @(no_return)
            @{&self.inner}.pause();
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        self.sink().pause();
    }
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    fn sink(&mut self) -> &mut rodio::Sink {
        self.sink.as_mut().unwrap()
    }
}

impl Drop for SoundEffect {
    fn drop(&mut self) {
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        self.sink.take().unwrap().detach();
    }
}
