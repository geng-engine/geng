use super::*;

pub struct Hot<T> {
    current: RefCell<T>,
    manager: Manager,
    path: PathBuf,
    need_update: Arc<std::sync::atomic::AtomicBool>,
    update: RefCell<Option<Future<T>>>,
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)] // This is here for delaying the drop of the watcher
    watcher: Option<notify::RecommendedWatcher>,
}

pub type Ref<'a, T> = std::cell::Ref<'a, T>;

impl<T: Load> Hot<T> {
    pub fn get(&self) -> Ref<T> {
        if let Ok(mut current) = self.current.try_borrow_mut() {
            let mut update = self.update.borrow_mut();
            if let Some(future) = &mut *update {
                if let std::task::Poll::Ready(result) = future.as_mut().poll(
                    &mut std::task::Context::from_waker(futures::task::noop_waker_ref()),
                ) {
                    *update = None;
                    match result {
                        Ok(new) => *current = new,
                        Err(e) => log::error!("{e}"),
                    }
                    self.need_update
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                }
            } else if self.need_update.load(std::sync::atomic::Ordering::SeqCst) {
                *update = Some(self.manager.load(&self.path).boxed_local())
            }
        }
        self.current.borrow()
    }
}

impl<T: Load> Load for Hot<T> {
    fn load(manager: &Manager, path: &Path) -> Future<Self> {
        let manager = manager.clone();
        let path = path.to_owned();
        let need_update = Arc::new(std::sync::atomic::AtomicBool::new(false));
        #[cfg(not(target_arch = "wasm32"))]
        let watcher = if manager.hot_reload_enabled() {
            use notify::Watcher;
            let need_update = need_update.clone();
            let mut watcher =
                notify::recommended_watcher(move |result: notify::Result<notify::Event>| {
                    let event = result.unwrap();
                    if event.kind.is_modify() {
                        need_update.store(true, std::sync::atomic::Ordering::SeqCst);
                    }
                })
                .unwrap();
            watcher
                .watch(&path, notify::RecursiveMode::Recursive)
                .unwrap();
            log::info!("watching {path:?}");
            Some(watcher)
        } else {
            None
        };
        async move {
            let initial = manager.load(&path).await?;
            Ok(Self {
                need_update,
                manager: manager.clone(),
                path: path.to_owned(),
                current: RefCell::new(initial),
                update: RefCell::new(None),
                #[cfg(not(target_arch = "wasm32"))]
                watcher,
            })
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = T::DEFAULT_EXT;
}
