use super::*;

pub struct Ellipse {
    pub transform: Mat3<f32>,
    pub cut: f32,
    /// `cut` is relative to the radius and should be in range `0.0..=1.0`.
    pub color: Rgba<f32>,
}

impl Ellipse {
    pub fn circle(center: Vec2<f32>, radius: f32, color: Rgba<f32>) -> Self {
        Self::unit(color).transform(Mat3::translate(center) * Mat3::scale_uniform(radius))
    }
    pub fn circle_with_cut(
        center: Vec2<f32>,
        inner_radius: f32,
        radius: f32,
        color: Rgba<f32>,
    ) -> Self {
        Self {
            cut: inner_radius / radius,
            ..Self::unit(color).transform(Mat3::translate(center) * Mat3::scale_uniform(radius))
        }
    }
    pub fn unit(color: Rgba<f32>) -> Self {
        Self {
            transform: Mat3::identity(),
            cut: 0.0,
            color,
        }
    }
    pub fn unit_with_cut(cut: f32, color: Rgba<f32>) -> Self {
        Self {
            transform: Mat3::identity(),
            cut,
            color,
        }
    }
}

impl Draw2d for Ellipse {
    fn draw_2d_transformed(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: Mat3<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &geng.inner.draw_2d.ellipse_program,
            ugli::DrawMode::TriangleFan,
            &geng.inner.draw_2d.unit_quad_geometry,
            (
                ugli::uniforms! {
                    u_model_matrix: transform * self.transform,
                    u_color: self.color,
                    u_framebuffer_size: framebuffer_size,
                    u_inner_cut: self.cut,
                },
                camera2d_uniforms(camera, framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::default()),
                ..default()
            },
        );
    }
}

impl Transform2d<f32> for Ellipse {
    fn bounding_quad(&self) -> batbox::geom::Quad<f32> {
        batbox::geom::Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}
