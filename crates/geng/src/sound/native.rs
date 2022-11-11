use super::*;

const EAR_OFFSET: f32 = 3.0;

struct Listener {
    pos: Vec3<f32>,
    forward: Vec3<f32>, // should be normalized
    up: Vec3<f32>,      // and orthogonal
    left_ear: Vec3<f32>,
    right_ear: Vec3<f32>,
}

impl Listener {
    fn update_ears(&mut self) {
        let v = Vec3::cross(self.forward, self.up).normalize_or_zero() * EAR_OFFSET;
        self.left_ear = self.pos - v;
        self.right_ear = self.pos + v;
    }
}

pub struct AudioContext {
    // output_stream: rodio::OutputStream,
    output_stream_handle: Arc<rodio::OutputStreamHandle>,
    listener: Arc<Mutex<Listener>>,
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
            listener: Arc::new(Mutex::new({
                let mut listener = Listener {
                    pos: Vec3::ZERO,
                    forward: vec3(0.0, -1.0, 0.0),
                    up: vec3(0.0, 0.0, 1.0),
                    left_ear: Vec3::ZERO,
                    right_ear: Vec3::ZERO,
                };
                listener.update_ears();
                listener
            })),
        }
    }

    pub fn set_listener_position(&self, pos: Vec3<f64>) {
        let mut listener = self.listener.lock().unwrap();
        listener.pos = pos.map(|x| x as f32);
        listener.update_ears();
    }

    pub fn set_listener_orientation(&self, forward: Vec3<f64>, up: Vec3<f64>) {
        let mut listener = self.listener.lock().unwrap();
        listener.forward = forward.map(|x| x as f32);
        listener.up = up.map(|x| x as f32);
        listener.update_ears();
    }
}

pub struct Sound {
    geng: Geng,
    source: rodio::source::Buffered<rodio::Decoder<std::io::Cursor<Vec<u8>>>>,
    pub looped: bool,
}

impl Sound {
    pub(crate) fn new(geng: &Geng, data: Vec<u8>) -> Self {
        Self {
            geng: geng.clone(),
            source: rodio::Source::buffered(
                rodio::Decoder::new(std::io::Cursor::new(data)).expect("Failed to decode audio"),
            ),
            looped: false,
        }
    }
    pub fn effect(&self) -> SoundEffect {
        let spatial_params = Arc::new(Mutex::new(None));
        SoundEffect {
            geng: self.geng.clone(),
            sink: Some({
                let sink = rodio::Sink::try_new(&self.geng.audio().output_stream_handle).unwrap();
                sink.pause();
                if self.looped {
                    sink.append(Source::new(
                        spatial_params.clone(),
                        self.geng.audio().listener.clone(),
                        &rodio::Source::repeat_infinite(self.source.clone()),
                    ));
                } else {
                    sink.append(Source::new(
                        spatial_params.clone(),
                        self.geng.audio().listener.clone(),
                        &self.source,
                    ));
                }
                sink
            }),
            spatial_params,
        }
    }
    pub fn play(&self) -> SoundEffect {
        let mut effect = self.effect();
        effect.play();
        effect
    }
}

struct SpatialParams {
    pos: Vec3<f32>,
    ref_dist: f32,
    max_dist: f32,
}

impl Default for SpatialParams {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            ref_dist: 1.0, // https://webaudio.github.io/web-audio-api/#dom-pannernode-refdistance
            max_dist: 10000.0, // https://webaudio.github.io/web-audio-api/#dom-pannernode-maxdistance
        }
    }
}

struct Source<I>
where
    I: rodio::Source,
    I::Item: rodio::Sample,
{
    spatial: rodio::source::ChannelVolume<I>,
    regular: I,
    is_spatial: bool,
    spatial_params: Arc<Mutex<Option<SpatialParams>>>,
    listener: Arc<Mutex<Listener>>,
}

impl<I> Source<I>
where
    I: rodio::Source,
    I::Item: rodio::Sample,
{
    fn new(
        spatial_params: Arc<Mutex<Option<SpatialParams>>>,
        listener: Arc<Mutex<Listener>>,
        source: &I,
    ) -> Self
    where
        I: Clone,
    {
        Self {
            spatial: rodio::source::ChannelVolume::new(source.clone(), vec![0.0, 0.0]),
            regular: source.clone(),
            is_spatial: false,
            spatial_params,
            listener,
        }
    }
    fn update_volume(&mut self) {
        if let Some(spatial_params) = &*self.spatial_params.lock().unwrap() {
            self.is_spatial = true;
            let listener = self.listener.lock().unwrap();

            // https://dvcs.w3.org/hg/audio/raw-file/tip/webaudio/specification.html#Spatialization-panning-algorithm
            let delta_pos = spatial_params.pos - listener.pos;
            let listener_right = Vec3::cross(listener.forward, listener.up);

            let plane_delta_pos = delta_pos - listener.up * Vec3::dot(delta_pos, listener.up);

            // kind of hack to skip atan, approximately correct
            let gain_right =
                (Vec3::dot(listener_right, plane_delta_pos.normalize_or_zero()) + 1.0) / 2.0;
            let gain_right = gain_right.clamp(0.0, 1.0); // just in case
            let gain_left = (1.0 - gain_right.sqr()).sqrt();

            let distance_gain = (1.0
                - (delta_pos.len() - spatial_params.ref_dist)
                    / (spatial_params.max_dist - spatial_params.ref_dist))
                .clamp(0.0, 1.0);

            self.spatial.set_volume(0, gain_left * distance_gain);
            self.spatial.set_volume(1, gain_right * distance_gain);
        } else {
            self.is_spatial = false;
        }
    }
}

impl<I> Iterator for Source<I>
where
    I: rodio::Source,
    I::Item: rodio::Sample,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.update_volume(); // TODO: not every time?
        if self.is_spatial {
            self.spatial.next()
        } else {
            self.regular.next()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_spatial {
            self.spatial.size_hint()
        } else {
            self.regular.size_hint()
        }
    }
}

impl<I> ExactSizeIterator for Source<I>
where
    I: rodio::Source + ExactSizeIterator,
    I::Item: rodio::Sample,
{
}

impl<I> rodio::Source for Source<I>
where
    I: rodio::Source,
    I::Item: rodio::Sample,
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        if self.is_spatial {
            self.spatial.current_frame_len()
        } else {
            self.regular.current_frame_len()
        }
    }

    #[inline]
    fn channels(&self) -> u16 {
        if self.is_spatial {
            self.spatial.channels()
        } else {
            self.regular.channels()
        }
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        if self.is_spatial {
            self.spatial.sample_rate()
        } else {
            self.regular.sample_rate()
        }
    }

    #[inline]
    fn total_duration(&self) -> Option<std::time::Duration> {
        if self.is_spatial {
            self.spatial.total_duration()
        } else {
            self.regular.total_duration()
        }
    }
}

pub struct SoundEffect {
    geng: Geng,
    spatial_params: Arc<Mutex<Option<SpatialParams>>>,
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
        self.make_spatial(|spatial| spatial.pos = position.map(|x| x as f32));
    }
    pub fn set_max_distance(&mut self, max_distance: f64) {
        self.make_spatial(|spatial| spatial.max_dist = max_distance as f32);
    }
    fn make_spatial(&mut self, f: impl FnOnce(&mut SpatialParams)) {
        let mut spatial = self.spatial_params.lock().unwrap();
        if spatial.is_none() {
            *spatial = Some(default());
        }
        f(spatial.as_mut().unwrap())
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
