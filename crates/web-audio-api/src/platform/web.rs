pub struct AudioContext(web_sys::AudioContext);

pub type AudioNodeRef<'a> = &'a web_sys::AudioNode;

pub fn connect(from: AudioNodeRef<'_>, to: AudioNodeRef<'_>) {
    from.connect_with_audio_node(to).unwrap();
}

/// Disconnects all outgoing connections from the AudioNode.
pub fn disconnect(node: AudioNodeRef<'_>) {
    node.disconnect().unwrap();
}

impl AudioContext {
    pub fn new() -> Self {
        Self(web_sys::AudioContext::new().unwrap())
    }

    pub fn destination(&self) -> AudioDestinationNode {
        AudioDestinationNode(self.0.destination())
    }

    pub fn listener(&self) -> AudioListener {
        AudioListener(self.0.listener())
    }

    pub async fn decode(&self, data: Vec<u8>) -> anyhow::Result<AudioBuffer> {
        let arraybuffer = js_sys::Uint8Array::from(data.as_slice()).buffer(); // TODO hmm
        let Ok(promise) = self.0.decode_audio_data(&arraybuffer) else {
            anyhow::bail!("whoops"); // TODO
        };
        let Ok(buffer) = wasm_bindgen_futures::JsFuture::from(promise).await else {
            anyhow::bail!("whoops"); // TODO
        };
        Ok(AudioBuffer(buffer.into()))
    }

    pub fn current_time(&self) -> f64 {
        self.0.current_time()
    }
}

pub struct AudioListener(web_sys::AudioListener);

impl AudioListener {
    pub fn set_position(&self, [x, y, z]: [f32; 3]) {
        self.0.set_position(x as f64, y as f64, z as f64);
    }
    pub fn set_orientation(
        &self,
        [forward_x, forward_y, forward_z]: [f32; 3],
        [up_x, up_y, up_z]: [f32; 3],
    ) {
        self.0.set_orientation(
            forward_x as f64,
            forward_y as f64,
            forward_z as f64,
            up_x as f64,
            up_y as f64,
            up_z as f64,
        );
    }
}

pub struct AudioDestinationNode(web_sys::AudioDestinationNode);

impl AudioDestinationNode {
    pub fn get_ref(&self) -> AudioNodeRef {
        &self.0
    }
}

pub struct AudioParam(web_sys::AudioParam);

impl AudioParam {
    pub fn set_value(&self, value: f32) {
        self.0.set_value(value);
    }

    pub fn linear_ramp_to_value_at_time(&self, value: f32, end_time: f64) {
        self.0.linear_ramp_to_value_at_time(value, end_time);
    }

    pub fn exponential_ramp_to_value_at_time(&self, value: f32, end_time: f64) {
        self.0.exponential_ramp_to_value_at_time(value, end_time);
    }
}

pub struct GainNode(web_sys::GainNode);

impl GainNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(web_sys::GainNode::new(&context.0).unwrap())
    }
    pub fn gain(&self) -> AudioParam {
        AudioParam(self.0.gain())
    }
    pub fn get_ref(&self) -> AudioNodeRef<'_> {
        &self.0
    }
}

pub struct PannerNode(web_sys::PannerNode);

impl PannerNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(web_sys::PannerNode::new(&context.0).unwrap())
    }
    pub fn get_ref(&self) -> AudioNodeRef<'_> {
        &self.0
    }

    pub fn set_distance_model(&mut self, model: crate::DistanceModel) {
        self.0.set_distance_model(match model {
            crate::DistanceModel::Linear => web_sys::DistanceModelType::Linear,
            crate::DistanceModel::Inverse => web_sys::DistanceModelType::Inverse,
            crate::DistanceModel::Exponential => web_sys::DistanceModelType::Exponential,
        });
    }

    pub fn set_position(&mut self, [x, y, z]: [f32; 3]) {
        self.0.set_position(x as f64, y as f64, z as f64);
    }

    pub fn set_ref_distance(&mut self, ref_distance: f64) {
        self.0.set_ref_distance(ref_distance);
    }

    pub fn set_max_distance(&mut self, max_distance: f64) {
        self.0.set_max_distance(max_distance);
    }
}

#[derive(Clone)]
pub struct AudioBuffer(web_sys::AudioBuffer);

impl AudioBuffer {
    pub fn duration(&self) -> f64 {
        self.0.duration()
    }
}

pub struct AudioBufferSourceNode(web_sys::AudioBufferSourceNode);

impl AudioBufferSourceNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(web_sys::AudioBufferSourceNode::new(&context.0).unwrap())
    }

    pub fn get_ref(&self) -> AudioNodeRef<'_> {
        &self.0
    }

    pub fn start_with_offset(&mut self, offset: f64) {
        self.0
            .start_with_when_and_grain_offset(self.0.context().current_time(), offset)
            .unwrap();
    }

    pub fn stop(&mut self) {
        self.0.stop().unwrap();
    }

    pub fn stop_at(&mut self, when: f64) {
        self.0.stop_with_when(when).unwrap();
    }

    pub fn set_loop(&mut self, looped: bool) {
        self.0.set_loop(looped);
    }

    pub fn playback_rate(&self) -> AudioParam {
        AudioParam(self.0.playback_rate().clone())
    }

    pub fn set_buffer(&mut self, buffer: AudioBuffer) {
        self.0.set_buffer(Some(&buffer.0));
    }
}
