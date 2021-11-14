use super::*;

mod empty;

pub use empty::*;

pub trait ProgressScreen: State {
    fn update_progress(&mut self, progress: f64) {
        #![allow(unused_variables)]
    }
}

impl ProgressScreen for EmptyState {}

pub struct LoadingScreen<T: 'static, L, G>
where
    L: ProgressScreen,
    G: State,
{
    future: Pin<Box<dyn Future<Output = T>>>,
    f: Option<Box<dyn FnOnce(T) -> G>>,
    state: L,
}

impl<T, L, G> LoadingScreen<T, L, G>
where
    L: ProgressScreen,
    G: State,
{
    pub fn new<F: FnOnce(T) -> G + 'static>(
        #[allow(unused_variables)] geng: &Geng,
        state: L,
        future: impl Future<Output = T> + 'static,
        f: F,
    ) -> Self {
        LoadingScreen {
            future: future.boxed_local(),
            f: Some(Box::new(f)),
            state,
        }
    }
}

impl<T, L, G> State for LoadingScreen<T, L, G>
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
    fn transition(&mut self) -> Option<Transition> {
        if self.f.is_some() {
            if let std::task::Poll::Ready(assets) =
                self.future
                    .as_mut()
                    .poll(&mut std::task::Context::from_waker(
                        futures::task::noop_waker_ref(),
                    ))
            {
                let state = (self.f.take().unwrap())(assets);
                return Some(Transition::Switch(Box::new(state)));
            }
        }
        None
    }
    fn ui(&mut self) -> Box<dyn ui::Widget + '_> {
        self.state.ui()
    }
}
