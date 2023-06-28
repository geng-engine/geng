//! Utility stuff

/// Construct default value of a type
pub fn default<T: Default>() -> T {
    T::default()
}

/// Find the directory where the program is running from
///
/// If running via cargo, returns the directory of the crate (`CARGO_MANIFEST_DIR`).
/// If running an example, this will be joined by `examples/<name>`
///
/// If running without cargo, this returns parent of [std::env::current_exe]
///
/// On the web this just returns `.`
pub fn run_dir() -> std::path::PathBuf {
    #[cfg(target_os = "android")]
    return match batbox_android::file_mode() {
        batbox_android::FileMode::FileSystem => batbox_android::app().external_data_path().unwrap(),
        batbox_android::FileMode::Assets => std::path::PathBuf::from("."),
    };
    if cfg!(target_arch = "wasm32") {
        return std::path::PathBuf::from(".");
    }
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(dir);
        let current_exe = std::env::current_exe().unwrap();
        if let Some(binary_type) = current_exe.parent() {
            if binary_type.file_name().unwrap() == "examples" {
                path = path.join("examples");
                if let Some(bin_name) = std::env::var_os("CARGO_BIN_NAME") {
                    path = path.join(bin_name);
                } else {
                    path = path.join(current_exe.file_stem().unwrap());
                }
            }
        }
        if path.is_dir() {
            return path;
        } else {
            log::warn!("run_dir was expected to be {path:?} but its not a valid directory path");
        }
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                return path.to_owned();
            }
        }
    }
    std::env::current_dir().unwrap()
}
