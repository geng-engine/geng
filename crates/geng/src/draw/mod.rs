use super::*;

mod aabb;

pub trait Drawable<Options: Copy> {
    type Camera: ?Sized;
    type Vertex: ugli::Vertex;
    type Instance: ugli::Vertex;
    type Uniforms: ugli::Uniforms;
    fn vertices(&self) -> Vec<Self::Vertex>;
    fn instances(&self) -> Option<Vec<Self::Instance>>;
    fn draw_mode() -> ugli::DrawMode;
    fn draw_parameters(&self, options: Options) -> ugli::DrawParameters;
    fn uniforms(
        &self,
        framebuffer: &ugli::Framebuffer,
        camera: &Self::Camera,
        options: Options,
    ) -> Self::Uniforms;
    fn program(geng: &Geng) -> &ugli::Program;
}

impl Geng {
    pub fn draw<T: Drawable<()>>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &T::Camera,
        object: T,
    ) {
        self.draw_with(framebuffer, camera, object, ());
    }
    pub fn draw_with<Options: Copy, T: Drawable<Options>>(
        &self,
        framebuffer: &mut ugli::Framebuffer,
        camera: &T::Camera,
        object: T,
        options: Options,
    ) {
        let program = T::program(self);
        let draw_mode = T::draw_mode();
        let uniforms = object.uniforms(framebuffer, camera, options);
        let draw_parameters = object.draw_parameters(options);
        let vertices = ugli::VertexBuffer::new_dynamic(self.ugli(), object.vertices());
        if let Some(instances) = object.instances() {
            let instances = ugli::VertexBuffer::new_dynamic(self.ugli(), instances);
            ugli::draw(
                framebuffer,
                program,
                draw_mode,
                ugli::instanced(&vertices, &instances),
                uniforms,
                draw_parameters,
            );
        } else {
            ugli::draw(
                framebuffer,
                program,
                draw_mode,
                &vertices,
                uniforms,
                draw_parameters,
            );
        }
    }
}
