use super::*;

pub struct AudioContext {
    // #[cfg(not(target_arch = "wasm32"))]
    // pub(crate) output_stream: rodio::OutputStream,
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) output_stream_handle: Arc<rodio::OutputStreamHandle>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) context: web_sys::AudioContext,
}

impl AudioContext {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn new() -> Self {
        Self {
            context: web_sys::AudioContext::new().expect("Failed to initialize audio context"),
        }
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

    pub fn set_listener_position(&self, pos: Vec3<f64>) {
        #[cfg(target_arch = "wasm32")]
        {
            self.context.listener().set_position(pos.x, pos.y, pos.z);
        }
    }

    pub fn set_listener_orientation(&self, forward: Vec3<f64>, up: Vec3<f64>) {
        #[cfg(target_arch = "wasm32")]
        {
            self.context
                .listener()
                .set_orientation(forward.x, forward.y, forward.z, up.x, up.y, up.z);
        }
    }
}

#[cfg(target_arch = "wasm32")]
enum SpatialState {
    NotSpatial(web_sys::MediaElementAudioSourceNode),
    Spatial(web_sys::PannerNode),
}

pub struct Sound {
    pub(crate) geng: Geng,
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
        #[cfg(target_arch = "wasm32")]
        let (audio_node, effect) = {
            let effect = self
                .inner
                .clone_node()
                .unwrap()
                .dyn_into::<web_sys::HtmlAudioElement>()
                .unwrap();
            let audio_node = self
                .geng
                .inner
                .audio
                .context
                .create_media_element_source(&effect)
                .unwrap();
            audio_node
                .connect_with_audio_node(&self.geng.inner.audio.context.destination())
                .unwrap();
            effect.set_loop(self.looped);
            (audio_node, effect)
        };
        SoundEffect {
            geng: self.geng.clone(),
            #[cfg(target_arch = "wasm32")]
            inner: effect,
            #[cfg(target_arch = "wasm32")]
            spatial_state: SpatialState::NotSpatial(audio_node),
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
    geng: Geng,
    #[cfg(target_arch = "wasm32")]
    inner: web_sys::HtmlAudioElement,
    #[cfg(target_arch = "wasm32")]
    spatial_state: SpatialState,
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
    pub fn set_position(&mut self, position: Vec3<f64>) {
        #[cfg(target_arch = "wasm32")]
        {
            let panner_node = self.make_spatial();
            panner_node.set_position(position.x, position.y, position.z);
        }
    }
    pub fn set_max_distance(&mut self, max_distance: f64) {
        #[cfg(target_arch = "wasm32")]
        {
            let panner_node = self.make_spatial();
            panner_node.set_max_distance(max_distance);
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn make_spatial(&mut self) -> &web_sys::PannerNode {
        if let SpatialState::NotSpatial(audio_node) = &self.spatial_state {
            let panner_node = web_sys::PannerNode::new(&self.geng.inner.audio.context).unwrap();
            panner_node.set_distance_model(web_sys::DistanceModelType::Linear);
            audio_node.disconnect().unwrap();
            audio_node
                .connect_with_audio_node(&panner_node)
                .unwrap()
                .connect_with_audio_node(&self.geng.inner.audio.context.destination())
                .unwrap();
            self.spatial_state = SpatialState::Spatial(panner_node);
        }
        let SpatialState::Spatial(panner_node) = &self.spatial_state else {
            unreachable!()
        };
        panner_node
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
