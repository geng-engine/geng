use super::*;

pub fn load_texture(
    manager: &Manager,
    path: &std::path::Path,
    options: &TextureOptions,
) -> Future<ugli::Texture> {
    let manager = manager.clone();
    let path = path.to_owned();
    let options = options.clone();
    async move {
        log::debug!("Loading {:?}", path);
        let image = image::load_from_memory(&manager.load_bytes(path).await?)?;
        manager.yield_now().await;
        let mut image = match image {
            image::DynamicImage::ImageRgba8(image) => image,
            _ => image.to_rgba8(),
        };
        manager.yield_now().await;
        if options.premultiply_alpha {
            for pixel in image.pixels_mut() {
                use image::Pixel;
                *pixel = pixel.map_without_alpha(|x| {
                    (x as f32 * (pixel[3] as f32 / 0xff as f32)).round() as u8
                });
            }
        }
        manager.yield_now().await;
        Ok(ugli::Texture::from_image_image(manager.ugli(), image))
    }
    .boxed_local()
}
