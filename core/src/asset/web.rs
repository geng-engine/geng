use super::*;

pub struct AssetManager {}

impl AssetManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadAsset for ugli::Texture {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let image = web_sys::HtmlImageElement::new().unwrap();
        let path = Rc::new(path.to_owned());
        let handler = {
            let image = image.clone();
            let ugli = geng.ugli().clone();
            let path = path.clone();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(ugli::Texture::from_image(&ugli, &image))
                    } else {
                        Err(anyhow!("Failed to load image from {:?}", path))
                    })
                    .map_err(|_| ())
                    .unwrap();
            }
        };
        #[wasm_bindgen(inline_js = r#"
        export function setup(image, handler) {
            image.onload = function() { handler(true); };
            image.onerror = function() { handler(false); };
        }
        "#)]
        extern "C" {
            fn setup(image: &web_sys::HtmlImageElement, handler: wasm_bindgen::JsValue);
        }
        setup(
            &image,
            wasm_bindgen::closure::Closure::once_into_js(handler),
        );
        image.set_src(&path);
        Box::pin(async move { receiver.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("png");
}

impl LoadAsset for Sound {
    fn load(_: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let audio = web_sys::HtmlAudioElement::new_with_src(&path).unwrap();
        let path = Rc::new(path.to_owned());
        let handler = {
            let audio = audio.clone();
            let path = path.clone();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(Sound {
                            inner: audio,
                            looped: false,
                        })
                    } else {
                        Err(anyhow!("Failed to load sound from {:?}", path))
                    })
                    .map_err(|_| ())
                    .unwrap();
            }
        };
        #[wasm_bindgen(inline_js = r#"
        export function setup(audio, handler) {
            audio.oncanplaythrough = function() { handler(true); };
            audio.onerror = function() { handler(false); };
        }
        "#)]
        extern "C" {
            fn setup(audio: &web_sys::HtmlAudioElement, handler: wasm_bindgen::JsValue);
        }
        setup(
            &audio,
            wasm_bindgen::closure::Closure::once_into_js(handler),
        );
        Box::pin(async move { receiver.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("wav");
}

impl LoadAsset for String {
    fn load(_: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let request = web_sys::XmlHttpRequest::new().unwrap();
        request.open("GET", &path).unwrap();
        let path = Rc::new(path.to_owned());
        let handler = {
            let request = request.clone();
            let path = path.clone();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(request.response_text().unwrap().unwrap())
                    } else {
                        Err(anyhow!("Failed to load {:?}", path))
                    })
                    .map_err(|_| ())
                    .unwrap();
            }
        };
        #[wasm_bindgen(inline_js = r#"
        export function setup(request, handler) {
            request.onreadystatechange = function () {
                if (request.readyState == 4) {
                    handler(request.status == 200);
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
        Box::pin(async move { receiver.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("txt");
}
