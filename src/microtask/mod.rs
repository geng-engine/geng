use crate::*;

pub fn spawn<F: FnOnce() + Send + 'static>(task: F) {
    #[cfg(target_arch = "wasm32")]
    {
        static QUEUE: once_cell::sync::Lazy<
            Mutex<std::collections::VecDeque<Box<dyn FnOnce() + Send>>>,
        > = once_cell::sync::Lazy::new(|| todo!());
        fn callback() {
            let timer = Timer::new();
            while let Some(task) = QUEUE.lock().unwrap().pop_front() {
                task();
                if timer.elapsed() > 0.001 {
                    break;
                }
            }
        }
        QUEUE.lock().unwrap().push_back(Box::new(task));
    }
    #[cfg(not(target_arch = "wasm32"))]
    global_threadpool().execute(task);
}
