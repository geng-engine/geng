use geng::prelude::*;

#[derive(geng::Assets)]
struct Assets {
    texture: ugli::Texture,
}

struct Grid<'a> {
    table: Vec<Vec<Option<&'a dyn geng::Draw2d>>>,
    transform: mat3<f32>,
}

impl<'a> Grid<'a> {
    pub fn new(size: vec2<usize>, objects: impl IntoIterator<Item = &'a dyn geng::Draw2d>) -> Self {
        let mut table = vec![vec![None; size.y]; size.x];
        for (storage, object) in table.iter_mut().flat_map(|row| row.iter_mut()).zip(objects) {
            *storage = Some(object);
        }
        Self {
            table,
            transform: mat3::scale(size.map(|x| x as f32)),
        }
    }
}

impl<'a> Transform2d<f32> for Grid<'a> {
    fn bounding_quad(&self) -> Quad<f32> {
        Quad {
            transform: self.transform,
        }
    }
    fn apply_transform(&mut self, transform: mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

impl<'a> geng::Draw2d for Grid<'a> {
    fn draw_2d_transformed(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn geng::AbstractCamera2d,
        transform: mat3<f32>,
    ) {
        for (x, column) in self.table.iter().enumerate() {
            for (y, object) in column.iter().enumerate() {
                if let Some(object) = *object {
                    geng.draw_2d_transformed(
                        framebuffer,
                        camera,
                        &object
                            .transformed()
                            .fit_into(Aabb2::point(vec2::ZERO).extend_uniform(0.9))
                            .transform(
                                self.transform
                                    * mat3::translate(vec2(-1.0, -1.0))
                                    * mat3::scale_uniform(2.0)
                                    * mat3::scale(vec2(
                                        1.0 / self.table.len() as f32,
                                        1.0 / self.table[0].len() as f32,
                                    ))
                                    * mat3::translate(vec2(x as f32, y as f32))
                                    * mat3::scale_uniform(0.5)
                                    * mat3::translate(vec2(1.0, 1.0)),
                            ),
                        transform,
                    );
                }
            }
        }
    }
}

struct State {
    geng: Geng,
    camera: geng::Camera2d,
    objects: Vec<Box<dyn draw_2d::Draw2d>>,
}

impl State {
    fn new(geng: &Geng, assets: Assets) -> Self {
        let mut result = Self {
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 10.0,
            },
            objects: vec![],
        };
        result.add(
            draw_2d::Quad::unit(Rgba::WHITE)
                .transform(mat3::rotate(0.5) * mat3::scale_uniform(0.5)),
        );
        result.add(draw_2d::TexturedQuad::unit(ugli::Texture::new_with(
            geng.ugli(),
            vec2(2, 2),
            |pos| match (pos.x, pos.y) {
                (0, 0) => Rgba::BLACK,
                (1, 0) => Rgba::RED,
                (1, 1) => Rgba::GREEN,
                (0, 1) => Rgba::BLUE,
                _ => unreachable!(),
            },
        )));
        result.add(draw_2d::TexturedQuad::unit(assets.texture));
        result.add(draw_2d::TexturedQuad::unit({
            const SIZE: usize = 128;
            let mut texture = ugli::Texture::new_uninitialized(geng.ugli(), vec2(SIZE, SIZE));
            let mut framebuffer = ugli::Framebuffer::new_color(
                geng.ugli(),
                ugli::ColorAttachment::Texture(&mut texture),
            );
            geng.draw_2d(
                &mut framebuffer,
                &geng::PixelPerfectCamera,
                &draw_2d::Polygon::new_gradient(vec![
                    draw_2d::ColoredVertex {
                        a_pos: vec2(0.0, 0.0),
                        a_color: Rgba::BLACK,
                    },
                    draw_2d::ColoredVertex {
                        a_pos: vec2(SIZE as f32, 0.0),
                        a_color: Rgba::RED,
                    },
                    draw_2d::ColoredVertex {
                        a_pos: vec2(SIZE as f32, SIZE as f32),
                        a_color: Rgba::GREEN,
                    },
                    draw_2d::ColoredVertex {
                        a_pos: vec2(0.0, SIZE as f32),
                        a_color: Rgba::BLUE,
                    },
                ]),
            );
            texture
        }));
        result.add(draw_2d::Ellipse::unit(Rgba::RED));
        result.add(
            draw_2d::Ellipse::unit_with_cut(0.5, Rgba::RED)
                .transform(mat3::rotate(f32::PI / 4.0) * mat3::scale(vec2(1.0, 0.5))),
        );
        result.add(draw_2d::Polygon::new_gradient(vec![
            draw_2d::ColoredVertex {
                a_pos: vec2(-1.0, -1.0),
                a_color: Rgba::RED,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(1.0, -1.0),
                a_color: Rgba::GREEN,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(0.0, 1.0),
                a_color: Rgba::BLUE,
            },
        ]));
        result.add(draw_2d::Polygon::strip(
            vec![
                vec2(-1.0, -1.0),
                vec2(0.0, -1.0),
                vec2(-0.5, 0.0),
                vec2(0.0, 0.0),
                vec2(0.5, 1.0),
                vec2(1.0, 0.5),
            ],
            Rgba::GRAY,
        ));
        result.add(
            draw_2d::Text::unit(geng.default_font().clone(), "Hello!", Rgba::WHITE)
                .transform(mat3::rotate(f32::PI / 6.0)),
        );
        result.add(
            draw_2d::Text::unit(geng.default_font().clone(), "", Rgba::WHITE)
                .transform(mat3::rotate(f32::PI / 6.0)),
        );
        result.add(draw_2d::Segment::new(
            Segment(vec2(-3.0, -5.0), vec2(3.0, 5.0)),
            0.5,
            Rgba::GREEN,
        ));
        result.add(draw_2d::Segment::new_gradient(
            draw_2d::ColoredVertex {
                a_pos: vec2(-5.0, 3.0),
                a_color: Rgba::BLUE,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(5.0, -3.0),
                a_color: Rgba::RED,
            },
            0.5,
        ));
        result.add(draw_2d::Chain::new(
            Chain::new(vec![
                vec2(-5.0, -5.0),
                vec2(5.0, -2.0),
                vec2(-5.0, 2.0),
                vec2(5.0, 5.0),
            ]),
            0.5,
            Rgba::RED,
            5,
        ));
        result.add(draw_2d::Chain::new_gradient(
            vec![
                draw_2d::ColoredVertex {
                    a_pos: vec2(-5.0, -5.0),
                    a_color: Rgba::RED,
                },
                draw_2d::ColoredVertex {
                    a_pos: vec2(5.0, -2.0),
                    a_color: Rgba::GREEN,
                },
                draw_2d::ColoredVertex {
                    a_pos: vec2(-5.0, 2.0),
                    a_color: Rgba::BLUE,
                },
                draw_2d::ColoredVertex {
                    a_pos: vec2(5.0, 5.0),
                    a_color: Rgba::BLACK,
                },
            ],
            0.5,
            5,
        ));
        result.add(draw_2d::Chain::new(
            CardinalSpline::new(
                vec![
                    vec2(-5.0, -5.0),
                    vec2(5.0, -2.0),
                    vec2(-5.0, 2.0),
                    vec2(5.0, 5.0),
                ],
                0.5,
            )
            .chain(10),
            0.5,
            Rgba::RED,
            1,
        ));
        result.add(draw_2d::Chain::new(
            Trajectory::parabola(
                [vec2(0.0, 3.0), vec2(-5.0, -2.0), vec2(5.0, 0.0)],
                -1.0..=1.0,
            )
            .chain(10),
            0.5,
            Rgba::RED,
            1,
        ));
        result.add(draw_2d::Chain::new(
            Trajectory::new(Box::new(|t| vec2(t, t * t * t)), -2.0..=2.0).chain(10),
            0.5,
            Rgba::RED,
            1,
        ));
        result
    }
    fn add<T: draw_2d::Draw2d + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        let mut size = 1;
        while size * size < self.objects.len() {
            size += 1;
        }
        let grid = Grid::new(
            vec2(size, size),
            self.objects.iter().map(|object| object.deref()),
        );
        let framebuffer_size = framebuffer.size();
        self.geng.draw_2d(
            framebuffer,
            &self.camera,
            &grid
                .transformed()
                .fit_into(self.camera.view_area(framebuffer_size.map(|x| x as f32))),
        );
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new("Let's draw!");
    geng.clone().run_loading(async move {
        let assets = geng
            .load_asset(run_dir().join("assets"))
            .await
            .expect("Failed to load assets");
        State::new(&geng, assets)
    });
}
