use super::*;

mod empty;

pub use empty::*;

pub trait ProgressScreen: State {
    fn update_progress(&mut self, progress: f64) {
        #![allow(unused_variables)]
    }
}

impl ProgressScreen for state::Empty {}

pub struct LoadingScreen<L, G>
where
    L: ProgressScreen,
    G: State,
{
    future: Pin<Box<dyn Future<Output = G>>>,
    state: L,
}

impl<L, G> LoadingScreen<L, G>
where
    L: ProgressScreen,
    G: State,
{
    pub fn new<F: Future<Output = G> + 'static>(
        #[allow(unused_variables)] geng: &Geng,
        state: L,
        future: F,
    ) -> Self {
        LoadingScreen {
            future: future.boxed_local(),
            state,
        }
    }
}

impl<L, G> State for LoadingScreen<L, G>
where
    L: ProgressScreen,
    G: State,
{
    fn update(&mut self, delta_time: f64) {
        self.state.update(delta_time);
        // TODO: state.update_progress(future.progress().unwrap());
    }
    fn fixed_update(&mut self, delta_time: f64) {
        self.state.fixed_update(delta_time);
    }
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.state.draw(framebuffer);
    }
    fn handle_event(&mut self, event: Event) {
        self.state.handle_event(event);
    }
    fn transition(&mut self) -> Option<state::Transition> {
        if let std::task::Poll::Ready(state) =
            self.future
                .as_mut()
                .poll(&mut std::task::Context::from_waker(
                    futures::task::noop_waker_ref(),
                ))
        {
            return Some(state::Transition::Switch(Box::new(state)));
        }
        None
    }
    fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> Box<dyn ui::Widget + 'a> {
        self.state.ui(cx)
    }
}
