use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod _impl;
#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod _impl;

pub(crate) use _impl::*;

pub type AssetFuture<T> = Pin<Box<dyn Future<Output = Result<T, anyhow::Error>>>>;

pub trait LoadAsset: Sized {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self>;
    const DEFAULT_EXT: Option<&'static str>;
}

impl<T: 'static> LoadAsset for Rc<T>
where
    T: LoadAsset,
{
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        let future = T::load(geng, path);
        Box::pin(async move { Ok(Rc::new(future.await?)) })
    }
    const DEFAULT_EXT: Option<&'static str> = T::DEFAULT_EXT;
}
