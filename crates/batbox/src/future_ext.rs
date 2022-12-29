#[allow(unused_imports)]
use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub trait ThreadPoolExt {
    fn spawn<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        f: F,
    ) -> futures::channel::oneshot::Receiver<T>;
}

#[cfg(not(target_arch = "wasm32"))]
impl ThreadPoolExt for ThreadPool {
    fn spawn<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        f: F,
    ) -> futures::channel::oneshot::Receiver<T> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        self.execute(move || {
            if sender.send(f()).is_err() {
                panic!("Failed to send value");
            }
        });
        receiver
    }
}
