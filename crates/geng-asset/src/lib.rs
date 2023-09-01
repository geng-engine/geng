use batbox_file as file;
use futures::prelude::*;
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
    window: geng_window::Window,
    #[cfg(feature = "audio")]
    audio: geng_audio::Audio,
    shader_lib: shader::Library,
    hot_reload_enabled: bool,
}

#[derive(Clone)]
pub struct Manager {
    inner: Rc<ManagerImpl>,
}

impl Manager {
    pub fn new(
        window: &geng_window::Window,
        #[cfg(feature = "audio")] audio: &geng_audio::Audio,
        hot_reload: bool,
    ) -> Self {
        Self {
            inner: Rc::new(ManagerImpl {
                window: window.clone(),
                ugli: window.ugli().clone(),
                #[cfg(feature = "audio")]
                audio: audio.clone(),
                shader_lib: shader::Library::new(window.ugli(), true, None),
                hot_reload_enabled: hot_reload,
            }),
        }
    }
    pub async fn yield_now(&self) {
        self.inner.window.yield_now().await
    }
    #[cfg(feature = "audio")]
    pub fn audio(&self) -> &geng_audio::Audio {
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
        T::load(self, path.as_ref(), &Default::default())
    }
    pub fn load_with<T: Load>(&self, path: impl AsRef<Path>, options: &T::Options) -> Future<T> {
        T::load(self, path.as_ref(), options)
    }
    /// Load asset from given path with specified or default extension
    pub fn load_ext<T: Load>(
        &self,
        path: impl AsRef<std::path::Path>,
        options: &T::Options,
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
        self.load_with(path, options)
    }
}

pub trait Collection {
    type Item;
}

impl<T> Collection for Vec<T> {
    type Item = T;
}

pub trait Load: Sized + 'static {
    type Options: Clone + Default;
    fn load(manager: &Manager, path: &Path, options: &Self::Options) -> Future<Self>;
    const DEFAULT_EXT: Option<&'static str>;
}

impl<T: 'static> Load for Rc<T>
where
    T: Load,
{
    type Options = T::Options;
    fn load(manager: &Manager, path: &Path, options: &Self::Options) -> Future<Self> {
        let inner = T::load(manager, path, options);
        async move { Ok(Rc::new(inner.await?)) }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = T::DEFAULT_EXT;
}

impl Load for ugli::Program {
    type Options = ();
    fn load(manager: &Manager, path: &Path, _options: &Self::Options) -> Future<Self> {
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
    type Options = ();
    fn load(manager: &Manager, path: &Path, _options: &Self::Options) -> Future<Self> {
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
    type Options = ();
    fn load(_manager: &Manager, path: &Path, _options: &Self::Options) -> Future<Self> {
        let path = path.to_owned();
        async move { file::load_string(&path).await }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("txt");
}

impl Load for Vec<u8> {
    type Options = ();
    fn load(_manager: &Manager, path: &Path, _options: &Self::Options) -> Future<Self> {
        let path = path.to_owned();
        async move { file::load_bytes(&path).await }.boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = None;
}

impl Load for geng_font::Font {
    type Options = geng_font::Options;
    fn load(manager: &Manager, path: &Path, options: &Self::Options) -> Future<Self> {
        let manager = manager.clone();
        let path = path.to_owned();
        let options = options.clone();
        async move {
            let data = file::load_bytes(path).await?;
            geng_font::Font::new(manager.ugli(), &data, &options)
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
mod audio_ {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct SoundOptions {
        pub looped: bool,
    }

    #[allow(clippy::derivable_impls)]
    impl Default for SoundOptions {
        fn default() -> Self {
            Self { looped: false }
        }
    }

    impl Load for geng_audio::Sound {
        type Options = SoundOptions;
        fn load(
            manager: &Manager,
            path: &std::path::Path,
            options: &Self::Options,
        ) -> Future<Self> {
            let manager = manager.clone();
            let path = path.to_owned();
            let options = options.clone();
            Box::pin(async move {
                let mut sound = manager.audio().load(path).await?;
                sound.set_looped(options.looped);
                Ok(sound)
            })
        }
        const DEFAULT_EXT: Option<&'static str> = Some("wav"); // TODO change to mp3 since wav doesnt work in safari?
    }
}

#[derive(Debug, Clone)]
pub struct TextureOptions {
    pub filter: ugli::Filter,
    pub wrap_mode: ugli::WrapMode,
    pub premultiply_alpha: bool,
}

impl Default for TextureOptions {
    fn default() -> Self {
        Self {
            filter: ugli::Filter::Linear,
            wrap_mode: ugli::WrapMode::Clamp,
            premultiply_alpha: false,
        }
    }
}

impl Load for ugli::Texture {
    type Options = TextureOptions;
    fn load(manager: &Manager, path: &std::path::Path, options: &Self::Options) -> Future<Self> {
        let manager = manager.clone();
        let path = path.to_owned();
        let options = options.clone();
        async move {
            let mut texture = platform::load_texture(&manager, &path, &options).await?;
            texture.set_filter(options.filter);
            texture.set_wrap_mode(options.wrap_mode);
            Ok(texture)
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("png");
}
