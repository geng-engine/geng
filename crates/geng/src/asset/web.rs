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
                    sender
                        .send(if success {
                            Ok(request.response().expect("1").into())
                        } else {
                            Err(anyhow!("Failed to load {:?}", path))
                        })
                        .map_err(|_| ());
                    // TODO this can be Err if other asset was not loaded, so the future for this one was dropped
                    // .expect(&format!("{path:?}"));
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
            return async move { receiver.await.unwrap() };
        }

        let geng = geng.clone();
        let data = load_arraybuffer(path);
        let path = path.to_owned();
        async move {
            let data = data.await?;
            let Ok(promise) = geng.audio().context.decode_audio_data(&data) else {
                anyhow::bail!("whoops"); // TODO
            };
            let Ok(buffer) = wasm_bindgen_futures::JsFuture::from(promise).await else {
                anyhow::bail!("whoops"); // TODO
            };
            let sound = Sound::new(&geng, buffer.into());
            Ok(sound)
        }
        .boxed_local()
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
