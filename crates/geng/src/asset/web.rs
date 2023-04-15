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
