use super::*;

pub struct CustomWidget<'a> {
    pub position: &'a mut Aabb2<f32>,         // Hidden
    pub animation_time: &'a mut f32,          // state
    pub sense: &'a mut geng::ui::Sense,       // Helper hidden state for interaction
    pub change: RefCell<&'a mut Option<f32>>, // Result of interaction is optionally a change that was made
    pub assets: &'a Assets,
    pub ratio: f32,
}

impl<'a> CustomWidget<'a> {
    pub fn new(cx: &'a geng::ui::Controller, assets: &'a Assets, ratio: f32) -> Self {
        Self {
            position: cx.get_state_with(|| Aabb2::point(vec2::ZERO)), // Specify default value for hidden state
            animation_time: cx.get_state(),
            sense: cx.get_state(), // Or just use Default trait
            change: RefCell::new(cx.get_state()),
            assets,
            ratio,
        }
    }

    // We had a RefCell so that this method doesn't need a mut reference to self
    pub fn get_change(&self) -> Option<f32> {
        self.change.borrow_mut().take()
    }
}

impl geng::ui::Widget for CustomWidget<'_> {
    fn calc_constraints(
        &mut self,
        _children: &geng::ui::ConstraintsContext,
    ) -> geng::ui::Constraints {
        geng::ui::Constraints {
            min_size: vec2(100.0, 100.0),
            flex: vec2(0.0, 0.0),
        }
    }
    // If using Sense helper this method must be added
    fn sense(&mut self) -> Option<&mut geng::ui::Sense> {
        Some(self.sense)
    }
    fn update(&mut self, delta_time: f64) {
        #![allow(unused_variables)]
    }
    fn draw(&mut self, cx: &mut geng::ui::DrawContext) {
        *self.position = cx.position.map(|x| x as f32); // Update hidden state to remember our widget's position

        #[derive(ugli::Vertex)]
        struct Vertex {
            a_pos: vec2<f32>,
        }

        ugli::draw(
            cx.framebuffer,
            &self.assets.shader,
            ugli::DrawMode::TriangleFan,
            &ugli::VertexBuffer::new_dynamic(
                cx.draw2d.ugli(),
                vec![
                    Vertex {
                        a_pos: vec2(0.0, 0.0),
                    },
                    Vertex {
                        a_pos: vec2(1.0, 0.0),
                    },
                    Vertex {
                        a_pos: vec2(1.0, 1.0),
                    },
                    Vertex {
                        a_pos: vec2(0.0, 1.0),
                    },
                ],
            ),
            (
                ugli::uniforms! {
                    u_texture: &self.assets.texture,
                    u_pos: cx.position.bottom_left().map(|x| x as f32),
                    u_size: cx.position.size().map(|x| x as f32),
                    u_ratio: self.ratio,
                },
                geng::PixelPerfectCamera.uniforms(cx.framebuffer.size().map(|x| x as f32)),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..default()
            },
        );
    }
    fn handle_event(&mut self, event: &geng::Event) {
        // Use helper to determine if we should process interactions
        if self.sense.is_captured() {
            if let geng::Event::MouseDown { position, .. }
            | geng::Event::MouseMove { position, .. } = &event
            {
                let new_value = ((position.y as f32 - self.position.min.y)
                    / self.position.height().max(0.1))
                .clamp(0.0, 1.0);
                **self.change.borrow_mut() = Some(new_value);
            }
        }
    }
}
