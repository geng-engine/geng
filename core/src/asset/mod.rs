use crate::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod _impl;
#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod _impl;

pub(crate) use _impl::*;

pub type AssetFuture<T> = Pin<Box<dyn Future<Output = Result<T, Error>>>>;

pub trait LoadAsset: Sized {
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self>;
}

impl LoadAsset for () {
    fn load(_: &Rc<Geng>, _: &str) -> AssetFuture<()> {
        unimplemented!()
    }
}

impl<T: 'static> LoadAsset for Rc<T>
where
    T: LoadAsset,
{
    fn load(geng: &Rc<Geng>, path: &str) -> AssetFuture<Self> {
        T::load(geng, path).map(|t| Ok(Rc::new(t?))).boxed_local()
    }
}
