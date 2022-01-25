use super::*;

pub trait ResultExt {
    type Output;
    fn unwrap_or_abort(self) -> Self::Output;
}

impl<T, E: Debug> ResultExt for Result<T, E> {
    type Output = T;
    fn unwrap_or_abort(self) -> T {
        match self {
            Ok(value) => value,
            Err(e) => {
                if cfg!(target_arch = "wasm32") {
                    std::panic::panic_any(format!("{:?}", e))
                } else {
                    error!("{:?}", e);
                    std::process::exit(-1);
                }
            }
        }
    }
}
