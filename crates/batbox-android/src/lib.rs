#![cfg(target_os = "android")]

use std::path::PathBuf;

pub type App = android_activity::AndroidApp;

static APP: std::sync::OnceLock<App> = std::sync::OnceLock::new();

pub fn init(app: App) {
    APP.set(app).unwrap();
}

pub fn app() -> &'static App {
    APP.get().expect("Android app was not set")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FileMode {
    FileSystem,
    Assets,
}

static FILE_MODE: std::sync::OnceLock<std::sync::Mutex<FileMode>> = std::sync::OnceLock::new();

fn file_mode_mutex() -> &'static std::sync::Mutex<FileMode> {
    FILE_MODE.get_or_init(|| std::sync::Mutex::new(FileMode::Assets))
}

pub fn set_file_mode(file_mode: FileMode) {
    *file_mode_mutex().lock().unwrap() = file_mode;
}

pub fn file_mode() -> FileMode {
    *file_mode_mutex().lock().unwrap()
}

mod make_it_work {
    use jni::{JNIEnv, JavaVM};
    use std::ffi::c_void;

    #[no_mangle]
    pub extern "C" fn JNI_OnLoad(
        vm: jni::JavaVM,
        res: *mut std::os::raw::c_void,
    ) -> jni::sys::jint {
        // Wait for debugger to connect OMEGALUL
        std::thread::sleep(std::time::Duration::from_secs(3));
        let env = vm.get_env().unwrap();
        let vm = vm.get_java_vm_pointer() as *mut c_void;
        unsafe {
            ndk_context::initialize_android_context(vm, res);
        }
        jni::JNIVersion::V6.into()
    }

    #[no_mangle]
    pub unsafe extern "C" fn __cxa_pure_virtual() {
        loop {}
    }
}

pub fn copy_assets_to_filesystem(
    asset_dirs: impl IntoIterator<Item = impl AsRef<std::path::Path>>,
    destination: impl AsRef<std::path::Path>,
) {
    copy(asset_dirs, destination).unwrap();

    use std::{
        error::Error,
        ffi::{CStr, CString},
        fs, io,
        path::{Path, PathBuf},
    };

    use jni::{
        objects::{JObject, JObjectArray, JValueGen},
        JNIEnv, JavaVM,
    };
    use ndk::asset::Asset;

    pub type CopyResult<T> = Result<T, Box<dyn Error>>;

    pub fn copy(
        asset_dirs: impl IntoIterator<Item = impl AsRef<Path>>,
        destination: impl AsRef<Path>,
    ) -> CopyResult<()> {
        // Create a VM for executing Java calls
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;

        // Query the Asset Manager
        let asset_manager = env
            .call_method(
                unsafe { JObject::from_raw(ctx.context().cast()) },
                "getAssets",
                "()Landroid/content/res/AssetManager;",
                &[],
            )?
            .l()?;

        // Copy assets
        for asset_dir in asset_dirs {
            copy_recursively(
                &mut *env,
                &asset_manager,
                asset_dir.as_ref().to_path_buf(),
                destination.as_ref().join(asset_dir),
            )?;
        }

        Ok(())
    }

    fn copy_recursively(
        env: &mut JNIEnv,
        asset_manager: &JObject,
        asset_dir: PathBuf,
        destination_dir: PathBuf,
    ) -> CopyResult<()> {
        for asset_filename in list(env, &asset_manager, &asset_dir)? {
            let asset_path = asset_dir.join(&asset_filename);
            if let Some(asset) = open_asset(&CString::new(asset_path.to_string_lossy().as_ref())?) {
                copy_asset(asset, asset_filename, &destination_dir)?;
            } else {
                copy_recursively(
                    env,
                    &asset_manager,
                    asset_path,
                    destination_dir.join(asset_filename),
                )?;
            }
        }
        Ok(())
    }

    fn list(
        env: &mut JNIEnv,
        asset_manager: &JObject,
        asset_dir: &Path,
    ) -> CopyResult<Vec<String>> {
        let asset_array: JObjectArray = env
            .call_method(
                asset_manager,
                "list",
                "(Ljava/lang/String;)[Ljava/lang/String;",
                &[JValueGen::Object(
                    &env.new_string(asset_dir.to_string_lossy())?.into(),
                )],
            )?
            .l()?
            .into();

        let mut assets = Vec::new();
        for index in 0..env.get_array_length(&asset_array)? {
            let elem = env.get_object_array_element(&asset_array, index)?.into();
            let asset: String = env.get_string(&elem)?.into();
            assets.push(asset);
        }

        Ok(assets)
    }

    fn open_asset(asset_path: &CStr) -> Option<Asset> {
        app().asset_manager().open(asset_path)
    }

    fn copy_asset(
        mut asset: Asset,
        filename: impl AsRef<Path>,
        destination_dir: impl AsRef<Path>,
    ) -> CopyResult<()> {
        fs::create_dir_all(destination_dir.as_ref())?;
        let mut file = fs::File::options()
            .create(true)
            .write(true)
            .open(destination_dir.as_ref().join(filename))?;

        io::copy(&mut asset, &mut file)?;
        Ok(())
    }
}
