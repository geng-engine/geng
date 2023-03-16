use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod _impl;
#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod _impl;

#[allow(unused_imports)]
pub(crate) use _impl::*;

pub type AssetFuture<T> = Pin<Box<dyn Future<Output = Result<T, anyhow::Error>>>>;

pub trait LoadAsset: Sized + 'static {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self>;
    const DEFAULT_EXT: Option<&'static str>;
}

impl<T: 'static> LoadAsset for Rc<T>
where
    T: LoadAsset,
{
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let future = T::load(geng, path);
        Box::pin(async move { Ok(Rc::new(future.await?)) })
    }
    const DEFAULT_EXT: Option<&'static str> = T::DEFAULT_EXT;
}

impl LoadAsset for ugli::Program {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let glsl = <String as LoadAsset>::load(geng, path);
        let geng = geng.clone();
        async move {
            let glsl = glsl.await?;
            geng.shader_lib().compile(&glsl)
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("glsl");
}

impl LoadAsset for serde_json::Value {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let string = <String as LoadAsset>::load(geng, path);
        async move {
            let string = string.await?;
            Ok(serde_json::from_str(&string)?)
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("json");
}

impl LoadAsset for String {
    fn load(_: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let path = path.to_owned();
        async move { file::load_string(&path).await }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("txt");
}

impl LoadAsset for Vec<u8> {
    fn load(_: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let path = path.to_owned();
        async move {
            let mut buf = Vec::new();
            file::load(&path).await?.read_to_end(&mut buf).await?;
            Ok(buf)
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = None;
}

impl LoadAsset for Font {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let path = path.to_owned();
        let geng = geng.clone();
        async move {
            let data = file::load_bytes(path).await?;
            Ok(Font::new(&geng, &data, default())?)
        }
        .boxed_local()
    }

    const DEFAULT_EXT: Option<&'static str> = Some("ttf");
}

#[derive(Debug)]
pub(crate) struct LoadProgress {
    pub progress: usize,
    pub total: usize,
}

impl LoadProgress {
    pub fn new() -> Self {
        Self {
            progress: 0,
            total: 0,
        }
    }
}

impl Geng {
    pub fn load_asset<T: LoadAsset>(&self, path: impl AsRef<std::path::Path>) -> AssetFuture<T> {
        let geng = self.clone();
        geng.inner.load_progress.borrow_mut().total += 1;
        T::load(self, path.as_ref())
            .map(move |result| {
                geng.inner.load_progress.borrow_mut().progress += 1;
                result
            })
            .boxed_local()
    }

    pub fn load_asset_ext<T: LoadAsset>(
        &self,
        path: impl AsRef<std::path::Path>,
        ext: Option<impl AsRef<str>>,
    ) -> AssetFuture<T> {
        match ext.as_ref().map(|s| s.as_ref()).or(T::DEFAULT_EXT) {
            Some(ext) => self.load_asset(path.as_ref().with_extension(ext)),
            None => self.load_asset(path),
        }
    }

    pub fn set_loading_progress_title(&self, title: &str) {
        // TODO: native
        #[cfg(target_arch = "wasm32")]
        {
            #[wasm_bindgen(inline_js = r#"
            export function set_progress_title(title) {
                window.gengUpdateProgressTitle(title);
            }
            "#)]
            extern "C" {
                fn set_progress_title(title: &str);
            }
            set_progress_title(title);
        }
    }

    pub fn set_loading_progress(&self, progress: f64, total: Option<f64>) {
        // TODO: native
        #[cfg(target_arch = "wasm32")]
        {
            #[wasm_bindgen(inline_js = r#"
            export function set_progress(progress, total) {
                window.gengUpdateProgress(progress, total);
            }
            "#)]
            extern "C" {
                fn set_progress(progress: f64, total: Option<f64>);
            }
            set_progress(progress, total);
        }
    }

    pub fn finish_loading(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            #[wasm_bindgen(inline_js = r#"
            export function finish_loading() {
                document.getElementById("geng-progress-screen").style.display = "none";
                document.getElementById("geng-canvas").style.display = "block";
            }
            "#)]
            extern "C" {
                fn finish_loading();
            }
            finish_loading();
        }
    }
}

pub struct Hot<T> {
    current: RefCell<T>,
    geng: Geng,
    path: std::path::PathBuf,
    need_update: Arc<std::sync::atomic::AtomicBool>,
    update: RefCell<Option<AssetFuture<T>>>,
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)] // This is here for delaying the drop of the watcher
    watcher: notify::RecommendedWatcher,
}

impl<T: LoadAsset> Hot<T> {
    pub fn get(&self) -> Ref<T> {
        if let Ok(mut current) = self.current.try_borrow_mut() {
            let mut update = self.update.borrow_mut();
            if let Some(future) = update.deref_mut() {
                if let std::task::Poll::Ready(result) = future.as_mut().poll(
                    &mut std::task::Context::from_waker(futures::task::noop_waker_ref()),
                ) {
                    *update = None;
                    match result {
                        Ok(new) => *current = new,
                        Err(e) => error!("{e}"),
                    }
                    self.need_update
                        .store(false, std::sync::atomic::Ordering::SeqCst);
                }
            } else if self.need_update.load(std::sync::atomic::Ordering::SeqCst) {
                *update = Some(self.geng.load_asset(&self.path))
            }
        }
        self.current.borrow()
    }
}

impl<T: LoadAsset> LoadAsset for Hot<T> {
    fn load(geng: &Geng, path: &std::path::Path) -> AssetFuture<Self> {
        let geng = geng.clone();
        let path = path.to_owned();
        async move {
            let need_update = Arc::new(std::sync::atomic::AtomicBool::new(false));
            #[cfg(not(target_arch = "wasm32"))]
            let watcher = {
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
                info!("watching {path:?}");
                watcher
            };
            let initial = geng.load_asset(&path).await?;
            Ok(Self {
                need_update,
                geng: geng.clone(),
                path,
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
