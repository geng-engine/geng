use super::*;

pub struct ConstraintOverride<T> {
    constraints: Constraints,
    child: T,
}

impl<T, R: AsRef<T>> AsRef<T> for ConstraintOverride<R> {
    fn as_ref(&self) -> &T {
        self.child.as_ref()
    }
}

impl<T, R: AsMut<T>> AsMut<T> for ConstraintOverride<R> {
    fn as_mut(&mut self) -> &mut T {
        self.child.as_mut()
    }
}

impl<T: Widget> Widget for ConstraintOverride<T> {
    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        self.constraints
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        cx.set_position(&self.child, cx.position);
    }
    fn walk_children_mut<'a>(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn fixed_size(self, size: vec2<f64>) -> ConstraintOverride<Self> {
            self.constraints_override(Constraints {
                min_size: size,
                flex: vec2(0.0, 0.0),
            })
        }
        fn constraints_override(
            self,
            new_constraints: widget::Constraints,
        ) -> ConstraintOverride<Self> {
            ConstraintOverride {
                constraints: new_constraints,
                child: self,
            }
        }
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;
