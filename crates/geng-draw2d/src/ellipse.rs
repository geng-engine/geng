use super::*;

pub struct Ellipse {
    pub transform: mat3<f32>,
    pub cut: f32,
    /// `cut` is relative to the radius and should be in range `0.0..=1.0`.
    pub color: Rgba<f32>,
}

impl Ellipse {
    pub fn circle(center: vec2<f32>, radius: f32, color: Rgba<f32>) -> Self {
        Self::unit(color).transform(mat3::translate(center) * mat3::scale_uniform(radius))
    }
    pub fn circle_with_cut(
        center: vec2<f32>,
        inner_radius: f32,
        radius: f32,
        color: Rgba<f32>,
    ) -> Self {
        Self {
            cut: inner_radius / radius,
            ..Self::unit(color).transform(mat3::translate(center) * mat3::scale_uniform(radius))
        }
    }
    pub fn unit(color: Rgba<f32>) -> Self {
        Self {
            transform: mat3::identity(),
            cut: 0.0,
            color,
        }
    }
    pub fn unit_with_cut(cut: f32, color: Rgba<f32>) -> Self {
        Self {
            transform: mat3::identity(),
            cut,
            color,
        }
    }
}

impl Draw2d for Ellipse {
    fn draw2d_transformed(
        &self,
        helper: &Helper,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn AbstractCamera2d,
        transform: mat3<f32>,
    ) {
        let framebuffer_size = framebuffer.size();
        ugli::draw(
            framebuffer,
            &helper.ellipse_program,
            ugli::DrawMode::TriangleFan,
            &helper.unit_quad_geometry,
            (
                ugli::uniforms! {
                    u_model_matrix: transform * self.transform,
                    u_color: self.color,
                    u_framebuffer_size: framebuffer_size,
                    u_inner_cut: self.cut,
                },
                camera.uniforms(framebuffer_size.map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..Default::default()
            },
        );
    }
}

impl Transform2d<f32> for Ellipse {
    fn bounding_quad(&self) -> batbox_lapp::Quad<f32> {
        batbox_lapp::Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}
