use geng::prelude::*;

fn main() {
    Geng::run("Hello, World!", |geng| async move {
        let mut events = geng.window().events();
        while let Some(_) = events.next().await {
            if geng.ugli().try_check().is_err() {
                panic!("WTF");
            }
            geng.window().ugli().check();
        }
    });
}
