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

pub trait LoadAsset: Sized {
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
