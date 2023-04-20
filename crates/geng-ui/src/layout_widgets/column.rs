use super::*;

#[derive(Deref, DerefMut)]
pub struct Column<'a> {
    #[deref]
    #[deref_mut]
    children: Vec<Box<dyn Widget + 'a>>,
}

pub fn column<'a>(widgets: Vec<Box<dyn Widget + 'a>>) -> Column<'a> {
    Column { children: widgets }
}

#[macro_export]
macro_rules! column {
    ($($x:expr),* $(,)?) => {
        $crate::column(vec![$(Box::new($x)),*])
    };
}

impl<'a> Widget for Column<'a> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: vec2(
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.x)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.y)
                    .sum(),
            ),
            flex: vec2(
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.x)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
                self.children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.y)
                    .sum(),
            ),
        }
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        let total_flex = self
            .children
            .iter()
            .map(|child| cx.get_constraints(child.deref()).flex.y)
            .sum::<f64>();
        let size_per_flex = if total_flex == 0.0 {
            0.0
        } else {
            (cx.position.height()
                - self
                    .children
                    .iter()
                    .map(|child| cx.get_constraints(child.deref()).min_size.y)
                    .sum::<f64>())
                / total_flex
        };
        let mut pos = cx.position.max.y;
        for child in &self.children {
            let child = child.deref();
            let height = cx.get_constraints(child).min_size.y
                + cx.get_constraints(child).flex.y * size_per_flex;
            pos -= height;
            cx.set_position(
                child,
                Aabb2::point(vec2(cx.position.min.x, pos))
                    .extend_positive(vec2(cx.position.width(), height)),
            );
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
                fn column(self) -> Column<'a> {
                    let ($($a,)*) = self;
                    super::column![$($a),*]
                }
            }
        };
    }
    call_for_tuples!(impl_for_tuple);

    pub trait TupleExt<'a> {
        fn column(self) -> Column<'a>;
    }
}

pub use ext::TupleExt as _;
