use super::*;

impl LoadAsset for ugli::Texture {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
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
        export function setup_image(image, handler) {
            image.onload = function() { handler(true); };
            image.onerror = function() { handler(false); };
        }
        "#)]
        extern "C" {
            fn setup_image(image: &web_sys::HtmlImageElement, handler: wasm_bindgen::JsValue);
        }
        setup_image(
            &image,
            wasm_bindgen::closure::Closure::once_into_js(handler),
        );
        image.set_src(path.to_str().unwrap());
        Box::pin(async move { receiver.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("png");
}

#[cfg(feature = "audio")]
impl LoadAsset for Sound {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let audio = web_sys::HtmlAudioElement::new_with_src(path.to_str().unwrap()).unwrap();
        let path = Rc::new(path.to_owned());
        let geng = geng.clone();
        let handler = {
            let audio = audio.clone();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(Sound {
                            geng,
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
        export function setup_audio(audio, handler) {
            audio.oncanplaythrough = function() { handler(true); };
            audio.onerror = function() { handler(false); };
            audio.load();
        }
        "#)]
        extern "C" {
            fn setup_audio(audio: &web_sys::HtmlAudioElement, handler: wasm_bindgen::JsValue);
        }
        setup_audio(
            &audio,
            wasm_bindgen::closure::Closure::once_into_js(handler),
        );
        Box::pin(async move { receiver.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("wav");
}

impl LoadAsset for String {
    fn load(_: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let request = web_sys::XmlHttpRequest::new().unwrap();
        request.open("GET", path.to_str().unwrap()).unwrap();
        let path = Rc::new(path.to_owned());
        let handler = {
            let request = request.clone();
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
        export function setup_string(request, handler) {
            request.onreadystatechange = function () {
                if (request.readyState == 4) {
                    handler(request.status == 200);
                }
            };
        }
        "#)]
        extern "C" {
            fn setup_string(request: &web_sys::XmlHttpRequest, handler: wasm_bindgen::JsValue);
        }
        setup_string(
            &request,
            wasm_bindgen::closure::Closure::once_into_js(handler),
        );
        request.send().unwrap();
        Box::pin(async move { receiver.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("txt");
}

impl LoadAsset for Vec<u8> {
    fn load(_: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let request = web_sys::XmlHttpRequest::new().unwrap();
        request.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
        request.open("GET", path.to_str().unwrap()).unwrap();
        let path = Rc::new(path.to_owned());
        let handler = {
            let request = request.clone();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(js_sys::Uint8Array::new(&request.response().unwrap()).to_vec())
                    } else {
                        Err(anyhow!("Failed to load {:?}", path))
                    })
                    .map_err(|_| ())
                    .unwrap();
            }
        };
        #[wasm_bindgen(inline_js = r#"
        export function setup_string(request, handler) {
            request.onreadystatechange = function () {
                if (request.readyState == 4) {
                    handler(request.status == 200);
                }
            };
        }
        "#)]
        extern "C" {
            fn setup_string(request: &web_sys::XmlHttpRequest, handler: wasm_bindgen::JsValue);
        }
        setup_string(
            &request,
            wasm_bindgen::closure::Closure::once_into_js(handler),
        );
        request.send().unwrap();
        Box::pin(async move { receiver.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("txt");
}
