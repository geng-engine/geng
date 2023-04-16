use super::*;

#[derive(clap::Parser)]
struct Opt {
    #[clap(long)]
    pub server: Option<String>,
    #[clap(long)]
    pub connect: Option<String>,
}

pub fn run<T: Model, G: geng::State>(
    game_name: &str,
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] model_constructor: impl FnOnce() -> T,
    game_constructor: impl FnOnce(&Geng, T::PlayerId, Remote<T>) -> G + 'static,
) {
    let mut opt: Opt = cli::parse();

    if opt.connect.is_none() && opt.server.is_none() {
        if cfg!(target_arch = "wasm32") {
            opt.connect = Some(
                option_env!("CONNECT")
                    .expect("Set CONNECT compile time env var")
                    .to_owned(),
            );
        } else {
            opt.server = Some("127.0.0.1:1155".to_owned());
            opt.connect = Some("ws://127.0.0.1:1155".to_owned());
        }
    }

    if opt.server.is_some() && opt.connect.is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        Server::new(opt.server.as_deref().unwrap(), model_constructor()).run();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let server = if let Some(addr) = &opt.server {
            let server = Server::new(addr, model_constructor());
            let server_handle = server.handle();
            let server_thread = std::thread::spawn(move || {
                server.run();
            });
            Some((server_handle, server_thread))
        } else {
            None
        };

        let geng = Geng::new(game_name);
        let state = ConnectingState::new(&geng, opt.connect.as_deref().unwrap(), {
            let geng = geng.clone();
            move |player_id, model| game_constructor(&geng, player_id, model)
        });
        geng.run(state);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some((server_handle, server_thread)) = server {
            server_handle.shutdown();
            server_thread.join().unwrap();
        }
    }
}
