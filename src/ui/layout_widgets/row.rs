use super::*;

#[derive(Deref, DerefMut)]
pub struct Row<'a> {
    core: WidgetCore,
    #[deref]
    #[deref_mut]
    children: Vec<Box<dyn Widget + 'a>>,
}

pub fn row<'a>(widgets: Vec<Box<dyn Widget + 'a>>) -> Row<'a> {
    Row {
        core: WidgetCore::void(),
        children: widgets,
    }
}

impl<'a> Widget for Row<'a> {
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn calc_constraints(&mut self) {
        self.core_mut().constraints.min_size.x = self
            .children
            .iter()
            .map(|child| child.core().constraints.min_size.x)
            .sum();
        self.core_mut().constraints.min_size.y = self
            .children
            .iter()
            .map(|child| child.core().constraints.min_size.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        self.core_mut().constraints.flex.x = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.x)
            .sum();
        self.core_mut().constraints.flex.y = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
    }
    fn layout_children(&mut self) {
        let total_flex = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.x)
            .sum::<f64>();
        let size_per_flex = if total_flex == 0.0 {
            0.0
        } else {
            (self.core().position.width()
                - self
                    .children
                    .iter()
                    .map(|child| child.core().constraints.min_size.x)
                    .sum::<f64>())
                / total_flex
        };
        let mut pos = self.core().position.x_min;
        for child in &mut self.children {
            let width = child.core().constraints.min_size.x
                + child.core().constraints.flex.x * size_per_flex;
            child.core_mut().position = AABB::point(vec2(pos, self.core.position.y_min))
                .extend_positive(vec2(width, self.core.position.height()));
            pos += width;
        }
    }
    fn walk_children_mut<'b>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'b>) {
        for child in &mut self.children {
            f(child.deref_mut());
        }
    }
}
