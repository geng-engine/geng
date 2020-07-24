use super::*;

pub struct AutoSave<T: Serialize> {
    value: T,
    path: String,
    changed: Cell<bool>,
}

impl<T: Serialize + for<'de> Deserialize<'de> + Default> AutoSave<T> {
    pub fn load(path: &str) -> Self {
        Self {
            value: load(path).unwrap_or_else(|| {
                let value = default();
                save(path, &value);
                value
            }),
            path: path.to_owned(),
            changed: Cell::new(false),
        }
    }
}

impl<T: Serialize> AutoSave<T> {
    pub fn save(&self) {
        save(&self.path, &self.value);
    }
}

impl<T: Serialize> Deref for AutoSave<T> {
    type Target = T;
    fn deref(&self) -> &T {
        if self.changed.get() {
            self.changed.set(false);
            self.save();
        }
        &self.value
    }
}

impl<T: Serialize> DerefMut for AutoSave<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.changed.set(true);
        &mut self.value
    }
}

impl<T: Serialize> Drop for AutoSave<T> {
    fn drop(&mut self) {
        self.save();
    }
}

fn save<T: Serialize>(path: &str, value: &T) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            storage.set_item(
                path,
                &serde_json::to_string(value).expect("Failed to serialize"),
            );
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut file = match std::fs::File::create(path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create {:?}: {}", path, e);
                return;
            }
        };
        if let Err(e) = file.write_all(
            ron::ser::to_string_pretty(value, default())
                .unwrap()
                .as_bytes(),
        ) {
            error!("Failed to save {:?}: {}", path, e);
        }
    }
}

fn load<T: for<'de> Deserialize<'de>>(path: &str) -> Option<T> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            match storage
                .get_item(path)
                .ok()
                .flatten()
                .map(|s| serde_json::from_str(&s))
            {
                Some(Ok(value)) => Some(value),
                Some(Err(e)) => {
                    error!("Failed to deserialize {:?}: {}", path, e);
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
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                warn!("Failed to open {:?}: {}", path, e);
                return None;
            }
        };
        match ron::de::from_reader(file) {
            Ok(value) => {
                info!("Successfully loaded {:?}", path);
                Some(value)
            }
            Err(e) => {
                error!("Failed to deserialize {:?}: {}", path, e);
                None
            }
        }
    }
}
