use super::*;

struct Logger {
    inner: env_logger::Logger,
}

static LOGGERS: once_cell::sync::Lazy<Mutex<Vec<Box<dyn log::Log>>>> =
    once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.inner.enabled(metadata)
    }
    // fn foo() {
    //     if metadata.level() > log::max_level() {
    //         return false;
    //     }
    //     match metadata.target().split_terminator(':').next().unwrap() {
    //         "ws" => metadata.level() <= log::Level::Error,
    //         "mio" => false,
    //         _ => true,
    //     }
    // }

    fn log(&self, record: &log::Record) {
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

pub fn init_with(mut builder: env_logger::Builder) -> Result<(), log::SetLoggerError> {
    let builder_info = format!("{:?}", builder);
    let logger = Logger {
        inner: builder.build(),
    };
    log::set_max_level(logger.inner.filter());
    log::set_boxed_logger(Box::new(logger))?;
    trace!("Logger initialized with {}", builder_info);
    std::panic::set_hook(Box::new(|info| {
        error!("{}", info);
        error!("{:?}", backtrace::Backtrace::new());
    }));
    Ok(())
}

pub fn builder() -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();
    builder
        .filter_level(if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .parse_env("LOG");
    builder
}

pub fn init() -> Result<(), log::SetLoggerError> {
    init_with(builder())
}

pub fn add_logger(logger: Box<dyn log::Log>) {
    LOGGERS.lock().unwrap().push(logger);
}
