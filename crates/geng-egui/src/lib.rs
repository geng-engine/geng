pub use egui;

use geng::prelude::*;

mod painter;

use painter::*;

pub struct EguiGeng {
    geng: Geng,
    egui_ctx: egui::CtxRef,
    egui_input: egui::RawInput,
    painter: Painter,
    shapes: Option<Vec<egui::epaint::ClippedShape>>,
}

impl EguiGeng {
    pub fn new(geng: &Geng) -> Self {
        Self {
            geng: geng.clone(),
            egui_ctx: egui::CtxRef::default(),
            egui_input: egui::RawInput::default(),
            painter: Painter::new(),
            shapes: None,
        }
    }

    pub fn get_context(&self) -> &egui::CtxRef {
        &self.egui_ctx
    }

    pub fn begin_frame(&mut self) {
        // TODO: gather input
        self.egui_ctx.begin_frame(self.egui_input.take());
    }

    pub fn end_frame(&mut self) {
        let (output, shapes) = self.egui_ctx.end_frame();
        if self.shapes.is_some() {
            eprintln!("Egui contents not drawn. You need to call `draw` after calling `end_frame`");
        }
        self.shapes = Some(shapes);

        // TODO: process output
    }

    pub fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        if let Some(shapes) = self.shapes.take() {
            let paint_jobs = self.egui_ctx.tessellate(shapes);
            self.painter
                .paint(&self.geng, paint_jobs, &self.egui_ctx.texture());
        } else {
            eprintln!("Failed to draw egui. You need to call `end_frame` before calling `draw`");
        }
    }
}
