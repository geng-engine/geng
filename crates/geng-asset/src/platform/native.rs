use super::*;
use anyhow::Context as _;

async fn spawn_blocking<T>(f: impl FnOnce() -> T) -> T {
    // TODO threadpool
    f()
}

pub fn load_texture(manager: &Manager, path: &std::path::Path) -> Future<ugli::Texture> {
    let ugli = manager.ugli().clone();
    let path = path.to_owned();
    let image_future = spawn_blocking(move || {
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
    async move { Ok(ugli::Texture::from_image(&ugli, image_future.await?)) }.boxed_local()
}
