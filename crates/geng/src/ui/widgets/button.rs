use super::*;

pub struct Button<'a> {
    sense: &'a mut Sense,
    inner: Box<dyn Widget + 'a>,
    f: Box<dyn FnMut() + 'a>,
}

impl<'a> Button<'a> {
    pub fn new(cx: &'a Controller, text: &str, f: impl FnMut() + 'a) -> Self {
        let sense: &'a mut Sense = cx.get_state();
        let text = Text::new(
            text.to_owned(),
            cx.theme().font.clone(),
            cx.theme().text_size,
            if sense.is_hovered() {
                cx.theme().hover_color
            } else {
                cx.theme().usable_color
            },
        )
        .shrink(if sense.is_captured() {
            cx.theme().press_ratio as f64
        } else {
            0.0
        });
        let mut ui = ui::stack![text];
        if sense.is_hovered() {
            ui.push(Box::new(
                ColorBox::new(cx.theme().hover_color)
                    .constraints_override(Constraints {
                        min_size: vec2(0.0, 1.0),
                        flex: vec2(1.0, 0.0),
                    })
                    .flex_align(vec2(Some(0.0), Some(0.0)), vec2(0.5, 0.0)),
            ));
        }
        Self {
            sense,
            inner: Box::new(ui),
            f: Box::new(f),
        }
    }
}

impl Widget for Button<'_> {
    fn sense(&mut self) -> Option<&mut Sense> {
        Some(self.sense)
    }
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        cx.get_constraints(&self.inner)
    }
    fn walk_children_mut(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + '_>) {
        f(&mut self.inner);
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        cx.set_position(&self.inner, cx.position);
    }
    fn handle_event(&mut self, event: &Event) {
        #![allow(unused_variables)]
        if self.sense.was_clicked() {
            (self.f)();
        }
    }
}
