fn main() {
    use batbox_la::*;
    use futures::prelude::*;
    use geng_window as window;
    use ugli::Ugli;

    fn f(ugli: &Ugli) {
        let program = ugli::Program::new(
            ugli,
            [
                &ugli::Shader::new(
                    ugli,
                    ugli::ShaderType::Vertex,
                    "void main() { gl_Position = vec4(0.0, 0.0, 0.0, 0.0); }",
                )
                .unwrap(),
                &ugli::Shader::new(ugli, ugli::ShaderType::Fragment, "void main() {}").unwrap(),
            ],
        )
        .unwrap();
        ugli.raw().use_program(program.raw());
        ugli.check();
    }
    window::run(
        &geng_window::Options::new("test"),
        move |window| async move {
            let ugli = window.ugli();
            f(ugli);
            while let Some(event) = window.events().next().await {
                if let window::Event::Draw = event {
                    // f(ugli);
                }
                if ugli.try_check().is_err() {
                    panic!("WTF");
                }
            }
        },
    );
}
