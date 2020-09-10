use super::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod _impl;
#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod _impl;

pub(crate) use _impl::*;

#[async_trait(?Send)]
pub trait LoadAsset: Sized {
    // TODO: non 'static args?
    async fn load(geng: Rc<Geng>, path: String) -> Result<Self, anyhow::Error>;
    fn default_ext() -> Option<&'static str>;
}

#[async_trait(?Send)]
impl<T: 'static> LoadAsset for Rc<T>
where
    T: LoadAsset,
{
    async fn load(geng: Rc<Geng>, path: String) -> Result<Self, anyhow::Error> {
        Ok(Rc::new(T::load(geng, path).await?))
    }
    fn default_ext() -> Option<&'static str> {
        T::default_ext()
    }
}
