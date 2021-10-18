use super::*;

#[derive(clap::Parser)]
struct Opt {
    #[clap(long)]
    addr: Option<String>,
    #[clap(long)]
    server: bool,
    #[clap(long)]
    with_server: bool,
}

impl Opt {
    pub fn addr(&self) -> &str {
        match &self.addr {
            Some(addr) => addr,
            None => option_env!("SERVER_ADDR").unwrap_or("127.0.0.1:1155"),
        }
    }
}

pub fn run<T: Model, G: State>(
    game_name: &str,
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))] model_constructor: impl FnOnce() -> T,
    game_constructor: impl FnOnce(&Geng, T::PlayerId, Remote<T>) -> G + 'static,
) {
    let opt: Opt = clap::Parser::parse();
    if opt.server {
        #[cfg(not(target_arch = "wasm32"))]
        Server::new(opt.addr(), model_constructor()).run();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let server = if opt.with_server {
            let server = Server::new(opt.addr(), model_constructor());
            let server_handle = server.handle();
            let server_thread = std::thread::spawn(move || {
                server.run();
            });
            Some((server_handle, server_thread))
        } else {
            None
        };

        let geng = Geng::new(game_name);
        let state = ConnectingState::new(&geng, opt.addr(), {
            let geng = geng.clone();
            move |player_id, model| game_constructor(&geng, player_id, model)
        });
        geng::run(&geng, state);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some((server_handle, server_thread)) = server {
            server_handle.shutdown();
            server_thread.join().unwrap();
        }
    }
}
