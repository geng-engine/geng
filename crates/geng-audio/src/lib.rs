use batbox_la::*;
use batbox_time as time;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use geng_web_audio_api as wa;

use wa::AudioNode as _;

#[derive(Clone)]
pub struct Audio {
    inner: Arc<AudioImpl>,
}

struct AudioImpl {
    context: wa::AudioContext,
    master_gain_node: wa::GainNode,
    default_type: SoundType,
    type_gain_nodes: Mutex<HashMap<SoundType, Arc<wa::GainNode>>>,
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
                default_type: SoundType::new(),
                type_gain_nodes: Mutex::new(HashMap::new()),
            }),
        })
    }

    pub fn listener(&self) -> Listener {
        Listener(self.inner.context.listener())
    }

    pub fn master_volume(&self) -> wa::AudioParam {
        self.inner.master_gain_node.gain()
    }

    pub fn volume(&self, r#type: SoundType) -> wa::AudioParam {
        self.register_type(r#type).gain()
    }

    pub fn default_type(&self) -> SoundType {
        self.inner.default_type
    }

    fn register_type(&self, r#type: SoundType) -> Arc<wa::GainNode> {
        self.inner
            .type_gain_nodes
            .lock()
            .unwrap()
            .entry(r#type)
            .or_insert_with(|| {
                let type_node = wa::GainNode::new(&self.inner.context);
                type_node.connect(&self.inner.master_gain_node);
                Arc::new(type_node)
            })
            .clone()
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
    pub fn effect(&self, r#type: SoundType) -> SoundEffect {
        let mut buffer_node = wa::AudioBufferSourceNode::new(&self.context.inner.context);
        buffer_node.set_buffer(self.audio_buffer.clone());
        buffer_node.set_loop(self.looped);
        let fade_node = wa::GainNode::new(&self.context.inner.context);
        let gain_node = wa::GainNode::new(&self.context.inner.context);
        buffer_node.connect(&fade_node).connect(&gain_node);
        // .connect(&self.context.inner.master_gain_node);
        // https://github.com/orottier/web-audio-api-rs/issues/494
        SoundEffect {
            r#type,
            context: self.context.clone(),
            source_node: buffer_node,
            fade_node,
            gain_node,
            fade_in_times: None,
            spatial_state: SpatialState::NotSpatial,
        }
    }
    pub fn play(&self) -> SoundEffect {
        let mut effect = self.effect(self.context.default_type());
        effect.play();
        effect
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SoundType(u64);

impl SoundType {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        SoundType(NEXT.fetch_add(1, Ordering::SeqCst))
    }
}

pub struct SoundEffect {
    context: Audio,
    r#type: SoundType,
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
                let duration = times.end - times.start;
                if duration.abs() < 1e-5 {
                    1.0
                } else {
                    ((current_time - times.start) / duration).clamp(0.0, 1.0) as f32
                }
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

    pub fn fade_to_volume(&mut self, volume: f32, duration: time::Duration) {
        let current_time = self.context.inner.context.current_time();
        let end_time = current_time + duration.as_secs_f64();
        let fade_gain = self.gain_node.gain();

        fade_gain.cancel_scheduled_changes(current_time);

        fade_gain.linear_ramp_to_value_at_time(volume, end_time);
    }

    pub fn play(&mut self) {
        self.play_from(time::Duration::from_secs_f64(0.0));
    }

    pub fn play_from(&mut self, offset: time::Duration) {
        let node: &dyn wa::AudioNode = match &self.spatial_state {
            SpatialState::NotSpatial => &self.gain_node,
            SpatialState::Spatial(panner) => panner,
        };
        node.connect(&*self.context.register_type(self.r#type));
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
            self.gain_node.connect(&panner_node);
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
