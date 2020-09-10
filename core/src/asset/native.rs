use super::*;

use std::collections::BinaryHeap;

#[derive(Clone)]
pub(crate) struct AssetManager {
    threadpool: ThreadPool,
    queue: Rc<RefCell<BinaryHeap<Job>>>,
}

struct Job {
    priority: i32,
    f: Box<dyn FnOnce() + Send>,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Job) -> Option<std::cmp::Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            #[cfg(debug_assertions)]
            threadpool: ThreadPool::new(1),
            #[cfg(not(debug_assertions))]
            threadpool: default(),
            queue: Rc::new(RefCell::new(BinaryHeap::new())),
        }
    }
}

#[async_trait(?Send)]
impl LoadAsset for ugli::Texture {
    async fn load(geng: Rc<Geng>, path: String) -> Result<Self, anyhow::Error> {
        let ugli = geng.ugli().clone();
        let image = geng
            .asset_manager
            .threadpool
            .spawn(move || {
                info!("Loading {:?}", path);
                fn load(path: &str) -> Result<image::RgbaImage, anyhow::Error> {
                    let image = image::open(path).context(path.to_owned())?;
                    Ok(match image {
                        image::DynamicImage::ImageRgba8(image) => image,
                        _ => image.to_rgba(),
                    })
                }
                load(&path)
            })
            .await??;
        Ok(ugli::Texture::from_image(&ugli, image))
    }
    fn default_ext() -> Option<&'static str> {
        Some("png")
    }
}

#[async_trait(?Send)]
impl LoadAsset for Sound {
    async fn load(geng: Rc<Geng>, path: String) -> Result<Self, anyhow::Error> {
        geng.asset_manager
            .threadpool
            .spawn(move || {
                info!("Loading {:?}", path);
                let mut data = Vec::new();
                std::fs::File::open(path)?.read_to_end(&mut data)?;
                Ok(Sound {
                    data: data.into(),
                    looped: false,
                })
            })
            .await?
    }
    fn default_ext() -> Option<&'static str> {
        Some("wav")
    }
}

#[async_trait(?Send)]
impl LoadAsset for String {
    async fn load(geng: Rc<Geng>, path: String) -> Result<Self, anyhow::Error> {
        geng.asset_manager
            .threadpool
            .spawn(move || {
                info!("Loading {:?}", path);
                let mut result = String::new();
                std::fs::File::open(path)?.read_to_string(&mut result)?;
                Ok(result)
            })
            .await?
    }
    fn default_ext() -> Option<&'static str> {
        Some("txt")
    }
}
