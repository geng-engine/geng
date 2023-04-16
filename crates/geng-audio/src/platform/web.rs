use anyhow::anyhow;
use batbox_la::*;
use batbox_time as time;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub struct Context {
    pub context: web_sys::AudioContext,
    master_gain_node: web_sys::GainNode,
}

impl Context {
    pub fn new() -> Self {
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
    context: Rc<Context>,
    inner: web_sys::AudioBuffer,
    pub looped: bool,
}

impl Sound {
    pub async fn load(context: &Rc<Context>, path: &std::path::Path) -> anyhow::Result<Self> {
        // TODO: batbox::load_file
        fn load_arraybuffer(
            path: &std::path::Path,
        ) -> impl Future<Output = anyhow::Result<js_sys::ArrayBuffer>> {
            let (sender, receiver) = futures::channel::oneshot::channel();
            let request = web_sys::XmlHttpRequest::new().unwrap();
            request.open("GET", path.to_str().unwrap()).unwrap();
            request.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
            let path = Rc::new(path.to_owned());
            let handler = {
                let request = request.clone();
                move |success: bool| {
                    let result = if success {
                        Ok(request.response().expect("1").into())
                    } else {
                        Err(anyhow!("Failed to load {:?}", path))
                    };
                    // Err means that future was dropped
                    let (Ok(()) | Err(_)) = sender.send(result);
                }
            };
            #[wasm_bindgen(inline_js = r#"
            export function setup(request, handler) {
                request.onreadystatechange = function () {
                    if (request.readyState == 4) {
                        handler(request.status == 200 || request.status == 206); // TODO why is there 206?
                    }
                };
            }
            "#)]
            extern "C" {
                fn setup(request: &web_sys::XmlHttpRequest, handler: wasm_bindgen::JsValue);
            }
            setup(
                &request,
                wasm_bindgen::closure::Closure::once_into_js(handler),
            );
            request.send().unwrap();
            async move { receiver.await.unwrap() }
        }

        let data = load_arraybuffer(path).await?;
        Self::decode_arraybuffer(context, data).await
    }
    pub async fn decode_bytes(context: &Rc<Context>, data: Vec<u8>) -> anyhow::Result<Self> {
        let arraybuffer = js_sys::Uint8Array::from(data.as_slice()).buffer(); // TODO hmm
        Self::decode_arraybuffer(context, arraybuffer).await
    }
    pub async fn decode_arraybuffer(
        context: &Rc<Context>,
        data: js_sys::ArrayBuffer,
    ) -> anyhow::Result<Self> {
        let Ok(promise) = context.context.decode_audio_data(&data) else {
            anyhow::bail!("whoops"); // TODO
        };
        let Ok(buffer) = wasm_bindgen_futures::JsFuture::from(promise).await else {
            anyhow::bail!("whoops"); // TODO
        };
        Ok(Self::from_audio_buffer(context, buffer.into()))
    }
    pub fn from_audio_buffer(context: &Rc<Context>, buffer: web_sys::AudioBuffer) -> Self {
        Self {
            context: context.clone(),
            inner: buffer,
            looped: false,
        }
    }
    pub fn duration(&self) -> time::Duration {
        time::Duration::from_secs_f64(self.inner.duration())
    }
    pub fn effect(&self) -> SoundEffect {
        let buffer_node = web_sys::AudioBufferSourceNode::new(&self.context.context).unwrap();
        buffer_node.set_buffer(Some(&self.inner));
        buffer_node.set_loop(self.looped);
        let gain_node = web_sys::GainNode::new(&self.context.context).unwrap();
        buffer_node.connect_with_audio_node(&gain_node).unwrap();
        let audio_node: web_sys::AudioNode = gain_node.clone().into();
        audio_node
            .connect_with_audio_node(&self.context.master_gain_node)
            .unwrap();
        SoundEffect {
            context: self.context.clone(),
            inner: buffer_node,
            gain_node,
            spatial_state: SpatialState::NotSpatial(audio_node),
        }
    }
}

pub struct SoundEffect {
    context: Rc<Context>,
    inner: web_sys::AudioBufferSourceNode,
    gain_node: web_sys::GainNode,
    spatial_state: SpatialState,
}

impl SoundEffect {
    pub fn set_volume(&mut self, volume: f64) {
        self.gain_node.gain().set_value(volume as f32);
    }
    pub fn play(&mut self) {
        self.play_from(time::Duration::from_secs_f64(0.0));
    }
    pub fn play_from(&mut self, offset: time::Duration) {
        self.inner
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
            let panner_node = web_sys::PannerNode::new(&self.context.context).unwrap();
            panner_node.set_distance_model(web_sys::DistanceModelType::Linear);
            audio_node.disconnect().unwrap();
            audio_node
                .connect_with_audio_node(&panner_node)
                .unwrap()
                .connect_with_audio_node(&self.context.master_gain_node)
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
