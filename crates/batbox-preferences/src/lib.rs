//! Saving/loading preferences (small amounts of user data)
//!
//! Web implementation uses [local storage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)

use serde::{de::DeserializeOwned, Serialize};

/// Base path where preferences are going to be saved/loaded from
pub fn base_path() -> std::path::PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        ".".into() // TODO: detect app name by url?
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let exe = std::env::current_exe().expect("Failed to find current exe");
        let app_name = exe.file_stem().unwrap();
        if let Some(dirs) =
            directories::ProjectDirs::from("", "", app_name.to_str().expect("Exe name is invalid"))
        {
            return dirs.preference_dir().to_path_buf();
        }
        if let Some(dir) = exe.parent() {
            return dir.to_path_buf();
        }
        std::env::current_dir().unwrap()
    }
}

/// Save given value for given key
pub fn save<T: Serialize>(key: &str, value: &T) {
    let base_path = base_path();
    let path = base_path.join(key);
    #[cfg(target_arch = "wasm32")]
    {
        let path = path.to_str().unwrap();
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            if let Err(e) = storage.set_item(
                path,
                &serde_json::to_string(value).expect("Failed to serialize"),
            ) {
                let _ = e; // TODO: error?
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::io::Write;
        if let Err(e) = std::fs::create_dir_all(&base_path) {
            log::error!("Failed to create preferences base path: {}", e);
            return;
        }
        let path = &path;
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(e) => {
                log::error!("Failed to create {:?}: {}", path, e);
                return;
            }
        };
        if let Err(e) = file.write_all(serde_json::to_string_pretty(value).unwrap().as_bytes()) {
            log::error!("Failed to save {:?}: {}", path, e);
        }
    }
}

/// Load value for given key
pub fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let base_path = base_path();
    let path = base_path.join(key);
    #[cfg(target_arch = "wasm32")]
    {
        let path = path.to_str().unwrap();
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            match storage
                .get_item(path)
                .ok()
                .flatten()
                .map(|s| serde_json::from_str(&s))
            {
                Some(Ok(value)) => Some(value),
                Some(Err(e)) => {
                    log::error!("Failed to deserialize {:?}: {}", path, e);
                    None
                }
                None => None,
            }
        } else {
            None
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = &path;
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                log::warn!("Failed to open {:?}: {}", path, e);
                return None;
            }
        };
        match serde_json::from_reader(file) {
            Ok(value) => {
                log::debug!("Successfully loaded {:?}", path);
                Some(value)
            }
            Err(e) => {
                log::error!("Failed to deserialize {:?}: {}", path, e);
                None
            }
        }
    }
}
