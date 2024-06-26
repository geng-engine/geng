use geng::prelude::*;

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("Hello, custom cursor!", |geng| async move {
        geng.window().set_cursor_type(geng::CursorType::Custom {
            image: geng
                .asset_manager()
                .load(run_dir().join("assets").join("cursor.png"))
                .await
                .unwrap(),
            hotspot: vec2(0, 0),
        });
        while let Some(_event) = geng.window().events().next().await {}
    });
}
