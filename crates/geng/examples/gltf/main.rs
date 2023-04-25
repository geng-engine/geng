use geng::prelude::*;

#[derive(geng::asset::Load)]
struct Assets {
    shader: ugli::Program,
}

#[derive(ugli::Vertex)]
struct Vertex {
    a_uv: vec2<f32>,
    a_mr_uv: vec2<f32>,
    a_pos: vec3<f32>,
    a_normal: vec3<f32>,
    a_color: Rgba<f32>,
}

pub struct Camera {
    pub fov: f32,
    pub pos: vec3<f32>,
    pub distance: f32,
    pub rot_h: f32,
    pub rot_v: f32,
}

impl Camera {
    pub fn eye_pos(&self) -> vec3<f32> {
        let v = vec2(self.distance, 0.0).rotate(self.rot_v);
        self.pos + vec3(0.0, -v.y, v.x)
    }
}

impl geng::AbstractCamera3d for Camera {
    fn view_matrix(&self) -> mat4<f32> {
        mat4::translate(vec3(0.0, 0.0, -self.distance))
            * mat4::rotate_x(-self.rot_v)
            * mat4::rotate_z(-self.rot_h)
            * mat4::translate(-self.pos)
    }

    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat4<f32> {
        mat4::perspective(
            self.fov,
            framebuffer_size.x / framebuffer_size.y,
            0.1,
            1000.0,
        )
    }
}

struct Material {
    base_color_texture: ugli::Texture,
    base_color_factor: Rgba<f32>,
    metallic_roughness_texture: ugli::Texture,
    metallic_factor: f32,
    roughness_factor: f32,
    // TODO: normal texture
    // TODO: occlusion texture
    // TODO: emissive texture
}

impl Material {
    fn uniforms(&self) -> impl ugli::Uniforms + '_ {
        ugli::uniforms! {
            u_base_color_texture: &self.base_color_texture,
            u_base_color_factor: self.base_color_factor,
            u_metallic_roughness_texture: &self.metallic_roughness_texture,
            u_metallic_factor: self.metallic_factor,
            u_roughness_factor: self.roughness_factor,
        }
    }
}

struct Mesh {
    data: ugli::VertexBuffer<Vertex>,
    material: Material,
}

struct Example {
    time: f32,
    meshes: Vec<Mesh>,
    geng: Geng,
    assets: Rc<Assets>,
    camera: Camera,
    transition: Rc<RefCell<Option<geng::state::Transition>>>,
}

impl Example {
    fn new(geng: Geng, assets: Rc<Assets>, gltf: Vec<u8>) -> Self {
        let (document, buffers, _images) = gltf::import_slice(&gltf).unwrap();
        let mut meshes = Vec::new();
        for mesh in document.meshes() {
            log::info!("{:?}", mesh.name());
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(|x| &**x));
                let positions: Vec<vec3<f32>> = reader
                    .read_positions()
                    .expect("No positions for primitive mesh WAT")
                    .map(|[x, y, z]| vec3(x, y, z))
                    .collect();
                let normals: Vec<vec3<f32>> = reader
                    .read_normals()
                    .expect("Missing normals, this is not supported yet")
                    .map(|[x, y, z]| vec3(x, y, z))
                    .collect();
                let colors: Option<Vec<Rgba<f32>>> = reader.read_colors(0).map(|colors| {
                    colors
                        .into_rgba_f32()
                        .map(|[r, g, b, a]| Rgba::new(r, g, b, a))
                        .collect()
                });
                let indices = reader
                    .read_indices()
                    .expect("Absent indices not supported yet")
                    .into_u32()
                    .map(|x| x as usize);
                assert_eq!(primitive.mode(), gltf::mesh::Mode::Triangles);
                let data = ugli::VertexBuffer::new_static(
                    geng.ugli(),
                    indices
                        .map(|index| Vertex {
                            a_mr_uv: vec2::ZERO, // TODO
                            a_uv: vec2::ZERO,    // TODO
                            a_pos: positions[index],
                            a_normal: normals[index], // TODO: optional
                            a_color: colors.as_ref().map_or(Rgba::WHITE, |colors| colors[index]),
                        })
                        .collect(),
                );
                let material = {
                    let material = primitive.material();
                    let white_texture =
                        || ugli::Texture::new_with(geng.ugli(), vec2(1, 1), |_| Rgba::WHITE);
                    Material {
                        base_color_texture: white_texture(), // TODO material.pbr_metallic_roughness().base_color_texture()
                        base_color_factor: {
                            let [r, g, b, a] =
                                material.pbr_metallic_roughness().base_color_factor();
                            Rgba::new(r, g, b, a)
                        },
                        metallic_roughness_texture: white_texture(), // TODO
                        metallic_factor: material.pbr_metallic_roughness().metallic_factor(),
                        roughness_factor: material.pbr_metallic_roughness().roughness_factor(),
                    }
                };
                meshes.push(Mesh { data, material });
            }
        }
        Self {
            time: 0.0,
            meshes,
            geng,
            assets,
            camera: Camera {
                fov: f32::PI / 3.0,
                pos: vec3(0.0, 0.0, 1.0),
                distance: 5.0,
                rot_h: 0.0,
                rot_v: f32::PI / 3.0,
            },
            transition: default(),
        }
    }
}

