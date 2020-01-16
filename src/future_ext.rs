use crate::*;

pub trait ThreadPoolExt {
    fn spawn<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        f: F,
    ) -> futures::channel::oneshot::Receiver<T>;
}

impl ThreadPoolExt for ThreadPool {
    fn spawn<T: Send + 'static, F: FnOnce() -> T + Send + 'static>(
        &self,
        f: F,
    ) -> futures::channel::oneshot::Receiver<T> {
        let (sender, receiver) = futures::channel::oneshot::channel();
        self.execute(move || {
            if let Err(_) = sender.send(f()) {
                panic!("Failed to send value");
            }
        });
        receiver
    }
}

pub mod ext {
    pub use super::ThreadPoolExt as _;
}
