use geng::prelude::*;

#[derive(geng::Assets)]
struct Assets {
    texture: ugli::Texture,
}

struct Grid<'a> {
    table: Vec<Vec<Option<&'a dyn geng::Draw2d>>>,
    transform: Mat3<f32>,
}

impl<'a> Grid<'a> {
    pub fn new(size: Vec2<usize>, objects: impl IntoIterator<Item = &'a dyn geng::Draw2d>) -> Self {
        let mut table = vec![vec![None; size.y]; size.x];
        for (storage, object) in table.iter_mut().flat_map(|row| row.iter_mut()).zip(objects) {
            *storage = Some(object);
        }
        Self {
            table,
            transform: Mat3::scale(size.map(|x| x as f32)),
        }
    }
}

impl<'a> Transform2d for Grid<'a> {
    fn bounding_quad(&self) -> Quad<f32> {
        Quad::from_matrix(self.transform)
    }
    fn apply_transform(&mut self, transform: Mat3<f32>) {
        self.transform = transform * self.transform;
    }
}

impl<'a> geng::Draw2d for Grid<'a> {
    fn draw_2d_transformed(
        &self,
        geng: &Geng,
        framebuffer: &mut ugli::Framebuffer,
        camera: &dyn geng::AbstractCamera2d,
        transform: Mat3<f32>,
    ) {
        for (x, column) in self.table.iter().enumerate() {
            for (y, object) in column.iter().enumerate() {
                if let Some(object) = *object {
                    geng.draw_2d_transformed(
                        framebuffer,
                        camera,
                        &object
                            .transformed()
                            .fit_into(AABB::point(Vec2::ZERO).extend_uniform(0.9))
                            .transform(
                                self.transform
                                    * Mat3::translate(vec2(-1.0, -1.0))
                                    * Mat3::scale_uniform(2.0)
                                    * Mat3::scale(vec2(
                                        1.0 / self.table.len() as f32,
                                        1.0 / self.table[0].len() as f32,
                                    ))
                                    * Mat3::translate(vec2(x as f32, y as f32))
                                    * Mat3::scale_uniform(0.5)
                                    * Mat3::translate(vec2(1.0, 1.0)),
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
                center: Vec2::ZERO,
                rotation: 0.0,
                fov: 10.0,
            },
            objects: vec![],
        };
        result.add(
            draw_2d::Quad::unit(Color::WHITE)
                .transform(Mat3::rotate(0.5) * Mat3::scale_uniform(0.5)),
        );
        result.add(draw_2d::TexturedQuad::unit(ugli::Texture::new_with(
            geng.ugli(),
            vec2(2, 2),
            |pos| match (pos.x, pos.y) {
                (0, 0) => Color::BLACK,
                (1, 0) => Color::RED,
                (1, 1) => Color::GREEN,
                (0, 1) => Color::BLUE,
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
                        a_color: Color::BLACK,
                    },
                    draw_2d::ColoredVertex {
                        a_pos: vec2(SIZE as f32, 0.0),
                        a_color: Color::RED,
                    },
                    draw_2d::ColoredVertex {
                        a_pos: vec2(SIZE as f32, SIZE as f32),
                        a_color: Color::GREEN,
                    },
                    draw_2d::ColoredVertex {
                        a_pos: vec2(0.0, SIZE as f32),
                        a_color: Color::BLUE,
                    },
                ]),
            );
            texture
        }));
        result.add(draw_2d::Ellipse::unit(Color::RED));
        result.add(
            draw_2d::Ellipse::unit_with_cut(0.5, Color::RED)
                .transform(Mat3::rotate(f32::PI / 4.0) * Mat3::scale(vec2(1.0, 0.5))),
        );
        result.add(draw_2d::Polygon::new_gradient(vec![
            draw_2d::ColoredVertex {
                a_pos: vec2(-1.0, -1.0),
                a_color: Color::RED,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(1.0, -1.0),
                a_color: Color::GREEN,
            },
            draw_2d::ColoredVertex {
                a_pos: vec2(0.0, 1.0),
                a_color: Color::BLUE,
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
            Color::GRAY,
        ));
        result.add(
            draw_2d::Text::unit(geng.default_font().clone(), "Hello!", Color::WHITE)
                .transform(Mat3::rotate(f32::PI / 6.0)),
        );
        result.add(
            draw_2d::Text::unit(geng.default_font().clone(), "", Color::WHITE)
                .transform(Mat3::rotate(f32::PI / 6.0)),
        );
        result
    }
    fn add<T: draw_2d::Draw2d + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl geng::State for State {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);

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
    logger::init().unwrap();
    // Setup working directory
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("examples/draw/static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }
    let geng = Geng::new("Let's draw!");
    let state = geng::LoadingScreen::new(
        &geng,
        geng::EmptyLoadingScreen,
        geng::LoadAsset::load(&geng, "."),
        {
            let geng = geng.clone();
            move |assets| State::new(&geng, assets.unwrap())
        },
    );
    geng::run(&geng, state)
}
