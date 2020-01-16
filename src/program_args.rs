use crate::*;

pub fn parse<T: StructOpt>() -> T {
    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    return StructOpt::from_iter({
        let mut args = Vec::<String>::new();
        args.push("lifeshot-io".to_owned()); // `Program` itself is the first arg
        let url = stdweb::web::window()
            .location()
            .expect("Failed to get window.location.href")
            .href()
            .expect("Failed to get window.location.href");
        let url = url::Url::parse(&url).expect("Failed to parse window.location.href");
        for (key, value) in url.query_pairs() {
            let key: &str = &key;
            let value: &str = &value;
            args.push("--".to_owned() + key + "=" + value);
        }
        trace!("href => args: {:?}", args);
        args
    });
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    return StructOpt::from_args();
}
