//! Logging
//!
//! Initialize the logger using one of the methods here, then just use the [log] crate stuff
//!
//! ```
//! batbox_logger::init();
//! log::info!("This is a doctest");
//! ```

use std::sync::Mutex;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct Logger {
    inner: env_logger::Logger,
}

static LOGGERS: once_cell::sync::Lazy<Mutex<Vec<Box<dyn log::Log>>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if !self.inner.enabled(metadata) {
            return false;
        }
        match metadata.target().split_terminator(':').next().unwrap() {
            "ws" => metadata.level() <= log::Level::Error,
            "mio" => false,
            _ => true,
        }
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        self.inner.log(record);
        if self.inner.matches(record) {
            for logger in LOGGERS.lock().unwrap().iter_mut() {
                if logger.enabled(record.metadata()) {
                    logger.log(record);
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                #[wasm_bindgen]
                extern "C" {
                    #[wasm_bindgen(js_namespace = console, js_name = log)]
                    fn console_log(s: &str);
                }
                console_log(&format!("{} - {}", record.level(), record.args()));
            }
        }
    }

    fn flush(&self) {
        self.inner.flush();
        for logger in LOGGERS.lock().unwrap().iter_mut() {
            logger.flush();
        }
    }
}

/// Initialize with a custom config
pub fn init_with(mut builder: env_logger::Builder) -> Result<(), log::SetLoggerError> {
    let builder_info = format!("{builder:?}");
    let logger = Logger {
        inner: builder.build(),
    };
    log::set_max_level(logger.inner.filter());
    log::set_boxed_logger(Box::new(logger))?;
    log::trace!("Logger initialized with {}", builder_info);
    std::panic::set_hook(Box::new(|info| {
        log::error!("{}", info);
        log::error!("{:?}", backtrace::Backtrace::new());
    }));
    Ok(())
}

/// Get the default logger builder configuration
pub fn builder() -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();
    builder
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format_timestamp(None)
        .format_module_path(true)
        .format_target(false)
        .parse_env("LOG");
    builder
}

/// Initialize using default config
pub fn init() {
    try_init().expect("Failed to initialize logger");
}

/// Initialize using default config, or return error
pub fn try_init() -> Result<(), log::SetLoggerError> {
    init_with(builder())
}

/// Initialize for tests ([crates::env_logger::Builder::is_test])
pub fn init_for_tests() {
    let mut builder = builder();
    builder.is_test(true);
    let _ = init_with(builder);
}

/// Add another custom logger to use in addition to the main one
pub fn add_logger(logger: Box<dyn log::Log>) {
    LOGGERS.lock().unwrap().push(logger);
}
