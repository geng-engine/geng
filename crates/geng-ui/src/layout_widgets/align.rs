use super::*;

pub struct Align<T> {
    align: vec2<f64>,
    maintain_aspect: bool,
    flex: vec2<Option<f64>>,
    child: T,
}

impl<T, R: AsRef<T>> AsRef<T> for Align<R> {
    fn as_ref(&self) -> &T {
        self.child.as_ref()
    }
}

impl<T, R: AsMut<T>> AsMut<T> for Align<R> {
    fn as_mut(&mut self) -> &mut T {
        self.child.as_mut()
    }
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn align(self, align: vec2<f64>) -> Align<Self> {
            Align {
                align,
                maintain_aspect: false,
                flex: vec2(None, None),
                child: self,
            }
        }
        fn center(self) -> Align<Self> {
            self.align(vec2(0.5, 0.5))
        }
        fn flex_align(self, flex: vec2<Option<f64>>, align: vec2<f64>) -> Align<Self> {
            Align {
                align,
                maintain_aspect: false,
                flex,
                child: self,
            }
        }
        fn maintain_aspect(self, align: vec2<f64>) -> Align<Self> {
            Align {
                align,
                maintain_aspect: true,
                flex: vec2(None, None),
                child: self,
            }
        }
    }

    impl<T: Widget> WidgetExt for T {}
}

pub use ext::WidgetExt as _;

impl<T: Widget> Widget for Align<T> {
    fn calc_constraints(&mut self, children: &ConstraintsContext) -> Constraints {
        let mut result = children.get_constraints(&self.child);
        if let Some(flex) = self.flex.x {
            result.flex.x = flex;
        }
        if let Some(flex) = self.flex.y {
            result.flex.y = flex;
        }
        result
    }
    fn layout_children(&mut self, cx: &mut LayoutContext) {
        let size = cx.position.size();
        let child_constraints = cx.get_constraints(&self.child);
        let mut child_size = vec2(
            if child_constraints.flex.x == 0.0 {
                partial_min(child_constraints.min_size.x, size.x)
            } else {
                size.x
            },
            if child_constraints.flex.y == 0.0 {
                partial_min(child_constraints.min_size.y, size.y)
            } else {
                size.y
            },
        );
        if self.maintain_aspect && child_constraints.min_size != vec2(0.0, 0.0) {
            let aspect = child_constraints.min_size.x / child_constraints.min_size.y;
            if child_size.y * aspect > child_size.x {
                child_size.y = child_size.x / aspect;
            }
            if child_size.y < child_size.x / aspect {
                child_size.x = child_size.y * aspect;
            }
        }
        cx.set_position(
            &self.child,
            Aabb2::point(
                cx.position.bottom_left()
                    + vec2(
                        (size.x - child_size.x) * self.align.x,
                        (size.y - child_size.y) * self.align.y,
                    ),
            )
            .extend_positive(child_size),
        );
    }
    fn walk_children_mut<'a>(&mut self, f: &mut dyn FnMut(&mut dyn Widget)) {
        f(&mut self.child);
    }
}
