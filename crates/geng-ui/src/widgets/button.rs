use super::*;

pub struct Button<'a> {
    sense: &'a mut Sense,
    clicked: bool,
    inner: Box<dyn Widget + 'a>,
    f: Option<Box<dyn FnMut() + 'a>>,
}

impl<'a> Button<'a> {
    pub fn new(cx: &'a Controller, text: &str) -> Self {
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
        let mut ui = stack![text];
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
            clicked: sense.take_clicked(),
            sense,
            inner: Box::new(ui),
            f: None,
        }
    }
    pub fn was_clicked(&self) -> bool {
        self.clicked
    }
}

impl Widget for Button<'_> {
    fn sense(&mut self) -> Option<&mut Sense> {
        Some(self.sense)
    }
    fn calc_constraints(&mut self, cx: &ConstraintsContext) -> Constraints {
        cx.get_constraints(&self.inner)
    }
    fn walk_children_mut(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.inner);
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        cx.set_position(&self.inner, cx.position);
    }
    fn handle_event(&mut self, event: &Event) {
        #![allow(unused_variables)]
        if let Some(f) = &mut self.f {
            if self.sense.take_clicked() {
                f();
            }
        }
    }
}
