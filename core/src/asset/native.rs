use crate::*;

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

impl LoadAsset for ugli::Texture {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let path = path.to_owned();
        let ugli = geng.ugli().clone();
        geng.asset_manager
            .threadpool
            .spawn(move || {
                info!("Loading {:?}", path);
                fn load(path: &str) -> Result<image::RgbaImage, Error> {
                    use failure::ResultExt;
                    let image = image::open(path).context(path.to_owned())?;
                    Ok(match image {
                        image::DynamicImage::ImageRgba8(image) => image,
                        _ => image.to_rgba(),
                    })
                }
                load(&path)
            })
            .map(|result| result.unwrap())
            .map(
                move |image: Result<image::RgbaImage, Error>| -> Result<ugli::Texture, Error> {
                    Ok(ugli::Texture::from_image(&ugli, image?))
                },
            )
            .boxed_local()
    }
}

impl LoadAsset for Sound {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let path = path.to_owned();
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
            .map(|result| result.unwrap())
            .boxed_local()
    }
}

impl LoadAsset for String {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let path = path.to_owned();
        geng.asset_manager
            .threadpool
            .spawn(move || {
                info!("Loading {:?}", path);
                let mut result = String::new();
                std::fs::File::open(path)?.read_to_string(&mut result)?;
                Ok(result)
            })
            .map(|result| result.unwrap())
            .boxed_local()
    }
}
