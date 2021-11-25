use super::*;

pub struct Text<F: std::borrow::Borrow<Font>, T: AsRef<str>> {
    font: F,
    text: T,
    color: Color<f32>,
    into_unit_transform: Mat3<f32>,
    transform: Mat3<f32>,
}

const SIZE_HACK: f32 = 1000.0;

impl<F: std::borrow::Borrow<Font>, T: AsRef<str>> Text<F, T> {
    pub fn unit(font: F, text: T, color: Color<f32>) -> Self {
        if let Some(aabb) = font
            .borrow()
            .measure_at(text.as_ref(), vec2(0.0, 0.0), SIZE_HACK)
        {
            let aspect = aabb.width() / aabb.height();
            Self {
                font,
                text,
                color,
                into_unit_transform: (Mat3::translate(aabb.center())
                    * Mat3::scale(aabb.size() / 2.0))
                .inverse(),
                transform: Mat3::scale(vec2(aspect, 1.0)),
            }
        } else {
            Self {
                font,
                text,
                color,
                into_unit_transform: Mat3::identity(),
                transform: Mat3::scale_uniform(0.0),
            }
        }
    }
}

impl<F: std::borrow::Borrow<Font>, T: AsRef<str>> Transform2d<f32> for Text<F, T> {
    fn bounding_quad(&self) -> batbox::Quad<f32> {
        batbox::Quad::from_matrix(self.transform)
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

impl<F: std::borrow::Borrow<Font>, T: AsRef<str>> Draw2d for Text<F, T> {
    fn draw_2d_transformed(
        &self,
        _geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: Mat3<f32>,
    ) {
        self.font.borrow().draw_impl(
            framebuffer,
            camera,
            transform * self.transform * self.into_unit_transform,
            self.text.as_ref(),
            vec2(0.0, 0.0),
            SIZE_HACK,
            self.color,
        );
    }
}