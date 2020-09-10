use super::*;

pub struct Align<T> {
    core: WidgetCore,
    align: Vec2<f64>,
    maintain_aspect: bool,
    flex: Vec2<Option<f64>>,
    child: T,
}

mod ext {
    use super::*;

    pub trait WidgetExt: Widget + Sized {
        fn align(self, align: Vec2<f64>) -> Align<Self> {
            Align {
                core: WidgetCore::void(),
                align,
                maintain_aspect: false,
                flex: vec2(None, None),
                child: self,
            }
        }
        fn center(self) -> Align<Self> {
            self.center()
        }
        fn flex_align(self, flex: Vec2<Option<f64>>, align: Vec2<f64>) -> Align<Self> {
            Align {
                core: WidgetCore::void(),
                align,
                maintain_aspect: false,
                flex,
                child: self,
            }
        }
        fn maintain_aspect(self, align: Vec2<f64>) -> Align<Self> {
            Align {
                core: WidgetCore::void(),
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
    fn core(&self) -> &WidgetCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut WidgetCore {
        &mut self.core
    }
    fn calc_constraints(&mut self) {
        self.core.constraints = self.child.core().constraints;
        if let Some(flex) = self.flex.x {
            self.core.constraints.flex.x = flex;
        }
        if let Some(flex) = self.flex.y {
            self.core.constraints.flex.y = flex;
        }
    }
    fn layout_children(&mut self) {
        let size = self.core().position.size();
        let mut child_size = vec2(
            if self.child.core().constraints.flex.x == 0.0 {
                partial_min(self.child.core().constraints.min_size.x, size.x)
            } else {
                size.x
            },
            if self.child.core().constraints.flex.y == 0.0 {
                partial_min(self.child.core().constraints.min_size.y, size.y)
            } else {
                size.y
            },
        );
        if self.maintain_aspect && self.child.core().constraints.min_size != vec2(0.0, 0.0) {
            let aspect =
                self.child.core().constraints.min_size.x / self.child.core().constraints.min_size.y;
            if child_size.y * aspect > child_size.x {
                child_size.y = child_size.x / aspect;
            }
            if child_size.y < child_size.x / aspect {
                child_size.x = child_size.y * aspect;
            }
        }
        self.child.core_mut().position = AABB::pos_size(
            self.core().position.bottom_left()
                + vec2(
                    (size.x - child_size.x) * self.align.x,
                    (size.y - child_size.y) * self.align.y,
                ),
            child_size,
        );
    }
    fn walk_children_mut<'a>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'a>) {
        f(&mut self.child);
    }
}
