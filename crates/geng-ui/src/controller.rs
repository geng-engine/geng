use super::*;

struct State {
    size: vec2<f64>,
    scale: f64,
    constraints: HashMap<*const c_void, Constraints>,
    positions: HashMap<*const c_void, Aabb2<f64>>,
    states: Vec<std::cell::UnsafeCell<Box<dyn std::any::Any>>>,
    next_state: usize,
}

impl State {
    fn get_constraints(&self, widget: &dyn Widget) -> Constraints {
        self.constraints[&widget_ptr(widget)]
    }
    fn set_constraints(&mut self, widget: &dyn Widget, constraints: Constraints) {
        self.constraints.insert(widget_ptr(widget), constraints);
    }
    fn get_position(&self, widget: &dyn Widget) -> Aabb2<f64> {
        self.positions[&widget_ptr(widget)]
    }
    fn set_position(&mut self, widget: &dyn Widget, position: Aabb2<f64>) {
        self.positions.insert(widget_ptr(widget), position);
    }
}

fn widget_ptr(widget: &dyn Widget) -> *const c_void {
    widget as *const _ as _
}

pub struct ConstraintsContext<'a> {
    pub theme: &'a Theme,
    state: &'a State,
}

impl ConstraintsContext<'_> {
    pub fn get_constraints(&self, widget: &dyn Widget) -> Constraints {
        self.state.get_constraints(widget)
    }
}

pub struct LayoutContext<'a> {
    pub theme: &'a Theme,
    pub position: Aabb2<f64>,
    state: &'a mut State,
}

impl LayoutContext<'_> {
    pub fn get_constraints(&self, widget: &dyn Widget) -> Constraints {
        self.state.get_constraints(widget)
    }
    pub fn set_position(&mut self, widget: &dyn Widget, position: Aabb2<f64>) {
        self.state.set_position(widget, position);
    }
}

pub struct Controller {
    target_ui_resolution: Option<vec2<f64>>,
    draw2d: draw2d::Helper,
    theme: Theme,
    state: RefCell<State>,
}

impl Controller {
    pub fn new(ugli: &Ugli, theme: Theme, target_ui_resolution: Option<vec2<f64>>) -> Self {
        Self {
            target_ui_resolution,
            draw2d: draw2d::Helper::new(ugli, true),
            theme,
            state: RefCell::new(State {
                size: vec2(1.0, 1.0),
                scale: 1.0,
                constraints: Default::default(),
                positions: Default::default(),
                states: Vec::new(),
                next_state: 0,
            }),
        }
    }

    pub fn draw2d(&self) -> &draw2d::Helper {
        &self.draw2d
    }
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
    pub fn get_state<T: Default + 'static>(&self) -> &mut T {
        self.get_state_with(T::default)
    }
    #[allow(clippy::mut_from_ref)]
    pub fn get_state_with<T: 'static>(&self, f: impl FnOnce() -> T) -> &mut T {
        let mut f = Some(f);
        let mut state = self.state.borrow_mut();
        if state.next_state >= state.states.len() {
            state
                .states
                .push(std::cell::UnsafeCell::new(Box::new(f.take().unwrap()())));
        }
        let current: &mut Box<dyn std::any::Any> =
            unsafe { &mut *state.states[state.next_state].get() };
        if !current.is::<T>() {
            *current = Box::new(f.take().unwrap()());
        }
        state.next_state += 1;
        current.downcast_mut().unwrap()
    }
}

impl Controller {
    pub fn update(&self, root: &mut dyn Widget, delta_time: f64) {
        self.layout(root);
        traverse_mut(root, &mut |widget| widget.update(delta_time), &mut |_| {});
    }
    fn layout(&self, root: &mut dyn Widget) {
        let mut state = self.state.borrow_mut();
        let state = state.deref_mut();
        state.constraints.clear();
        state.positions.clear();
        traverse_mut(root, &mut |_| {}, &mut |widget| {
            let constraints = widget.calc_constraints(&ConstraintsContext {
                theme: &self.theme,
                state,
            });
            state.set_constraints(widget, constraints);
        });
        let root_position = Aabb2::ZERO.extend_positive(state.size);
        state.set_position(root, root_position);
        traverse_mut(
            root,
            &mut |widget| {
                widget.layout_children(&mut LayoutContext {
                    theme: &self.theme,
                    position: state.get_position(widget),
                    state,
                });
            },
            &mut |_| {},
        );
        for position in state.positions.values_mut() {
            *position = position.map(|x| x * state.scale);
        }

        while state.states.len() > state.next_state {
            state.states.pop();
        }
        state.next_state = 0;
    }
    pub fn draw(&self, root: &mut dyn Widget, framebuffer: &mut ugli::Framebuffer) {
        {
            let mut state = self.state.borrow_mut();
            let framebuffer_size = framebuffer.size().map(|x| x as f64);
            state.scale = match self.target_ui_resolution {
                Some(target_size) => {
                    (framebuffer_size.x / target_size.x).max(framebuffer_size.y / target_size.y)
                }
                None => 1.0,
            };
            state.size = framebuffer_size / state.scale;
        }
        self.layout(root);
        let state = self.state.borrow();
        traverse_mut(
            root,
            &mut |widget| {
                widget.draw(&mut DrawContext {
                    draw2d: &self.draw2d,
                    theme: &self.theme,
                    position: state.get_position(widget),
                    framebuffer,
                });
            },
            &mut |_| {},
        );
    }
    pub fn handle_event(&self, root: &mut dyn Widget, event: Event) -> bool {
        let event = &event;
        self.layout(root);
        let state = self.state.borrow_mut();
        let mut captured = false;
        traverse_mut(
            root,
            &mut |widget| {
                let widget_position = state.get_position(widget);
                enum CursorEvent {
                    Move(vec2<f64>),
                    Press(vec2<f64>),
                    Release(vec2<f64>),
                }
                let cursor = match *event {
                    Event::MouseMove { position, .. } => CursorEvent::Move(position),
                    Event::MouseDown { position, .. } => CursorEvent::Press(position),
                    Event::MouseUp { position, .. } => CursorEvent::Release(position),
                    Event::TouchMove(Touch { position, .. }) => CursorEvent::Move(position),
                    Event::TouchStart(Touch { position, .. }) => CursorEvent::Press(position),
                    Event::TouchEnd(Touch { position, .. }) => CursorEvent::Release(position),
                    _ => return,
                };
                match cursor {
                    CursorEvent::Move(position) => {
                        if let Some(sense) = widget.sense() {
                            sense.set_hovered(widget_position.contains(position));
                        }
                        widget.handle_event(event);
                    }
                    CursorEvent::Press(position) => {
                        if widget_position.contains(position) {
                            if let Some(sense) = widget.sense() {
                                sense.set_captured(true);
                                widget.handle_event(event);
                            }
                        } else if let Some(sense) = widget.sense() {
                            if sense.is_captured() {
                                widget.handle_event(event);
                            }
                        }
                    }
                    CursorEvent::Release(position) => {
                        let mut default_sense = Sense::default();
                        let sense = widget.sense().unwrap_or(&mut default_sense);
                        let was_captured = sense.is_captured();
                        sense.set_captured(false);
                        if was_captured && widget_position.contains(position) {
                            sense.click();
                        }
                        if was_captured || widget_position.contains(position) {
                            widget.handle_event(event);
                        }
                    }
                }
            },
            &mut |widget| {
                if let Some(sense) = widget.sense() {
                    if sense.is_captured() {
                        captured = true;
                    }
                }
            },
        );
        captured
    }
}
