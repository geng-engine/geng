#![cfg(target_os = "android")]

pub type App = android_activity::AndroidApp;

static APP: std::sync::OnceLock<App> = std::sync::OnceLock::new();

pub fn init(app: App) {
    APP.set(app).unwrap();
}

pub fn app() -> &'static App {
    APP.get().expect("Android app was not set")
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
