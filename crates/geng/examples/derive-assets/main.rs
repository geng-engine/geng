use geng::prelude::*;

#[derive(geng::Assets)]
struct EmptyAssets {}

#[derive(geng::Assets)]
struct Assets {
    _0: EmptyAssets,
    #[asset(path = "list/*.txt", list = "1..=3")]
    _1: Vec<String>,
    #[asset(listed_in = "list.json")]
    list: Vec<String>,
}

struct Example;

impl geng::State for Example {
    fn draw(&mut self, _framebuffer: &mut ugli::Framebuffer) {}
    fn transition(&mut self) -> Option<geng::Transition> {
        Some(geng::Transition::Pop)
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new("derive assets example");
    geng.clone().run_loading(async move {
        let assets: Assets = geng.load_asset(run_dir().join("assets")).await.unwrap();
        info!("{:?}", assets.list);
        Example
    })
}
