use geng::prelude::*;

#[derive(geng::Assets)]
struct Assets {
    body: ugli::Texture,
    front_leg: ugli::Texture,
    back_leg: ugli::Texture,
    hand: ugli::Texture,
}

struct CrabRave {
    geng: Geng,
    assets: Assets,
    t: f32,
}

impl CrabRave {
    fn new(geng: Geng, assets: Assets) -> Self {
        Self {
            geng,
            assets,
            t: 0.0,
        }
    }
}

impl geng::State for CrabRave {
    fn update(&mut self, delta_time: f64) {
        self.t += delta_time as f32 * 10.0;
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let camera = geng::Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 3.0,
        };
        let body = Mat3::translate(vec2(1.0, 0.0).rotate(self.t) * vec2(1.0, 0.5) * 0.1)
            * Mat3::rotate(self.t.sin() * 0.2);

        let limb = |attach_pos: Vec2<f32>, end_pos: Vec2<f32>| -> Mat3<f32> {
            let attach_pos_world = (body * attach_pos.extend(1.0)).xy();
            let m = |p: Vec2<f32>| -> Mat3<f32> {
                let v = p - end_pos;
                Mat3::from_orts(v, v.rotate_90())
            };
            let m1 = m(attach_pos);
            let m2 = m(attach_pos_world);
            Mat3::translate(end_pos) * m2 * m1.inverse() * Mat3::translate(-end_pos)
        };

        let front_left_leg = limb(vec2(-0.5, -0.5), vec2(-0.5, -1.0));
        let front_right_leg = limb(vec2(0.5, -0.5), vec2(0.5, -1.0)) * Mat3::scale(vec2(-1.0, 1.0));
        let back_left_leg = limb(vec2(-0.4, -0.6), vec2(-0.4, -0.9));
        let back_right_leg = limb(vec2(0.4, -0.6), vec2(0.4, -0.9)) * Mat3::scale(vec2(-1.0, 1.0));
        let left_hand = limb(vec2(-0.5, -0.5), vec2(-0.75, 0.0));
        let right_hand = limb(vec2(0.5, -0.5), vec2(0.75, 0.0)) * Mat3::scale(vec2(-1.0, 1.0));

        for (texture, matrix) in [
            (&self.assets.back_leg, back_left_leg),
            (&self.assets.back_leg, back_right_leg),
            (&self.assets.body, body),
            (&self.assets.front_leg, front_left_leg),
            (&self.assets.front_leg, front_right_leg),
            (&self.assets.hand, left_hand),
            (&self.assets.hand, right_hand),
        ] {
            self.geng.draw_2d(
                framebuffer,
                &camera,
                &draw_2d::TexturedQuad::unit(texture).transform(matrix),
            );
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new("CrabRave");
    geng::run(
        &geng,
        geng::LoadingScreen::new(
            &geng,
            geng::EmptyLoadingScreen,
            geng::LoadAsset::load(&geng, &static_path()),
            {
                let geng = geng.clone();
                move |assets| {
                    let mut assets: Assets = assets.unwrap();
                    assets.body.set_filter(ugli::Filter::Nearest);
                    assets.back_leg.set_filter(ugli::Filter::Nearest);
                    assets.front_leg.set_filter(ugli::Filter::Nearest);
                    assets.hand.set_filter(ugli::Filter::Nearest);
                    CrabRave::new(geng, assets)
                }
            },
        ),
    );
}
