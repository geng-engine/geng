use crate::*;

struct Logger;

static LOGGERS: once_cell::sync::Lazy<Mutex<Vec<Box<dyn log::Log>>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        if metadata.level() > log::max_level() {
            return false;
        }
        match metadata.target().split_terminator(':').next().unwrap() {
            "ws" => metadata.level() <= log::Level::Error,
            "mio" => false,
            _ => true,
        }
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
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

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init_with_level(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)?;
    std::panic::set_hook(Box::new(|info| {
        error!("{}", info);
        error!("{:?}", backtrace::Backtrace::new());
    }));
    log::set_max_level(level);
    #[cfg(not(target_arch = "wasm32"))]
    add_logger(Box::new({
        let mut logger = stderrlog::new();
        logger.verbosity(4);
        logger
    }));
    Ok(())
}

pub fn init() -> Result<(), log::SetLoggerError> {
    if cfg!(debug_assertions) {
        init_with_level(log::LevelFilter::Debug)
    } else {
        init_with_level(log::LevelFilter::Info)
    }
}

pub fn add_logger(logger: Box<dyn log::Log>) {
    LOGGERS.lock().unwrap().push(logger);
}
