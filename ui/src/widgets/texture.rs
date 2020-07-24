use super::*;

pub struct Texture<'a> {
    geng: Rc<Geng>,
    core: WidgetCore,
    texture: &'a ugli::Texture,
}

impl<'a> Texture<'a> {
    pub fn new(geng: &Rc<Geng>, texture: &'a ugli::Texture) -> Self {
        Self {
            geng: geng.clone(),
            core: WidgetCore::void(),
            texture,
        }
    }
}

impl<'a> Widget for Texture<'a> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.geng.draw_2d().textured_quad(
            framebuffer,
            self.core().position.map(|x| x as f32),
            self.texture,
            Color::WHITE,
        );
    }
}
