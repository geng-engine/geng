use super::*;

// TODO: on web this always copies into a vec, maybe this should return a reference that we can use to get data later
pub fn load_file(path: &std::path::Path) -> impl Future<Output = anyhow::Result<Vec<u8>>> {
    #[cfg(target_arch = "wasm32")]
    {
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
        async move { receiver.await.unwrap() }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let file = std::fs::File::open(path);
        async move {
            let mut file = file?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            Ok(buf)
        }
    }
}
