use super::*;

pub fn load_texture(manager: &Manager, path: &std::path::Path) -> Future<ugli::Texture> {
    let ugli = manager.ugli().clone();
    let path = path.to_owned();
    async move {
        log::debug!("Loading {:?}", path);
        let image = image::load_from_memory(&file::load_bytes(path).await?)?;
        let image = match image {
            image::DynamicImage::ImageRgba8(image) => image,
            _ => image.to_rgba8(),
        };
        Ok(ugli::Texture::from_image(&ugli, image))
    }
    .boxed_local()
}
