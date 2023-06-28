fn main() {
    let window = geng_window::Window::new(&geng_window::Options {
        fullscreen: false,
        vsync: true,
        title: "Test".to_owned(),
        antialias: false,
        transparency: false,
        size: None,
    });
    while !window.should_close() {
        window.swap_buffers();
    }
}
