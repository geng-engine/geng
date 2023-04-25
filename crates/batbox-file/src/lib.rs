//! Loading files
//!
//! Since [std::fs] is not working on the web, use this for consistency

use anyhow::anyhow;
use futures::prelude::*;
use serde::de::DeserializeOwned;
#[cfg(not(target_arch = "wasm32"))]
use std::pin::Pin;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;

/// Load a file at given path, returning an async reader as result
///
/// Supports both files and urls
pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<impl AsyncBufRead> {
    let path = path.as_ref();
    #[cfg(target_arch = "wasm32")]
    {
        let fetch: JsFuture = web_sys::window()
            .expect("window unavailable")
            .fetch_with_str(path.to_str().expect("path is not a valid str"))
            .into();
        let response: web_sys::Response = match fetch.await {
            Ok(response) => response.unchecked_into(),
            Err(e) => anyhow::bail!("{e:?}"),
        };
        let status = http::StatusCode::from_u16(response.status())?;
        if !status.is_success() {
            anyhow::bail!("Http status: {status}");
        }
        let body = response.body().expect("response without body?");
        Ok(futures::io::BufReader::new(read_stream(body)))
    }
    #[cfg(not(target_arch = "wasm32"))]
    match path
        .to_str()
        .and_then(|path| url::Url::parse(path).ok())
        .filter(|url| matches!(url.scheme(), "http" | "https"))
    {
        Some(url) => {
            log::debug!("{:?}", url.scheme());
            let request = reqwest::get(url);
            let request = async_compat::Compat::new(request); // Because of tokio inside reqwest
            let response = request.await?;
            let status = response.status();
            if !status.is_success() {
                anyhow::bail!("Http status: {status}");
            }
            let reader = response
                .bytes_stream()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                .into_async_read();
            let reader = futures::io::BufReader::new(reader);
            Ok(Box::pin(reader) as Pin<Box<dyn AsyncBufRead>>)
        }
        None => {
            let file = async_std::fs::File::open(path).await?;
            let reader = futures::io::BufReader::new(file);
            Ok(Box::pin(reader) as Pin<Box<dyn AsyncBufRead>>)
        }
    }
}

/// Turns web_sys::ReadableStream into AsyncRead
#[cfg(target_arch = "wasm32")]
pub fn read_stream(stream: web_sys::ReadableStream) -> impl AsyncRead {
    let stream = wasm_streams::ReadableStream::from_raw(stream.unchecked_into());

    fn js_to_string(js_value: &JsValue) -> Option<String> {
        js_value.as_string().or_else(|| {
            js_sys::Object::try_from(js_value)
                .map(|js_object| js_object.to_string().as_string().unwrap_throw())
        })
    }
    fn js_to_io_error(js_value: JsValue) -> std::io::Error {
        let message = js_to_string(&js_value).unwrap_or_else(|| "Unknown error".to_string());
        std::io::Error::new(std::io::ErrorKind::Other, message)
    }

    // TODO: BYOB not supported, not working, wot?
    // let reader = stream.into_async_read();
    stream
        .into_stream()
        .map(|result| match result {
            Ok(chunk) => Ok(chunk.unchecked_into::<js_sys::Uint8Array>().to_vec()),
            Err(e) => Err(js_to_io_error(e)),
        })
        .into_async_read()
}

/// Load file as a vec of bytes
pub async fn load_bytes(path: impl AsRef<std::path::Path>) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    load(path).await?.read_to_end(&mut buf).await?;
    Ok(buf)
}

/// Load file as a string
pub async fn load_string(path: impl AsRef<std::path::Path>) -> anyhow::Result<String> {
    let mut buf = String::new();
    load(path).await?.read_to_string(&mut buf).await?;
    Ok(buf)
}

/// Load file and deserialize into given type using deserializer based on extension
///
/// Supports:
/// - json
/// - toml
/// - ron
pub async fn load_detect<T: DeserializeOwned>(
    path: impl AsRef<std::path::Path>,
) -> anyhow::Result<T> {
    let path = path.as_ref();
    let ext = path
        .extension()
        .ok_or(anyhow!("Expected to have extension"))?;
    let ext = ext.to_str().ok_or(anyhow!("Extension is not valid str"))?;
    let data = load_bytes(path).await?;
    let value = match ext {
        "json" => serde_json::from_reader(data.as_slice())?,
        "toml" => toml::from_slice(&data)?,
        "ron" => ron::de::from_bytes(&data)?,
        _ => anyhow::bail!("{ext:?} is unsupported"),
    };
    Ok(value)
}

/// Load json file and deserialize into given type
pub async fn load_json<T: DeserializeOwned>(
    path: impl AsRef<std::path::Path>,
) -> anyhow::Result<T> {
    let json: String = load_string(path).await?;
    let value = serde_json::from_str(&json)?;
    Ok(value)
}
