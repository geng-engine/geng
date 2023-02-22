use super::*;

const EAR_OFFSET: f32 = 3.0;

struct Listener {
    pos: vec3<f32>,
    forward: vec3<f32>, // should be normalized
    up: vec3<f32>,      // and orthogonal
    left_ear: vec3<f32>,
    right_ear: vec3<f32>,
}

impl Listener {
    fn update_ears(&mut self) {
        let v = vec3::cross(self.forward, self.up).normalize_or_zero() * EAR_OFFSET;
        self.left_ear = self.pos - v;
        self.right_ear = self.pos + v;
    }
}

pub struct AudioContext {
    config: rodio::SupportedStreamConfig,
    // output_stream: rodio::OutputStream,
    output_stream_handle: Arc<rodio::OutputStreamHandle>,
    listener: Arc<Mutex<Listener>>,
    volume: Arc<atomic_float::AtomicF32>,
}

impl AudioContext {
    pub(crate) fn new() -> Self {
        fn create_rodio_output_stream() -> Result<
            (
                rodio::SupportedStreamConfig,
                rodio::OutputStream,
                rodio::OutputStreamHandle,
            ),
            rodio::StreamError,
        > {
            fn try_from_device(
                device: &rodio::Device,
            ) -> Result<
                (
                    rodio::SupportedStreamConfig,
                    rodio::OutputStream,
                    rodio::OutputStreamHandle,
                ),
                rodio::StreamError,
            > {
                use rodio::cpal::traits::DeviceTrait;
                let default_config = device.default_output_config()?;
                rodio::OutputStream::try_from_device_config(device, default_config.clone())
                    .map(move |(stream, handle)| (default_config, stream, handle))
            }

            use rodio::cpal::traits::HostTrait;
            let default_device = rodio::cpal::default_host()
                .default_output_device()
                .ok_or(rodio::StreamError::NoDevice)?;

            let default_stream = try_from_device(&default_device);

            default_stream.or_else(|original_err| {
                // default device didn't work, try other ones
                let mut devices = match rodio::cpal::default_host().output_devices() {
                    Ok(d) => d,
                    Err(_) => return Err(original_err),
                };

                devices
                    .find_map(|d| try_from_device(&d).ok())
                    .ok_or(original_err)
            })
        }

        // https://github.com/RustAudio/rodio/issues/214
        let (config, stream_handle) = std::thread::spawn(|| {
            let (config, stream, handle) = create_rodio_output_stream().unwrap();
            mem::forget(stream);
            (config, handle)
        })
        .join()
        .unwrap();
        Self {
            config,
            output_stream_handle: Arc::new(stream_handle),
            listener: Arc::new(Mutex::new({
                let mut listener = Listener {
                    pos: vec3::ZERO,
                    forward: vec3(0.0, -1.0, 0.0),
                    up: vec3(0.0, 0.0, 1.0),
                    left_ear: vec3::ZERO,
                    right_ear: vec3::ZERO,
                };
                listener.update_ears();
                listener
            })),
            volume: Arc::new(atomic_float::AtomicF32::new(1.0)),
        }
    }

