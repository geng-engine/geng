use super::*;

pub type Program = web_sys::WebGlProgram;
pub type Shader = web_sys::WebGlShader;

impl Context {
    pub fn attach_shader(&self, program: &Program, shader: &Shader) {
        self.inner.attach_shader(program, shader);
    }

    pub fn compile_shader(&self, shader: &Shader) {
        self.inner.compile_shader(shader);
    }

    pub fn create_program(&self) -> Option<Program> {
        self.inner.create_program()
    }

    pub fn create_shader(&self, typ: Enum) -> Option<Shader> {
        self.inner.create_shader(typ)
    }

    pub fn delete_program(&self, program: &Program) {
        self.inner.delete_program(Some(program));
    }

    pub fn delete_shader(&self, shader: &Shader) {
        self.inner.delete_shader(Some(shader));
    }

    pub fn detach_shader(&self, program: &Program, shader: &Shader) {
        self.inner.detach_shader(program, shader);
    }

    pub fn get_program_info_log(&self, program: &Program) -> String {
        self.inner.get_program_info_log(program).unwrap()
    }

    pub fn get_program_parameter_bool(&self, program: &Program, pname: Enum) -> Bool {
        self.inner
            .get_program_parameter(program, pname)
            .as_bool()
            .unwrap()
    }

    pub fn get_program_parameter_int(&self, program: &Program, pname: Enum) -> Int {
        self.inner
            .get_program_parameter(program, pname)
            .as_f64()
            .unwrap() as Int
    }

    pub fn get_shader_info_log(&self, shader: &Shader) -> String {
        self.inner.get_shader_info_log(shader).unwrap()
    }

    pub fn get_shader_parameter_bool(&self, shader: &Shader, pname: Enum) -> Bool {
        self.inner
            .get_shader_parameter(shader, pname)
            .as_bool()
            .unwrap()
    }

    pub fn get_shader_parameter_int(&self, shader: &Shader, pname: Enum) -> Int {
        self.inner
            .get_shader_parameter(shader, pname)
            .as_f64()
            .unwrap() as Int
    }

    pub fn link_program(&self, program: &Program) {
        self.inner.link_program(program);
    }

    pub fn shader_source(&self, shader: &Shader, source: &str) {
        self.inner.shader_source(shader, source);
    }

    pub fn use_program(&self, program: &Program) {
        self.inner.use_program(Some(program));
    }
}
