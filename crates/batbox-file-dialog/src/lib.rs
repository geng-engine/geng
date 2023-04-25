//! File dialogs

use futures::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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
            Ok(futures::io::BufReader::new(batbox_file::read_stream(
                self.file.stream(),
            )))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            futures::executor::block_on(batbox_file::load(self.path))
        }
    }
}

/// Show a select file dialog
///
/// The callback may be called at any moment in the future.
/// The callback will not be called if user cancels the dialog.
/// On the web this will only work if called during user interaction.
// TODO: filter
pub fn select(callback: impl FnOnce(SelectedFile) + 'static) {
    #[cfg(target_arch = "wasm32")]
    {
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
        if let Some(path) = tinyfiledialogs::open_file_dialog("Select", "", None) {
            let file = SelectedFile { path: path.into() };
            callback(file);
        }
    }
}

/// Show a save file dialog and write the data into the file
pub fn save(file_name: &str, data: &[u8]) -> anyhow::Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let data = web_sys::Blob::new_with_u8_array_sequence(&js_sys::Array::of1(
            &js_sys::Uint8Array::from(data), // TODO: no copy?
        ))
        .expect("failed to create blob");
        let a: web_sys::HtmlAnchorElement = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document")
            .create_element("a")
            .expect("failed to create <a> element")
            .unchecked_into();
        let url =
            web_sys::Url::create_object_url_with_blob(&data).expect("failed to create blob url");
        a.set_href(&url);
        a.set_download(file_name);
        // TODO force "Save As"? Currently no dialog appearing
        a.click();
        web_sys::Url::revoke_object_url(&a.href()).expect("failed to revoke url");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(path) = tinyfiledialogs::save_file_dialog("Save", file_name) {
            let file = std::fs::File::create(path)?;
            let mut writer = std::io::BufWriter::new(file);
            std::io::Write::write_all(&mut writer, data)?;
        }
    }
    Ok(())
}
