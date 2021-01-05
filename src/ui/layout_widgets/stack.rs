use super::*;

#[derive(Deref, DerefMut)]
pub struct Stack<'a> {
    core: WidgetCore,
    #[deref]
    #[deref_mut]
    children: Vec<Box<dyn Widget + 'a>>,
}

pub fn stack<'a>(widgets: Vec<Box<dyn Widget + 'a>>) -> Stack<'a> {
    Stack {
        core: WidgetCore::void(),
        children: widgets,
    }
}

impl<'a> Widget for Stack<'a> {
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
            .unwrap();
        self.core_mut().constraints.min_size.y = self
            .children
            .iter()
            .map(|child| child.core().constraints.min_size.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        self.core_mut().constraints.flex.x = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        self.core_mut().constraints.flex.y = self
            .children
            .iter()
            .map(|child| child.core().constraints.flex.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
    }
    fn layout_children(&mut self) {
        for child in &mut self.children {
            child.core_mut().position = self.core.position;
        }
    }
    fn walk_children_mut<'b>(&mut self, mut f: Box<dyn FnMut(&mut dyn Widget) + 'b>) {
        for child in &mut self.children {
            f(child.deref_mut());
        }
    }
}
