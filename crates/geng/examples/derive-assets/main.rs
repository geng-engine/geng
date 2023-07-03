use geng::prelude::*;

#[derive(geng::asset::Load)]
struct EmptyAssets {}

#[derive(geng::asset::Load)]
struct Assets {
    _0: EmptyAssets,
    #[load(path = "list/*.txt", list = "1..=3")]
    _1: Vec<String>,
    #[load(listed_in = "list.json")]
    list: Vec<String>,
}

struct Example;

impl geng::State for Example {
    fn draw(&mut self, _framebuffer: &mut ugli::Framebuffer) {}
    fn transition(&mut self) -> Option<geng::state::Transition> {
        Some(geng::state::Transition::Pop)
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("derive assets example", |geng| async move {
        let assets: Assets = geng
            .asset_manager()
            .load(run_dir().join("assets"))
            .await
            .unwrap();
        log::info!("{:?}", assets.list);
        geng.run_state(Example).await;
    });
}
