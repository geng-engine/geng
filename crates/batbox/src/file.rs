//! Loading files
//!
//! Since [std::fs] is not working on the web, use this for consistency
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    pub use crate::file;
}

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
            info!("{:?}", url.scheme());
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

#[cfg(target_arch = "wasm32")]
fn read_stream(stream: web_sys::ReadableStream) -> impl AsyncRead {
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

/// Load json file and deserialize into given type
pub async fn load_json<T: DeserializeOwned>(
    path: impl AsRef<std::path::Path>,
) -> anyhow::Result<T> {
    let json: String = load_string(path).await?;
    let value = serde_json::from_str(&json)?;
    Ok(value)
}

/// A file selected using [select] dialog
pub struct SelectedFile {
    #[cfg(target_arch = "wasm32")]
    file: web_sys::File,
    #[cfg(not(target_arch = "wasm32"))]
    path: std::path::PathBuf,
}

impl SelectedFile {
    /// Find out the name of the file
    pub fn name(&self) -> std::ffi::OsString {
        #[cfg(target_arch = "wasm32")]
        {
            self.file.name().into()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.path.file_name().expect("no filename").to_owned()
        }
    }

    /// Get file path
    ///
    /// Not available on the web
    #[cfg(not(target_arch = "wasm32"))]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Get reader for the file
    pub fn reader(self) -> anyhow::Result<impl AsyncBufRead> {
        #[cfg(target_arch = "wasm32")]
        {
            Ok(futures::io::BufReader::new(read_stream(self.file.stream())))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(load(self.path))
        }
    }
}

/// Show a select file dialog
///
/// The callback may be called at any moment in the future.
/// The callback will not be called if user cancels the dialog.
/// On the web this will only work if called during user interaction.
// TODO: filter
pub fn select(title: &str, callback: impl FnOnce(SelectedFile) + 'static) {
    #[cfg(target_arch = "wasm32")]
    {
        let _title = title;
        let input: web_sys::HtmlInputElement = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document")
            .create_element("input")
            .expect("failed to create input")
            .unchecked_into();
        input.set_type("file");
        input.set_onchange(Some(
            Closure::once_into_js({
                let input = input.clone();
                move || {
                    let Some(files) = input.files() else { return };
                    let Some(file) = files.get(0) else { return };
                    let file = SelectedFile { file };
                    callback(file);
                }
            })
            .unchecked_ref(),
        ));
        input.click();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // TODO migrate to rfd maybe?
        if let Some(path) = tinyfiledialogs::open_file_dialog(title, "", None) {
            let file = SelectedFile { path: path.into() };
            callback(file);
        }
    }
}

// TODO
#[cfg(not(target_arch = "wasm32"))]
pub fn save<F: FnOnce(&mut (dyn Write + Send)) -> std::io::Result<()>>(
    title: &str,
    default_path: &str,
    f: F,
) -> std::io::Result<()> {
    if let Some(path) = tinyfiledialogs::save_file_dialog(title, default_path) {
        f(&mut std::io::BufWriter::new(std::fs::File::create(path)?))?;
    }
    Ok(())
}
