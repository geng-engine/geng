use super::*;

#[derive(Deref, DerefMut)]
pub struct Row<'a> {
    #[deref]
    #[deref_mut]
    children: Vec<Box<dyn Widget + 'a>>,
}

pub fn row<'a>(widgets: Vec<Box<dyn Widget + 'a>>) -> Row<'a> {
    Row { children: widgets }
}

#[macro_export]
macro_rules! row {
    ($($x:expr),* $(,)?) => {
        $crate::row(vec![$(Box::new($x)),*])
    };
}

impl<'a> Widget for Row<'a> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: vec2(
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.x)
                    .sum(),
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.y)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
            ),
            flex: vec2(
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.x)
                    .sum(),
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.y)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
            ),
        }
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        let total_flex = self
            .children
            .iter()
            .map(|child| cx.get_constraints(child.deref()).flex.x)
            .sum::<f64>();
        let size_per_flex = if total_flex == 0.0 {
            0.0
        } else {
            (cx.position.width()
                - self
                    .children
                    .iter()
                    .map(|child| cx.get_constraints(child.deref()).min_size.x)
                    .sum::<f64>())
                / total_flex
        };
        let mut pos = cx.position.min.x;
        for child in &self.children {
            let child = child.deref();
            let width = cx.get_constraints(child).min_size.x
                + cx.get_constraints(child).flex.x * size_per_flex;
            cx.set_position(
                child,
                Aabb2::point(vec2(pos, cx.position.min.y))
                    .extend_positive(vec2(width, cx.position.height())),
            );
            pos += width;
        }
    }
    fn walk_children_mut<'b>(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        for child in &mut self.children {
            f(child.deref_mut());
        }
    }
}

mod ext {
    use super::*;

    macro_rules! impl_for_tuple {
        ($($a:ident),*) => {
            impl<'a, $($a: Widget + 'a),*> TupleExt<'a> for ($($a,)*) {
                fn row(self) -> Row<'a> {
                    let ($($a,)*) = self;
                    row![$($a),*]
                }
            }
        };
    }
    call_for_tuples!(impl_for_tuple);

    pub trait TupleExt<'a> {
        fn row(self) -> Row<'a>;
    }
}

pub use ext::TupleExt as _;