impl geng::State for Example {
    fn update(&mut self, delta_time: f64) {
        let _delta_time = delta_time as f32;
        // self.time += delta_time;
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size().map(|x| x as f32);
        ugli::clear(framebuffer, Some(Rgba::BLACK), Some(1.0), None);
        for mesh in &self.meshes {
            ugli::draw(
                framebuffer,
                &self.assets.shader,
                ugli::DrawMode::Triangles,
                &mesh.data,
                (
                    mesh.material.uniforms(),
                    ugli::uniforms! {
                        u_model_matrix: mat4::rotate_z(self.time), // TODO
                        u_eye_pos: self.camera.eye_pos(),
                        u_light_dir: vec3(1.0, -2.0, 5.0),
                        u_light_color: Rgba::WHITE,
                        u_ambient_light_color: Rgba::WHITE,
                        u_ambient_light_intensity: 0.1,
                    },
                    self.camera.uniforms(framebuffer_size),
                ),
                ugli::DrawParameters {
                    depth_func: Some(ugli::DepthFunc::Less),
                    ..default()
                },
            );
        }
    }
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseMove { delta, .. } => {
                if !self.geng.window().pressed_buttons().is_empty() {
                    let sense = 0.01;
                    self.camera.rot_h += delta.x as f32 * sense;
                    self.camera.rot_v =
                        (self.camera.rot_v + delta.y as f32 * sense).clamp(0.0, f32::PI);
                }
            }

            geng::Event::KeyDown { key: geng::Key::S }
                if self.geng.window().is_key_pressed(geng::Key::LCtrl) =>
            {
                file_dialog::save("test.txt", "Hello, world!".as_bytes()).unwrap();
            }
            geng::Event::KeyDown { key: geng::Key::O }
                if self.geng.window().is_key_pressed(geng::Key::LCtrl) =>
            {
                let geng = self.geng.clone();
                let assets = self.assets.clone();
                let transition = self.transition.clone();
                file_dialog::select(move |file| {
                    *transition.borrow_mut() = Some(geng::state::Transition::Switch(Box::new(
                        load(geng, assets, async move {
                            let mut reader = file.reader().unwrap();
                            let mut buf = Vec::new();
                            reader.read_to_end(&mut buf).await.unwrap();
                            buf
                        }),
                    )))
                });
            }
            _ => {}
        }
    }
    fn transition(&mut self) -> Option<geng::state::Transition> {
        self.transition.borrow_mut().take()
    }
}

fn load(
    geng: Geng,
    assets: Rc<Assets>,
    gltf: impl Future<Output = Vec<u8>> + 'static,
) -> impl geng::State {
    geng::LoadingScreen::new(
        &geng.clone(),
        geng::EmptyLoadingScreen::new(&geng.clone()),
        async move { Example::new(geng, assets, gltf.await) },
    )
}

#[derive(clap::Parser)]
struct Opt {
    path: Option<std::path::PathBuf>,
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let opt: Opt = cli::parse();
    let geng = Geng::new("Example");
    let path = opt
        .path
        .unwrap_or(run_dir().join("assets").join("crab.glb"));
    geng.clone().run_loading(async move {
        let assets = geng
            .asset_manager()
            .load(run_dir().join("assets"))
            .await
            .expect("Failed to load assets");
        let assets = Rc::new(assets);
        load(
            geng,
            assets,
            file::load_bytes(path).map(|result| result.unwrap()),
        )
    });
}
