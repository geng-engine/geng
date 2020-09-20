use super::*;

pub struct Renderer {
    geng: Rc<Geng>,
}

impl Renderer {
    pub fn new(geng: &Rc<Geng>) -> Self {
        Self { geng: geng.clone() }
    }
    pub fn update(&mut self, delta_time: f32) {}
    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer, model: &Model) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

        let center_coords = framebuffer.size().map(|x| (x as f32) / 2.0);

        self.geng.draw_2d().circle(
            framebuffer,
            model.ball.position + center_coords,
            model.ball.radius,
            Color::RED,
        );
        self.geng.draw_2d().quad(
            framebuffer,
            AABB::pos_size(
                model.player_left.position - model.player_left.size + center_coords,
                model.player_left.size * 2.0,
            ),
            Color::BLUE,
        );
        self.geng.draw_2d().quad(
            framebuffer,
            AABB::pos_size(
                model.player_right.position - model.player_right.size + center_coords,
                model.player_right.size * 2.0,
            ),
            Color::YELLOW,
        );
    }
}
