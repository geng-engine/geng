use geng::prelude::*;

#[derive(geng::asset::Load)]
struct Assets {
    #[load(postprocess = "make_repeated")]
    texture: ugli::Texture,
}

fn make_repeated(texture: &mut ugli::Texture) {
    texture.set_wrap_mode_separate(ugli::WrapMode::Repeat, ugli::WrapMode::Clamp);
}

struct Example {
    geng: Geng,
    assets: Hot<Assets>,
}

impl Example {
    fn new(geng: Geng, assets: Hot<Assets>) -> Self {
        Self { geng, assets }
    }
}

impl geng::State for Example {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        const N: usize = 4;
        let points: Vec<vec2<f32>> = (0..=N)
            .map(|i| {
                let angle = f32::PI * i as f32 / N as f32;
                vec2(3.0, 0.0).rotate(f32::PI - angle)
            })
            .collect();
        struct Point {
            pos: vec2<f32>,
            normal: vec2<f32>,
        }
        let points: Vec<Point> = {
            let mut res = vec![Point {
                pos: points[0],
                normal: (points[1] - points[0]).rotate_90().normalize(),
            }];
            for i in 1..points.len() - 1 {
                let n1 = (points[i] - points[i - 1]).rotate_90().normalize();
                let n2 = (points[i + 1] - points[i]).rotate_90().normalize();
                const R: usize = 100;
                for j in 0..=R {
                    fn lerp(a: vec2<f32>, b: vec2<f32>, t: f32) -> vec2<f32> {
                        a * (1.0 - t) + b * t
                    }
                    fn slerp(a: vec2<f32>, b: vec2<f32>, t: f32) -> vec2<f32> {
                        lerp(a, b, t).normalize()
                    }
                    res.push(Point {
                        pos: points[i],
                        normal: slerp(n1, n2, j as f32 / R as f32),
                    });
                }
            }
            res.push(Point {
                pos: points[points.len() - 1],
                normal: (points[points.len() - 1] - points[points.len() - 2])
                    .rotate_90()
                    .normalize(),
            });
            res
        };
        let mut ts: Vec<f32> = vec![0.0; points.len()];
        for i in 1..points.len() {
            let p = |i: usize| points[i].pos + points[i].normal * 0.5;
            ts[i] = ts[i - 1] + (p(i) - p(i - 1)).len();
        }
        let assets = self.assets.get();
        let texture = &assets.texture;
        self.geng.draw2d().draw2d(
            framebuffer,
            &geng::Camera2d {
                center: vec2(0.0, 2.5),
                rotation: 0.0,
                fov: 10.0,
            },
            &draw2d::TexturedPolygon::strip(
                izip![points, ts]
                    .flat_map(move |(Point { pos: p, normal: n }, t)| {
                        let t = t / (texture.size().x as f32 / texture.size().y as f32);
                        [
                            draw2d::TexturedVertex {
                                a_pos: p,
                                a_vt: vec2(t, 0.0),
                                a_color: Rgba::WHITE,
                            },
                            draw2d::TexturedVertex {
                                a_pos: p + n,
                                a_vt: vec2(t, 1.0),
                                a_color: Rgba::WHITE,
                            },
                        ]
                    })
                    .collect(),
                texture,
            ),
        );
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new("Line Texture");
    geng.clone().run_loading(async move {
        let assets: Hot<Assets> = geng
            .asset_manager()
            .load(run_dir().join("assets"))
            .await
            .expect("Failed to load assets");
        Example::new(geng, assets)
    });
}
