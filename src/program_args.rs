use crate::*;

pub fn parse<T: StructOpt>() -> T {
    #[cfg(target_arch = "wasm32")]
    return StructOpt::from_iter({
        let mut args = Vec::<String>::new();
        args.push("lifeshot-io".to_owned()); // `Program` itself is the first arg
        let url = url::Url::parse(&web_sys::window().unwrap().location().href().unwrap())
            .expect("Failed to parse window.location.href");
        for (key, value) in url.query_pairs() {
            let key: &str = &key;
            let value: &str = &value;
            args.push("--".to_owned() + key + "=" + value);
        }
        trace!("href => args: {:?}", args);
        args
    });
    #[cfg(not(target_arch = "wasm32"))]
    return StructOpt::from_args();
}
