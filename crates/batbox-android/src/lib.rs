#![cfg(target_os = "android")]

pub type App = android_activity::AndroidApp;

static APP: std::sync::OnceLock<App> = std::sync::OnceLock::new();

pub fn init(app: App) {
    APP.set(app).unwrap();
}

pub fn app() -> &'static App {
    APP.get().expect("Android app was not set")
}
