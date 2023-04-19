use super::*;

// TODO align?
pub struct Text<F: std::borrow::Borrow<Font>, T: AsRef<str>> {
    pub font: F,
    pub text: T,
    pub color: Rgba<f32>,
    pub into_unit_transform: mat3<f32>,
    pub transform: mat3<f32>,
    pub true_transform: mat3<f32>, // TODO: only have this
}

impl<F: std::borrow::Borrow<Font>, T: AsRef<str>> Text<F, T> {
    pub fn unit(font: F, text: T, color: Rgba<f32>) -> Self {
        if let Some(aabb) = font
            .borrow()
            .measure(text.as_ref(), vec2::splat(TextAlign::LEFT))
        {
            let aspect = aabb.width() / aabb.height();
            Self {
                font,
                text,
                color,
                into_unit_transform: (mat3::translate(aabb.center())
                    * mat3::scale(aabb.size() / 2.0))
                .inverse(),
                transform: mat3::scale(vec2(aspect, 1.0)),
                true_transform: mat3::translate(vec2(-aspect, -1.0)) * mat3::scale_uniform(4.0),
            }
        } else {
            Self {
                font,
                text,
                color,
                into_unit_transform: mat3::identity(),
                transform: mat3::scale_uniform(0.0),
                true_transform: mat3::identity(),
            }
        }
    }
}

impl<F: std::borrow::Borrow<Font>, T: AsRef<str>> Transform2d<f32> for Text<F, T> {
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
        self.true_transform = transform * self.true_transform;
    }
}

impl<F: std::borrow::Borrow<Font>, T: AsRef<str>> Draw2d for Text<F, T> {
    fn draw2d_transformed(
        &self,
        _: &Helper,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: mat3<f32>,
    ) {
        self.font.borrow().draw(
            framebuffer,
            camera,
            self.text.as_ref(),
            vec2::splat(TextAlign::LEFT),
            transform * self.transform * self.into_unit_transform,
            self.color,
        );
    }
}
