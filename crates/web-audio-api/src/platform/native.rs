use std::sync::Arc;

use web_audio_api::{
    context::BaseAudioContext,
    node::{AudioNode, AudioScheduledSourceNode},
};

async fn spawn_blocking<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> Result<T, futures::channel::oneshot::Canceled> {
    let (sender, receiver) = futures::channel::oneshot::channel();
    std::thread::spawn(move || {
        let result = f();
        if let Err(_value) = sender.send(result) {
            // receiver was dropped
        }
    });
    receiver.await
}

pub struct AudioContext(Arc<web_audio_api::context::AudioContext>);

pub type AudioNodeRef<'a> = &'a dyn web_audio_api::node::AudioNode;

pub fn connect(from: AudioNodeRef<'_>, to: AudioNodeRef<'_>) {
    from.connect(to);
}

/// Disconnects all outgoing connections from the AudioNode.
pub fn disconnect(node: AudioNodeRef<'_>) {
    node.disconnect();
}

impl AudioContext {
    pub fn new() -> Self {
        Self(Arc::new(web_audio_api::context::AudioContext::new(
            web_audio_api::context::AudioContextOptions {
                latency_hint: if cfg!(target_os = "linux") {
                    // See notes on Linux in web_audio_api crate
                    web_audio_api::context::AudioContextLatencyCategory::Playback
                } else {
                    web_audio_api::context::AudioContextLatencyCategory::Interactive
                },
                ..Default::default()
            },
        )))
    }

    pub fn destination(&self) -> AudioDestinationNode {
        AudioDestinationNode(self.0.destination())
    }

    pub fn listener(&self) -> AudioListener {
        AudioListener(self.0.listener())
    }

    pub async fn decode(&self, data: Vec<u8>) -> anyhow::Result<AudioBuffer> {
        let inner = self.0.clone();
        let reader = std::io::Cursor::new(data);
        Ok(AudioBuffer(Arc::new(
            spawn_blocking(move || inner.decode_audio_data_sync(reader))
                .await?
                .map_err(|e| anyhow::anyhow!(e))?,
        )))
    }

    pub fn current_time(&self) -> f64 {
        self.0.current_time()
    }
}

pub struct AudioListener(web_audio_api::AudioListener);

impl AudioListener {
    pub fn set_position(&self, [x, y, z]: [f32; 3]) {
        self.0.position_x().set_value(x);
        self.0.position_y().set_value(y);
        self.0.position_z().set_value(z);
    }
    pub fn set_orientation(
        &self,
        [forward_x, forward_y, forward_z]: [f32; 3],
        [up_x, up_y, up_z]: [f32; 3],
    ) {
        self.0.forward_x().set_value(forward_x);
        self.0.forward_y().set_value(forward_y);
        self.0.forward_z().set_value(forward_z);
        self.0.up_x().set_value(up_x);
        self.0.up_y().set_value(up_y);
        self.0.up_z().set_value(up_z);
    }
}

pub struct AudioDestinationNode(web_audio_api::node::AudioDestinationNode);

impl AudioDestinationNode {
    pub fn get_ref(&self) -> AudioNodeRef {
        &self.0
    }
}

pub struct AudioParam(web_audio_api::AudioParam);

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
    pub fn cancel_scheduled_changes(&self, cancel_time: f64) {
        self.0.cancel_scheduled_values(cancel_time);
    }
    pub fn value(&self) -> f32 {
        self.0.value()
    }
}

impl From<&web_audio_api::AudioParam> for AudioParam {
    fn from(param: &web_audio_api::AudioParam) -> Self {
        Self(param.clone())
    }
}

pub struct GainNode(web_audio_api::node::GainNode);

impl GainNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(web_audio_api::node::GainNode::new(
            &*context.0,
            web_audio_api::node::GainOptions::default(),
        ))
    }
    pub fn gain(&self) -> AudioParam {
        self.0.gain().into()
    }
    pub fn get_ref(&self) -> AudioNodeRef<'_> {
        &self.0
    }
}

pub struct PannerNode(web_audio_api::node::PannerNode);

impl PannerNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(web_audio_api::node::PannerNode::new(
            &*context.0,
            web_audio_api::node::PannerOptions::default(),
        ))
    }
    pub fn get_ref(&self) -> AudioNodeRef<'_> {
        &self.0
    }

    pub fn set_distance_model(&mut self, model: crate::DistanceModel) {
        self.0.set_distance_model(match model {
            crate::DistanceModel::Linear => web_audio_api::node::DistanceModelType::Linear,
            crate::DistanceModel::Inverse => web_audio_api::node::DistanceModelType::Inverse,
            crate::DistanceModel::Exponential => {
                web_audio_api::node::DistanceModelType::Exponential
            }
        });
    }

    pub fn set_position(&mut self, [x, y, z]: [f32; 3]) {
        self.0.set_position(x, y, z);
    }

    pub fn set_ref_distance(&mut self, ref_distance: f64) {
        self.0.set_ref_distance(ref_distance);
    }

    pub fn set_max_distance(&mut self, max_distance: f64) {
        self.0.set_max_distance(max_distance);
    }
}

#[derive(Clone)]
pub struct AudioBuffer(Arc<web_audio_api::AudioBuffer>);

impl AudioBuffer {
    pub fn duration(&self) -> f64 {
        self.0.duration()
    }
}

pub struct AudioBufferSourceNode(web_audio_api::node::AudioBufferSourceNode);

impl AudioBufferSourceNode {
    pub fn new(context: &AudioContext) -> Self {
        Self(web_audio_api::node::AudioBufferSourceNode::new(
            &*context.0,
            web_audio_api::node::AudioBufferSourceOptions::default(),
        ))
    }

    pub fn get_ref(&self) -> AudioNodeRef<'_> {
        &self.0
    }

    pub fn start_with_offset(&mut self, offset: f64) {
        self.0
            .start_at_with_offset(self.0.context().current_time(), offset);
    }

    pub fn stop(&mut self) {
        self.0.stop();
    }

    pub fn stop_at(&mut self, when: f64) {
        self.0.stop_at(when);
    }

    pub fn set_loop(&mut self, looped: bool) {
        self.0.set_loop(looped);
    }

    pub fn playback_rate(&self) -> AudioParam {
        AudioParam(self.0.playback_rate().clone())
    }

    pub fn set_buffer(&mut self, buffer: AudioBuffer) {
        fn arc_unwrap_or_clone<T: Clone>(arc: Arc<T>) -> T {
            // TODO: wait for stable: https://github.com/rust-lang/rust/issues/93610
            match Arc::try_unwrap(arc) {
                Ok(value) => value,
                Err(arc) => (*arc).clone(),
            }
        }
        self.0.set_buffer(arc_unwrap_or_clone(buffer.0));
    }

    pub fn position(&self) -> f64 {
        self.0.position()
    }
}
