use super::*;

pub struct ConstraintOverride<T> {
    core: WidgetCore,
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
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn layout_children(&mut self) {
        self.child.core_mut().position = self.core().position;
    }
    fn walk_children_mut<'a>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        f(&mut self.child);
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn fixed_size(self, size: Vec2<f64>) -> ConstraintOverride<Self> {
            let mut core = WidgetCore::void();
            core.constraints.min_size = size;
            core.constraints.flex = vec2(0.0, 0.0);
            ConstraintOverride { core, child: self }
        }
        fn constraints_override(
            self,
            new_constraints: widget::Constraints,
        ) -> ConstraintOverride<Self> {
            let mut core = WidgetCore::void();
            core.constraints = new_constraints;
            ConstraintOverride { core, child: self }
        }
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;
