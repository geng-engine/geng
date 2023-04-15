use super::*;

#[derive(Clone)]
pub(crate) struct AssetManager {
    threadpool: threadpool::ThreadPool,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            threadpool: threadpool::ThreadPool::new(1),
            #[cfg(not(debug_assertions))]
            threadpool: default(),
        }
    }
    fn spawn<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        f: F,
    ) -> futures::channel::oneshot::Receiver<T> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        self.threadpool.execute(move || {
            if sender.send(f()).is_err() {
                panic!("Failed to send value");
            }
        });
        receiver
    }
}

impl LoadAsset for ugli::Texture {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let ugli = geng.ugli().clone();
        let path = path.to_owned();
        let image_future = geng.inner.asset_manager.spawn(move || {
            log::debug!("Loading {:?}", path);
            fn load(path: &std::path::Path) -> anyhow::Result<image::RgbaImage> {
                let image = image::open(path).context(format!("Failed to load {path:?}"))?;
                Ok(match image {
                    image::DynamicImage::ImageRgba8(image) => image,
                    _ => image.to_rgba8(),
                })
            }
            load(&path)
        });
        Box::pin(async move { Ok(ugli::Texture::from_image(&ugli, image_future.await??)) })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("png");
}
