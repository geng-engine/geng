use batbox_la::*;
use batbox_time as time;
use std::path::Path;
use std::sync::Arc;

use geng_web_audio_api as wa;

use wa::AudioNode as _;

#[derive(Clone)]
pub struct Audio {
    inner: Arc<AudioImpl>,
}

struct AudioImpl {
    context: wa::AudioContext,
    master_gain_node: wa::GainNode,
}

impl Audio {
    pub fn new() -> anyhow::Result<Self> {
        let context = wa::AudioContext::new()?;
        let master_gain_node = wa::GainNode::new(&context);
        master_gain_node.connect(&context.destination());
        Ok(Self {
            inner: Arc::new(AudioImpl {
                context,
                master_gain_node,
            }),
        })
    }

    pub fn listener(&self) -> Listener {
        Listener(self.inner.context.listener())
    }

    pub fn set_volume(&self, volume: f32) {
        self.inner.master_gain_node.gain().set_value(volume);
    }

    pub fn volume(&self) -> f32 {
        self.inner.master_gain_node.gain().value()
    }
}

pub struct Listener(wa::AudioListener);

impl Listener {
    pub fn set_position(&self, pos: vec3<f32>) {
        self.0.set_position(**pos);
    }

    pub fn set_orientation(&self, forward: vec3<f32>, up: vec3<f32>) {
        self.0.set_orientation(**forward, **up);
    }
}

enum SpatialState {
    NotSpatial,
    Spatial(wa::PannerNode),
}

pub struct Sound {
    context: Audio,
    audio_buffer: wa::AudioBuffer,
    pub looped: bool, // TODO move to .effect() arg, loop_start, loop_end
}

impl Audio {
    pub async fn load(&self, path: impl AsRef<Path>) -> anyhow::Result<Sound> {
        let data = batbox_file::load_bytes(path).await?;
        self.decode(data).await
    }
    pub async fn decode(&self, data: Vec<u8>) -> anyhow::Result<Sound> {
        let inner = self.inner.context.decode(data).await?;
        Ok(Sound {
            context: self.clone(),
            audio_buffer: inner,
            looped: false,
        })
    }
}

impl Sound {
    pub fn duration(&self) -> time::Duration {
        time::Duration::from_secs_f64(self.audio_buffer.duration())
    }
    pub fn effect(&self) -> SoundEffect {
        let mut buffer_node = wa::AudioBufferSourceNode::new(&self.context.inner.context);
        buffer_node.set_buffer(self.audio_buffer.clone());
        buffer_node.set_loop(self.looped);
        let fade_node = wa::GainNode::new(&self.context.inner.context);
        let gain_node = wa::GainNode::new(&self.context.inner.context);
        buffer_node.connect(&fade_node).connect(&gain_node);
        // .connect(&self.context.inner.master_gain_node);
        // https://github.com/orottier/web-audio-api-rs/issues/494
        SoundEffect {
            context: self.context.clone(),
            source_node: buffer_node,
            fade_node,
            gain_node,
            fade_in_times: None,
            spatial_state: SpatialState::NotSpatial,
        }
    }
    pub fn play(&self) -> SoundEffect {
        let mut effect = self.effect();
        effect.play();
        effect
    }
}

pub struct SoundEffect {
    context: Audio,
    source_node: wa::AudioBufferSourceNode,
    gain_node: wa::GainNode,
    fade_node: wa::GainNode,
    fade_in_times: Option<std::ops::Range<f64>>,
    spatial_state: SpatialState,
}

impl SoundEffect {
    pub fn set_looped(&mut self, looped: bool) {
        self.source_node.set_loop(looped);
    }
    pub fn fade_in(&mut self, duration: time::Duration) {
        let current_time = self.context.inner.context.current_time();
        let end_time = current_time + duration.as_secs_f64();
        let fade_gain = self.fade_node.gain();
        fade_gain.cancel_scheduled_changes(current_time);

        // fade_gain.set_value(0.0);
        // workaround for https://bugzilla.mozilla.org/show_bug.cgi?id=1171438
        fade_gain.linear_ramp_to_value_at_time(0.0, current_time);

        fade_gain.linear_ramp_to_value_at_time(1.0, end_time);

        self.fade_in_times = Some(current_time..end_time);
    }
    pub fn fade_out(&mut self, duration: time::Duration) {
        let current_time = self.context.inner.context.current_time();
        let current_value = match self.fade_in_times.take() {
            Some(times) => {
                ((current_time - times.start) / (times.end - times.start)).clamp(0.0, 1.0) as f32
            }
            None => 1.0,
        };
        // actual fade out duration will be shorter if fade_out is called shortly after fade_in
        let fade_out_duration = duration.as_secs_f64() * current_value as f64;
        let end_time = current_time + fade_out_duration;
        let fade_gain = self.fade_node.gain();
        fade_gain.cancel_scheduled_changes(current_time);

        // Like in fade in, working around a bug
        fade_gain.linear_ramp_to_value_at_time(current_value, current_time);

        fade_gain.linear_ramp_to_value_at_time(0.0, end_time);
        self.source_node.stop_at(end_time);
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.gain_node.gain().set_value(volume);
    }
    pub fn play(&mut self) {
        self.play_from(time::Duration::from_secs_f64(0.0));
    }
    pub fn play_from(&mut self, offset: time::Duration) {
        if let SpatialState::NotSpatial = self.spatial_state {
            self.gain_node.connect(&self.context.inner.master_gain_node);
        }
        self.source_node.start_with_offset(offset.as_secs_f64());
    }
    pub fn set_speed(&mut self, speed: f32) {
        self.source_node.playback_rate().set_value(speed);
    }
    pub fn stop(&mut self) {
        self.source_node.stop();
    }
    pub fn set_position(&mut self, position: vec3<f32>) {
        let panner_node = self.make_spatial();
        panner_node.set_position(**position);
    }
    pub fn set_ref_distance(&mut self, distance: f32) {
        let panner_node = self.make_spatial();
        panner_node.set_ref_distance(distance as f64);
    }
    pub fn set_max_distance(&mut self, max_distance: f32) {
        let panner_node = self.make_spatial();
        panner_node.set_max_distance(max_distance as f64);
    }
    fn make_spatial(&mut self) -> &mut wa::PannerNode {
        if let SpatialState::NotSpatial = &self.spatial_state {
            let mut panner_node = wa::PannerNode::new(&self.context.inner.context);
            panner_node.set_distance_model(wa::DistanceModel::Linear);
            // self.gain_node.disconnect();
            self.gain_node
                .connect(&panner_node)
                .connect(&self.context.inner.master_gain_node);
            self.spatial_state = SpatialState::Spatial(panner_node);
        }
        let SpatialState::Spatial(panner_node) = &mut self.spatial_state else {
            unreachable!()
        };
        panner_node
    }

    pub fn playback_position(&self) -> time::Duration {
        time::Duration::from_secs_f64(self.source_node.position())
    }
}

impl Drop for SoundEffect {
    fn drop(&mut self) {}
}
