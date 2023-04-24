use batbox_la::*;
use batbox_time as time;
use std::rc::Rc;

mod platform;

#[derive(Clone)]
pub struct Audio {
    inner: Rc<platform::Context>,
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

impl Audio {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(platform::Context::new()),
        }
    }
    pub fn set_volume(&self, volume: f64) {
        self.inner.set_volume(volume);
    }
    pub fn set_listener_position(&self, position: vec3<f64>) {
        self.inner.set_listener_position(position);
    }
    pub fn set_listener_orientation(&self, forward: vec3<f64>, up: vec3<f64>) {
        self.inner.set_listener_orientation(forward, up);
    }
    pub async fn load(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<Sound> {
        Ok(Sound {
            inner: platform::Sound::load(&self.inner, path.as_ref()).await?,
        })
    }
    pub async fn decode_bytes(&self, data: Vec<u8>) -> anyhow::Result<Sound> {
        Ok(Sound {
            inner: platform::Sound::decode_bytes(&self.inner, data).await?,
        })
    }
}

pub struct Sound {
    inner: platform::Sound,
}

impl Sound {
    pub fn looped(&self) -> bool {
        self.inner.looped
    }
    pub fn set_looped(&mut self, looped: bool) {
        self.inner.looped = looped;
    }
    pub fn duration(&self) -> time::Duration {
        self.inner.duration()
    }
    pub fn effect(&self) -> SoundEffect {
        SoundEffect {
            inner: self.inner.effect(),
        }
    }
    pub fn play(&self) -> SoundEffect {
        let mut effect = self.effect();
        effect.play();
        effect
    }
}

pub struct SoundEffect {
    inner: platform::SoundEffect,
}

impl SoundEffect {
    pub fn play(&mut self) {
        self.inner.play();
    }
    pub fn play_from(&mut self, offset: time::Duration) {
        self.inner.play_from(offset);
    }
    pub fn stop(&mut self) {
        self.inner.stop();
    }
    pub fn set_volume(&mut self, volume: f64) {
        self.inner.set_volume(volume);
    }
    pub fn set_speed(&mut self, speed: f64) {
        self.inner.set_speed(speed);
    }
    pub fn set_position(&mut self, position: vec3<f64>) {
        self.inner.set_position(position);
    }
    pub fn set_ref_distance(&mut self, ref_distance: f64) {
        self.inner.set_ref_distance(ref_distance);
    }
    pub fn set_max_distance(&mut self, max_distance: f64) {
        self.inner.set_max_distance(max_distance);
    }
}
