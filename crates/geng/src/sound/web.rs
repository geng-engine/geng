use super::*;

pub struct AudioContext {
    pub(crate) context: web_sys::AudioContext,
    master_gain_node: web_sys::GainNode,
}

impl AudioContext {
    pub(crate) fn new() -> Self {
        let context = web_sys::AudioContext::new().expect("Failed to initialize audio context");
        let master_gain_node = web_sys::GainNode::new(&context).unwrap();
        master_gain_node
            .connect_with_audio_node(&context.destination())
            .unwrap();
        Self {
            context,
            master_gain_node,
        }
    }

    pub fn set_listener_position(&self, pos: vec3<f64>) {
        self.context.listener().set_position(pos.x, pos.y, pos.z);
    }

    pub fn set_listener_orientation(&self, forward: vec3<f64>, up: vec3<f64>) {
        self.context
            .listener()
            .set_orientation(forward.x, forward.y, forward.z, up.x, up.y, up.z);
    }

    pub fn set_volume(&self, volume: f64) {
        self.master_gain_node.gain().set_value(volume as f32);
    }
}

enum SpatialState {
    NotSpatial(web_sys::AudioNode),
    Spatial(web_sys::PannerNode),
}

pub struct Sound {
    geng: Geng,
    inner: web_sys::AudioBuffer,
    pub looped: bool,
}

impl Sound {
    pub(crate) fn new(geng: &Geng, buffer: web_sys::AudioBuffer) -> Self {
        Self {
            geng: geng.clone(),
            inner: buffer,
            looped: false,
        }
    }
    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.inner.duration())
    }
    pub fn effect(&self) -> SoundEffect {
        let buffer_node =
            web_sys::AudioBufferSourceNode::new(&self.geng.inner.audio.context).unwrap();
        buffer_node.set_buffer(Some(&self.inner));
        buffer_node.set_loop(self.looped);
        let gain_node = web_sys::GainNode::new(&self.geng.inner.audio.context).unwrap();
        buffer_node.connect_with_audio_node(&gain_node).unwrap();
        let audio_node: web_sys::AudioNode = gain_node.clone().into();
        audio_node
            .connect_with_audio_node(&self.geng.inner.audio.master_gain_node)
            .unwrap();
        SoundEffect {
            geng: self.geng.clone(),
            inner: buffer_node,
            gain_node,
            spatial_state: SpatialState::NotSpatial(audio_node),
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
    inner: web_sys::AudioBufferSourceNode,
    gain_node: web_sys::GainNode,
    spatial_state: SpatialState,
}

impl SoundEffect {
    pub fn set_volume(&mut self, volume: f64) {
        self.gain_node.gain().set_value(volume as f32);
    }
    pub fn play(&mut self) {
        self.play_from(Duration::from_secs_f64(0.0));
    }
    pub fn play_from(&mut self, offset: Duration) {
        let _ = self
            .inner
            .start_with_when_and_grain_offset(0.0, offset.as_secs_f64())
            .unwrap();
    }
    pub fn set_speed(&mut self, speed: f64) {
        self.inner.playback_rate().set_value(speed as f32);
    }
    pub fn stop(&mut self) {
        self.inner.stop().unwrap();
    }
    // TODO
    // pub fn pause(&mut self) {
    //     self.inner.pause().unwrap();
    // }
    pub fn set_position(&mut self, position: vec3<f64>) {
        let panner_node = self.make_spatial();
        panner_node.set_position(position.x, position.y, position.z);
    }
    pub fn set_ref_distance(&mut self, distance: f64) {
        let panner_node = self.make_spatial();
        panner_node.set_ref_distance(distance);
    }
    pub fn set_max_distance(&mut self, max_distance: f64) {
        let panner_node = self.make_spatial();
        panner_node.set_max_distance(max_distance);
    }
    fn make_spatial(&mut self) -> &web_sys::PannerNode {
        if let SpatialState::NotSpatial(audio_node) = &self.spatial_state {
            let panner_node = web_sys::PannerNode::new(&self.geng.inner.audio.context).unwrap();
            panner_node.set_distance_model(web_sys::DistanceModelType::Linear);
            audio_node.disconnect().unwrap();
            audio_node
                .connect_with_audio_node(&panner_node)
                .unwrap()
                .connect_with_audio_node(&self.geng.inner.audio.master_gain_node)
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
    fn drop(&mut self) {}
}
