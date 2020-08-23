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
    fn default_ext() -> Option<&'static str>;
}

impl LoadAsset for () {
    fn load(_: &Rc<Geng>, _: &str) -> AssetFuture<()> {
        unimplemented!()
    }
    fn default_ext() -> Option<&'static str> {
        None
    }
}

impl<T: 'static> LoadAsset for Rc<T>
where
    T: LoadAsset,
{
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        T::load(geng, path).map(|t| Ok(Rc::new(t?))).boxed_local()
    }
    fn default_ext() -> Option<&'static str> {
        T::default_ext()
    }
}
