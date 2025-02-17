use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Camera2dFov {
    Vertical(f32),
    Horizontal(f32),
    MinSide(f32),
    MaxSide(f32),
    Cover { width: f32, height: f32, scale: f32 },
    FitInto { width: f32, height: f32, scale: f32 },
}
impl Camera2dFov {
    pub fn value_mut(&mut self) -> &mut f32 {
        match self {
            Camera2dFov::Vertical(value) => value,
            Camera2dFov::Horizontal(value) => value,
            Camera2dFov::MinSide(value) => value,
            Camera2dFov::MaxSide(value) => value,
            Camera2dFov::FitInto { scale, .. } | Camera2dFov::Cover { scale, .. } => scale,
        }
    }
    pub fn value(&self) -> f32 {
        match *self {
            Camera2dFov::Vertical(value) => value,
            Camera2dFov::Horizontal(value) => value,
            Camera2dFov::MinSide(value) => value,
            Camera2dFov::MaxSide(value) => value,
            Camera2dFov::FitInto { scale, .. } | Camera2dFov::Cover { scale, .. } => scale,
        }
    }
}

/// 2-dimensional camera.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Camera2d {
    pub center: vec2<f32>,
    pub rotation: Angle<f32>,
    pub fov: Camera2dFov,
}

impl AbstractCamera2d for Camera2d {
    fn view_matrix(&self) -> mat3<f32> {
        mat3::rotate(-self.rotation) * mat3::translate(-self.center)
    }
    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat3<f32> {
        let aspect = framebuffer_size.aspect();
        let (vertical, fov) = match self.fov {
            Camera2dFov::Vertical(fov) => (true, fov),
            Camera2dFov::Horizontal(fov) => (false, fov),
            Camera2dFov::MinSide(fov) => (aspect > 1.0, fov),
            Camera2dFov::MaxSide(fov) => (aspect < 1.0, fov),
            Camera2dFov::Cover {
                width,
                height,
                scale,
            } => {
                let vertical = aspect > width / height;
                (vertical, scale * if vertical { height } else { width })
            }
            Camera2dFov::FitInto {
                width,
                height,
                scale,
            } => {
                let vertical = aspect < width / height;
                (vertical, scale * if vertical { height } else { width })
            }
        };
        let vertical_fov = if vertical { fov } else { fov / aspect };
        let horizontal_fov = 1.0 / aspect;
        mat3::scale(vec2(2.0 / horizontal_fov, 2.0 / vertical_fov))
    }
}
