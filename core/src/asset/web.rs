use crate::*;

pub struct AssetManager {}

impl AssetManager {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadAsset for ugli::Texture {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let image = stdweb::web::html_element::ImageElement::new();
        let handler = {
            let image = image.clone();
            let ugli = geng.ugli().clone();
            let path = path.to_owned();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(ugli::Texture::from_image(&ugli, image))
                    } else {
                        Err(format_err!("Failed to load image from {:?}", path))
                    })
                    .map_err(|_| ())
                    .unwrap();
            }
        };
        // TODO: https://github.com/koute/stdweb/issues/171
        js! {
            @(no_return)
            var handler = @{stdweb::Once(handler)};
            var image = @{image.clone()};
            image.onload = function() { handler(true); };
            image.onerror = function() { handler(false); };
        }
        image.set_src(path);
        receiver.map(|result| result.unwrap()).boxed_local()
    }
}

impl LoadAsset for Sound {
    fn load(_: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let audio: stdweb::Reference = stdweb::unstable::TryInto::try_into(js! {
            return new Audio(@{path});
        })
        .unwrap();
        let handler = {
            let audio = audio.clone();
            let path = path.to_owned();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(Sound {
                            inner: audio,
                            looped: false,
                        })
                    } else {
                        Err(format_err!("Failed to load sound from {:?}", path))
                    })
                    .map_err(|_| ())
                    .unwrap();
            }
        };
        // TODO: https://github.com/koute/stdweb/issues/171
        js! {
            @(no_return)
            var handler = @{stdweb::Once(handler)};
            var audio = @{&audio};
            audio.oncanplaythrough = function() { handler(true); };
            audio.onerror = function() { handler(false); };
        }
        receiver.map(|result| result.unwrap()).boxed_local()
    }
}

impl LoadAsset for String {
    fn load(_: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        let request = stdweb::web::XmlHttpRequest::new();
        request.open("GET", path).unwrap();
        let handler = {
            let request = request.clone();
            let path = path.to_owned();
            move |success: bool| {
                sender
                    .send(if success {
                        Ok(request.response_text().unwrap().unwrap())
                    } else {
                        Err(format_err!("Failed to load {:?}", path))
                    })
                    .map_err(|_| ())
                    .unwrap();
            }
        };
        js! {
            @(no_return)
            var handler = @{stdweb::Once(handler)};
            var request = @{request.clone()};
            request.onreadystatechange = function () {
                if (request.readyState == 4) {
                    handler(request.status == 200);
                }
            };
        }
        request.send().unwrap();
        receiver.map(|result| result.unwrap()).boxed_local()
    }
}
