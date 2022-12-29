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
    limb_offsets: [Vec2<f32>; 5],
}

impl CrabRave {
    fn new(geng: Geng, assets: Assets) -> Self {
        Self {
            geng,
            assets,
            t: 0.0,
            limb_offsets: [Vec2::ZERO; 5],
        }
    }
}

impl geng::State for CrabRave {
    fn update(&mut self, delta_time: f64) {
        self.t += delta_time as f32 * 10.0;
        let x = (self.t / 10.0).cos();
        for i in 0..5 {
            let y = (self.t + (i as f32 / 2.0) * f32::PI * 2.0).sin() * 0.1;
            self.limb_offsets[i] = vec2(x, y);
            // let p = if y > 0.0 {
            //     vec2(x, y)
            // } else {
            //     vec2(self.legs[i].x, 0.0)
            // };
            // self.legs[i] = p;
            // self.legs[i] += (p - self.legs[i]) * (delta_time as f32 * 10.0).min(1.0);
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let camera = geng::Camera2d {
            center: Vec2::ZERO,
            rotation: 0.0,
            fov: 3.0,
        };
        let x = (self.t / 10.0).cos();
        let body =
            Mat3::translate(vec2(x, 0.0) + vec2(1.0, 0.0).rotate(self.t) * vec2(1.0, 0.5) * 0.1)
                * Mat3::rotate(self.t.sin() * 0.2);

        let limb =
            |attach_pos: Vec2<f32>, end_pos: Vec2<f32>, target_end_pos: Vec2<f32>| -> Mat3<f32> {
                let attach_pos_world = (body * attach_pos.extend(1.0)).xy();
                let (attach_pos, attach_pos_world, end_pos, target_end_pos) =
                    (end_pos, target_end_pos, attach_pos, attach_pos_world);
                let m = |v: Vec2<f32>| -> Mat3<f32> {
                    let v = v.normalize_or_zero();
                    Mat3::from_orts(v, v.rotate_90())
                };
                let m1 = m(attach_pos - end_pos);
                let m2 = m(attach_pos_world - target_end_pos);
                Mat3::translate(target_end_pos) * m2 * m1.inverse() * Mat3::translate(-end_pos)
            };

        let front_left_leg = limb(
            vec2(-0.5, -0.5),
            vec2(-0.5, -1.0),
            vec2(-0.5, -1.0) + self.limb_offsets[0],
        );
        let front_right_leg = limb(
            vec2(0.5, -0.5),
            vec2(0.5, -1.0),
            vec2(0.5, -1.0) + self.limb_offsets[1],
        ) * Mat3::scale(vec2(-1.0, 1.0));
        let back_left_leg = limb(
            vec2(-0.4, -0.6),
            vec2(-0.4, -0.9),
            vec2(-0.4, -0.9) + self.limb_offsets[2],
        );
        let back_right_leg = limb(
            vec2(0.4, -0.6),
            vec2(0.4, -0.9),
            vec2(0.4, -0.9) + self.limb_offsets[3],
        ) * Mat3::scale(vec2(-1.0, 1.0));
        let left_hand = limb(
            vec2(-0.5, -0.5),
            vec2(-0.75, 0.0),
            vec2(-0.75, 0.0) + self.limb_offsets[4],
        );
        let right_hand = limb(
            vec2(0.5, -0.5),
            vec2(0.75, 0.0),
            vec2(0.75, 0.0) + self.limb_offsets[4],
        ) * Mat3::scale(vec2(-1.0, 1.0));

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
            geng::LoadAsset::load(&geng, &run_dir().join("assets")),
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
