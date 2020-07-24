use super::*;

pub struct Column<'a> {
    core: WidgetCore,
    children: Vec<Box<dyn Widget + 'a>>,
}

impl<'a> Deref for Column<'a> {
    type Target = Vec<Box<dyn Widget + 'a>>;
    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl DerefMut for Column<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

pub fn column<'a>(widgets: Vec<Box<dyn Widget + 'a>>) -> Column<'a> {
    Column {
        core: WidgetCore::void(),
        children: widgets,
    }
}

#[macro_export]
macro_rules! column {
    ($($x:expr,)*) => {
        $crate::column(vec![$(Box::new($x)),*])
    };
    ($($x:expr),*) => {
        $crate::column!($($x,)*)
    };
}

impl<'a> Widget for Column<'a> {
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
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        self.core_mut().constraints.min_size.y = self
            .children
            .iter()
            .map(|child| child.core().constraints.min_size.y)
            .sum();
        self.core_mut().constraints.flex.x = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);
        self.core_mut().constraints.flex.y = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.y)
            .sum();
    }
    fn layout_children(&mut self) {
        let total_flex = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.y)
            .sum::<f64>();
        let size_per_flex = if total_flex == 0.0 {
            0.0
        } else {
            (self.core().position.height()
                - self
                    .children
                    .iter()
                    .map(|child| child.core().constraints.min_size.y)
                    .sum::<f64>())
                / total_flex
        };
        let mut pos = self.core().position.y_max;
        for child in &mut self.children {
            let height = child.core().constraints.min_size.y
                + child.core().constraints.flex.y * size_per_flex;
            pos -= height;
            child.core_mut().position = AABB::pos_size(
                vec2(self.core.position.x_min, pos),
                vec2(self.core.position.width(), height),
            );
        }
    }
    fn walk_children_mut<'b>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'b>) {
        for child in &mut self.children {
            f(child.deref_mut());
        }
    }
}