    pub fn set_volume(&self, volume: f64) {
        self.volume
            .store(volume as f32, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_listener_position(&self, pos: vec3<f64>) {
        let mut listener = self.listener.lock().unwrap();
        listener.pos = pos.map(|x| x as f32);
        listener.update_ears();
    }

    pub fn set_listener_orientation(&self, forward: vec3<f64>, up: vec3<f64>) {
        let mut listener = self.listener.lock().unwrap();
        listener.forward = forward.map(|x| x as f32);
        listener.up = up.map(|x| x as f32);
        listener.update_ears();
    }
}

pub struct Sound {
    geng: Geng,
    source: rodio::source::Buffered<
        rodio::source::UniformSourceIterator<rodio::Decoder<std::io::Cursor<Vec<u8>>>, i16>,
    >,
    pub looped: bool,
}

impl Sound {
    pub(crate) fn new(geng: &Geng, data: Vec<u8>) -> Self {
        Self {
            geng: geng.clone(),
            source: rodio::Source::buffered(rodio::source::UniformSourceIterator::new(
                rodio::Decoder::new(std::io::Cursor::new(data)).expect("Failed to decode audio"),
                geng.audio().config.channels(), // TODO: what if more than 2 channels? we are screwed? LUL
                geng.audio().config.sample_rate().0,
            )),
            looped: false,
        }
    }
    pub fn duration(&self) -> Duration {
        rodio::Source::total_duration(&self.source).unwrap().into()
    }
    pub fn effect(&self) -> SoundEffect {
        let spatial_params = Arc::new(Mutex::new(None));
        SoundEffect {
            geng: self.geng.clone(),
            sink: Some({
                let sink = rodio::Sink::try_new(&self.geng.audio().output_stream_handle).unwrap();
                sink.pause();
                sink
            }),
            source: Some(if self.looped {
                Box::new(Source::new(
                    self.geng.audio(),
                    spatial_params.clone(),
                    &rodio::Source::repeat_infinite(self.source.clone()),
                ))
            } else {
                Box::new(Source::new(
                    self.geng.audio(),
                    spatial_params.clone(),
                    &self.source,
                ))
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
    pos: vec3<f32>,
    ref_dist: f32,
    max_dist: f32,
}

impl Default for SpatialParams {
    fn default() -> Self {
        Self {
            pos: vec3::ZERO,
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
    next_update_volume: usize,
    spatial_params: Arc<Mutex<Option<SpatialParams>>>,
    listener: Arc<Mutex<Listener>>,
    volume: Arc<atomic_float::AtomicF32>,
}

impl<I> Source<I>
where
    I: rodio::Source,
    I::Item: rodio::Sample,
{
    fn new(
        context: &AudioContext,
        spatial_params: Arc<Mutex<Option<SpatialParams>>>,
        source: &I,
    ) -> Self
    where
        I: Clone,
    {
        Self {
            next_update_volume: 0,
            spatial: rodio::source::ChannelVolume::new(source.clone(), vec![0.0, 0.0]),
            regular: source.clone(),
            is_spatial: false,
            spatial_params,
            listener: context.listener.clone(),
            volume: context.volume.clone(),
        }
    }
    fn update_volume(&mut self) {
        if let Some(spatial_params) = &*self.spatial_params.lock().unwrap() {
            self.is_spatial = true;
            let listener = self.listener.lock().unwrap();

            // https://dvcs.w3.org/hg/audio/raw-file/tip/webaudio/specification.html#Spatialization-panning-algorithm
            let delta_pos = spatial_params.pos - listener.pos;
            let listener_right = vec3::cross(listener.forward, listener.up);

            let plane_delta_pos = delta_pos - listener.up * vec3::dot(delta_pos, listener.up);

            // kind of hack to skip atan, approximately correct
            let gain_right =
                (vec3::dot(listener_right, plane_delta_pos.normalize_or_zero()) + 1.0) / 2.0;
            let gain_right = gain_right.clamp(0.0, 1.0); // just in case
            let gain_left = (1.0 - gain_right.sqr()).sqrt();

            let distance_gain = (1.0
                - (delta_pos.len() - spatial_params.ref_dist)
                    / (spatial_params.max_dist - spatial_params.ref_dist))
                .clamp(0.0, 1.0);

            let gain_left = gain_left * distance_gain;
            let gain_right = gain_right * distance_gain;
            self.spatial.set_volume(0, gain_left);
            self.spatial.set_volume(1, gain_right);
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
        if self.next_update_volume == 0 {
            self.update_volume();
            self.next_update_volume = 100;
        } else {
            self.next_update_volume -= 1;
        }
        if self.is_spatial {
            self.spatial.next()
        } else {
            self.regular.next()
        }
        .map(|sample| {
            rodio::Sample::amplify(
                sample,
                self.volume.load(std::sync::atomic::Ordering::SeqCst),
            )
        })
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
    source: Option<Box<dyn rodio::Source<Item = i16> + Send>>,
}

impl SoundEffect {
    pub fn set_volume(&mut self, volume: f64) {
        self.sink().set_volume(volume as f32);
    }
    pub fn play(&mut self) {
        self.play_from(Duration::from_secs_f64(0.0));
    }
    pub fn play_from(&mut self, offset: Duration) {
        let source = self.source.take().expect("Already playing");
        let source = rodio::Source::skip_duration(source, offset.into());
        self.sink().append(source);
        self.sink().play();
    }
    pub fn set_speed(&mut self, speed: f64) {
        if let Some(sink) = &mut self.sink {
            sink.set_speed(speed as f32);
        }
    }
    pub fn stop(&mut self) {
        self.sink().stop();
    }
    // TODO web
    // pub fn pause(&mut self) {
    //     self.sink().pause();
    // }
    fn sink(&mut self) -> &mut rodio::Sink {
        self.sink.as_mut().unwrap()
    }
    pub fn set_position(&mut self, position: vec3<f64>) {
        self.make_spatial(|spatial| spatial.pos = position.map(|x| x as f32));
    }
    pub fn set_ref_distance(&mut self, distance: f64) {
        self.make_spatial(|spatial| spatial.ref_dist = distance as f32);
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
