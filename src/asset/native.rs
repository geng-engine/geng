use super::*;

#[derive(Clone)]
pub(crate) struct AssetManager {
    threadpool: ThreadPool,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            threadpool: ThreadPool::new(1),
            #[cfg(not(debug_assertions))]
            threadpool: default(),
        }
    }
}

impl LoadAsset for ugli::Texture {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let ugli = geng.ugli().clone();
        let path = path.to_owned();
        let image_future = geng.asset_manager.threadpool.spawn(move || {
            info!("Loading {:?}", path);
            fn load(path: &str) -> Result<image::RgbaImage, anyhow::Error> {
                let image = image::open(path).context(path.to_owned())?;
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

#[cfg(feature = "audio")]
impl LoadAsset for Sound {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let geng = geng.clone();
        let path = path.to_owned();
        let data = geng
            .asset_manager
            .threadpool
            .spawn(move || -> Result<_, anyhow::Error> {
                info!("Loading {:?}", path);
                let mut data = Vec::new();
                std::fs::File::open(path)?.read_to_end(&mut data)?;
                Ok(data)
            });
        Box::pin(async move {
            Ok(Sound {
                output_stream_handle: geng.audio.output_stream_handle.clone(),
                data: data.await??.into(),
                looped: false,
            })
        })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("wav");
}

impl LoadAsset for String {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let geng = geng.clone();
        let path = path.to_owned();
        let future = geng.asset_manager.threadpool.spawn(move || {
            info!("Loading {:?}", path);
            let mut result = String::new();
            std::fs::File::open(path)?.read_to_string(&mut result)?;
            Ok(result)
        });
        Box::pin(async move { future.await? })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("txt");
}
