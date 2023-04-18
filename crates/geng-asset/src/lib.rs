use batbox_file as file;
use futures::prelude::*;
#[cfg(feature = "audio")]
use geng_audio::{self as audio, Audio};
use geng_shader as shader;
use std::cell::RefCell;
use std::future::Future as StdFuture;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use ugli::Ugli;

pub mod hot;
mod platform;

pub use hot::Hot;

pub use geng_asset_derive::*;

pub type Future<T> = Pin<Box<dyn StdFuture<Output = anyhow::Result<T>>>>;

struct ManagerImpl {
    ugli: Ugli,
    #[cfg(feature = "audio")]
    audio: Audio,
    shader_lib: shader::Library,
    hot_reload_enabled: bool,
}

#[derive(Clone)]
pub struct Manager {
    inner: Rc<ManagerImpl>,
}

impl Manager {
    pub fn new(ugli: &Ugli, #[cfg(feature = "audio")] audio: &Audio, hot_reload: bool) -> Self {
        Self {
            inner: Rc::new(ManagerImpl {
                ugli: ugli.clone(),
                #[cfg(feature = "audio")]
                audio: audio.clone(),
                shader_lib: shader::Library::new(ugli, true, None),
                hot_reload_enabled: hot_reload,
            }),
        }
    }
    #[cfg(feature = "audio")]
    pub fn audio(&self) -> &Audio {
        &self.inner.audio
    }
    pub fn ugli(&self) -> &Ugli {
        &self.inner.ugli
    }
    pub fn shader_lib(&self) -> &shader::Library {
        &self.inner.shader_lib
    }
    pub fn hot_reload_enabled(&self) -> bool {
        self.inner.hot_reload_enabled
    }
    pub fn load<T: Load>(&self, path: impl AsRef<Path>) -> Future<T> {
        T::load(self, path.as_ref())
    }
    pub fn load_ext<T: Load>(
        &self,
        path: impl AsRef<std::path::Path>,
        ext: Option<impl AsRef<str>>,
    ) -> Future<T> {
        let path = path.as_ref();
        let path_buf_tmp;
        let path = match ext.as_ref().map(|s| s.as_ref()).or(T::DEFAULT_EXT) {
            Some(ext) => {
                path_buf_tmp = path.with_extension(ext);
                &path_buf_tmp
            }
            None => path,
        };
        self.load(path)
    }
}

pub trait Load: Sized + 'static {
    fn load(manager: &Manager, path: &Path) -> Future<Self>;
    const DEFAULT_EXT: Option<&'static str>;
}

impl<T: 'static> Load for Rc<T>
where
    T: Load,
{
    fn load(manager: &Manager, path: &Path) -> Future<Self> {
        let inner = T::load(manager, path);
        async move { Ok(Rc::new(inner.await?)) }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = T::DEFAULT_EXT;
}

impl Load for ugli::Program {
    fn load(manager: &Manager, path: &Path) -> Future<Self> {
        let glsl: Future<String> = manager.load(path);
        let manager = manager.clone();
        async move {
            let glsl: String = glsl.await?;
            manager.shader_lib().compile(&glsl)
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("glsl");
}

impl Load for serde_json::Value {
    fn load(manager: &Manager, path: &Path) -> Future<Self> {
        let string: Future<String> = manager.load(path);
        async move {
            let string: String = string.await?;
            Ok(serde_json::from_str(&string)?)
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("json");
}

impl Load for String {
    fn load(_manager: &Manager, path: &Path) -> Future<Self> {
        let path = path.to_owned();
        async move { file::load_string(&path).await }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("txt");
}

impl Load for Vec<u8> {
    fn load(_manager: &Manager, path: &Path) -> Future<Self> {
        let path = path.to_owned();
        async move { file::load_bytes(&path).await }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = None;
}

impl Load for geng_font::Font {
    fn load(manager: &Manager, path: &Path) -> Future<Self> {
        let manager = manager.clone();
        let path = path.to_owned();
        async move {
            let data = file::load_bytes(path).await?;
            geng_font::Font::new(manager.ugli(), &data, geng_font::Options::default())
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("ttf");
}

#[derive(Debug)]
pub struct LoadProgress {
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

impl Default for LoadProgress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "audio")]
impl Load for audio::Sound {
    fn load(manager: &Manager, path: &std::path::Path) -> Future<Self> {
        let manager = manager.clone();
        let path = path.to_owned();
        Box::pin(async move { manager.audio().load(path).await })
    }
    const DEFAULT_EXT: Option<&'static str> = Some("wav"); // TODO change to mp3 since wav doesnt work in safari?
}

impl Load for ugli::Texture {
    fn load(manager: &Manager, path: &std::path::Path) -> Future<Self> {
        platform::load_texture(manager, path)
    }
    const DEFAULT_EXT: Option<&'static str> = Some("png");
}
