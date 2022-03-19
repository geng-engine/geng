use super::*;

#[derive(Deref, DerefMut)]
pub struct Stack<'a> {
    #[deref]
    #[deref_mut]
    children: Vec<Box<dyn Widget + 'a>>,
}

pub fn stack<'a>(widgets: Vec<Box<dyn Widget + 'a>>) -> Stack<'a> {
    Stack { children: widgets }
}

impl<'a> Widget for Stack<'a> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        Constraints {
            min_size: Vec2 {
                x: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.x)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
                y: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).min_size.y)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            },
            flex: Vec2 {
                x: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.x)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
                y: self
                    .children
                    .iter()
                    .map(|child| children.get_constraints(child.deref()).flex.y)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            },
        }
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        for child in &self.children {
            cx.set_position(child.deref(), cx.position);
        }
    }
    fn walk_children_mut<'b>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'b>) {
        for child in &mut self.children {
            f(child.deref_mut());
        }
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt<'a>: Widget + Sized + 'a {
        fn background(self, other: impl Widget + 'a) -> Stack<'a> {
            stack![self, other]
        }
    }

    impl<'a, T: Widget + 'a> WidgetExt<'a> for T {}

    macro_rules! impl_for_tuple {
        ($($a:ident),*) => {
            impl<'a, $($a: Widget + 'a),*> TupleExt<'a> for ($($a,)*) {
                fn stack(self) -> Stack<'a> {
                    let ($($a,)*) = self;
                    ui::stack![$($a),*]
                }
            }
        };
    }
    impl_tuples!(impl_for_tuple);

    pub trait TupleExt<'a> {
        fn stack(self) -> Stack<'a>;
    }
}

pub use ext::{TupleExt as _, WidgetExt as _};
