#[cfg(not(target_arch = "wasm32"))]
mod cargo_geng;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
    cargo_geng::run()
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("cargo-geng can not work on the web");
}
