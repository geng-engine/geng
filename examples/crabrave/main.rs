use geng::prelude::*;

#[derive(geng::asset::Load)]
struct Assets {
    #[load(postprocess = "pixelate")]
    body: ugli::Texture,
    #[load(postprocess = "pixelate")]
    front_leg: ugli::Texture,
    #[load(postprocess = "pixelate")]
    back_leg: ugli::Texture,
    #[load(postprocess = "pixelate")]
    hand: ugli::Texture,
}

fn pixelate(texture: &mut ugli::Texture) {
    texture.set_filter(ugli::Filter::Nearest);
}

struct CrabRave {
    geng: Geng,
    assets: Hot<Assets>,
    t: f32,
    limb_offsets: [vec2<f32>; 5],
}

impl CrabRave {
    fn new(geng: Geng, assets: Hot<Assets>) -> Self {
        Self {
            geng,
            assets,
            t: 0.0,
            limb_offsets: [vec2::ZERO; 5],
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
    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyPress { key: geng::Key::S } = event {
            if self.geng.window().is_key_pressed(geng::Key::ControlLeft) {
                let mut texture =
                    ugli::Texture::new_uninitialized(self.geng.ugli(), self.geng.window().size());
                texture.render_into(|framebuffer| {
                    self.draw(framebuffer);
                });
                let mut output = std::io::Cursor::new(Vec::new());
                texture
                    .to_image_image()
                    .write_to(&mut output, image::ImageFormat::Png)
                    .unwrap();
                file_dialog::save("crabrave.png", &output.into_inner()).unwrap();
            }
        }
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        let camera = geng::Camera2d {
            center: vec2::ZERO,
            rotation: Angle::ZERO,
            fov: Camera2dFov::MinSide(3.0),
        };
        let x = (self.t / 10.0).cos();
        let body = mat3::translate(
            vec2(x, 0.0)
                + vec2(1.0, 0.0).rotate(Angle::from_radians(self.t)) * vec2(1.0, 0.5) * 0.1,
        ) * mat3::rotate(Angle::from_radians(self.t.sin() * 0.2));

        let limb =
            |attach_pos: vec2<f32>, end_pos: vec2<f32>, target_end_pos: vec2<f32>| -> mat3<f32> {
                let attach_pos_world = (body * attach_pos.extend(1.0)).xy();
                let (attach_pos, attach_pos_world, end_pos, target_end_pos) =
                    (end_pos, target_end_pos, attach_pos, attach_pos_world);
                let m = |v: vec2<f32>| -> mat3<f32> {
                    let v = v.normalize_or_zero();
                    mat3::from_orts(v, v.rotate_90())
                };
                let m1 = m(attach_pos - end_pos);
                let m2 = m(attach_pos_world - target_end_pos);
                mat3::translate(target_end_pos) * m2 * m1.inverse() * mat3::translate(-end_pos)
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
        ) * mat3::scale(vec2(-1.0, 1.0));
        let back_left_leg = limb(
            vec2(-0.4, -0.6),
            vec2(-0.4, -0.9),
            vec2(-0.4, -0.9) + self.limb_offsets[2],
        );
        let back_right_leg = limb(
            vec2(0.4, -0.6),
            vec2(0.4, -0.9),
            vec2(0.4, -0.9) + self.limb_offsets[3],
        ) * mat3::scale(vec2(-1.0, 1.0));
        let left_hand = limb(
            vec2(-0.5, -0.5),
            vec2(-0.75, 0.0),
            vec2(-0.75, 0.0) + self.limb_offsets[4],
        );
        let right_hand = limb(
            vec2(0.5, -0.5),
            vec2(0.75, 0.0),
            vec2(0.75, 0.0) + self.limb_offsets[4],
        ) * mat3::scale(vec2(-1.0, 1.0));

        let assets = self.assets.get();
        for (texture, matrix) in [
            (&assets.back_leg, back_left_leg),
            (&assets.back_leg, back_right_leg),
            (&assets.body, body),
            (&assets.front_leg, front_left_leg),
            (&assets.front_leg, front_right_leg),
            (&assets.hand, left_hand),
            (&assets.hand, right_hand),
        ] {
            self.geng.draw2d().draw2d(
                framebuffer,
                &camera,
                &draw2d::TexturedQuad::unit(texture).transform(matrix),
            );
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("CrabRave", |geng| async move {
        let assets: Hot<Assets> = geng
            .asset_manager()
            .load(run_dir().join("assets"))
            .await
            .expect("Failed to load assets");
        geng.run_state(CrabRave::new(geng.clone(), assets)).await;
    });
}
