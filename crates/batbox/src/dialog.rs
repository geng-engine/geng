#[allow(unused_imports)]
use super::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn save_file<F: FnOnce(&mut (dyn Write + Send)) -> std::io::Result<()>>(
    title: &str,
    default_path: &str,
    f: F,
) -> std::io::Result<()> {
    if let Some(path) = tinyfiledialogs::save_file_dialog(title, default_path) {
        f(&mut std::io::BufWriter::new(std::fs::File::create(path)?))?;
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn select_file(title: &str) -> Option<std::path::PathBuf> {
    tinyfiledialogs::open_file_dialog(title, "", None).map(|path| path.into())
}
