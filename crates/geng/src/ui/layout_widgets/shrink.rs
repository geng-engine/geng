use super::*;

pub struct Shrink<T> {
    ratio: f64,
    child: T,
}

impl<T, R: AsRef<T>> AsRef<T> for Shrink<R> {
    fn as_ref(&self) -> &T {
        self.child.as_ref()
    }
}

impl<T, R: AsMut<T>> AsMut<T> for Shrink<R> {
    fn as_mut(&mut self) -> &mut T {
        self.child.as_mut()
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn shrink(self, ratio: f64) -> Shrink<Self> {
            Shrink { ratio, child: self }
        }
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;

impl<T: Widget> Widget for Shrink<T> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        children.get_constraints(&self.child)
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        let ratio = self.ratio / 2.0;
        cx.set_position(
            &self.child,
            AABB {
                x_min: cx.position.x_min + cx.position.width() * ratio,
                x_max: cx.position.x_max - cx.position.width() * ratio,
                y_min: cx.position.y_min + cx.position.height() * ratio,
                y_max: cx.position.y_max - cx.position.height() * ratio,
            },
        );
    }
    fn walk_children_mut<'a>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        f(&mut self.child);
    }
}
