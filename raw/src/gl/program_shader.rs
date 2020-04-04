use super::*;

pub type Program = gl::types::GLuint;
pub type Shader = gl::types::GLuint;

impl Context {
    pub fn attach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            gl::AttachShader(*program, *shader);
        }
    }

    pub fn compile_shader(&self, shader: &Shader) {
        unsafe {
            gl::CompileShader(*shader);
        }
    }

    pub fn create_program(&self) -> Option<Program> {
        let handle = unsafe { gl::CreateProgram() };
        if handle == 0 {
            None
        } else {
            Some(handle)
        }
    }

    pub fn create_shader(&self, typ: Enum) -> Option<Shader> {
        let handle = unsafe { gl::CreateShader(typ) };
        if handle == 0 {
            None
        } else {
            Some(handle)
        }
    }

    pub fn delete_program(&self, program: &Program) {
        unsafe {
            gl::DeleteProgram(*program);
        }
    }

    pub fn delete_shader(&self, shader: &Shader) {
        unsafe {
            gl::DeleteShader(*shader);
        }
    }

    pub fn detach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            gl::DetachShader(*program, *shader);
        }
    }

    pub fn get_program_info_log(&self, program: &Program) -> String {
        let mut info_log_length = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetProgramiv(*program, gl::INFO_LOG_LENGTH, info_log_length.as_mut_ptr());
        }
        let info_log_length = unsafe { info_log_length.assume_init() };
        let mut info_log_bytes =
            vec![std::mem::MaybeUninit::<u8>::uninit(); info_log_length as usize];
        unsafe {
            gl::GetProgramInfoLog(
                *program,
                info_log_bytes.len() as SizeI,
                std::ptr::null_mut(),
                info_log_bytes.as_mut_ptr() as *mut _,
            );
        }
        String::from_utf8(unsafe { std::mem::transmute(info_log_bytes) }).unwrap()
    }

    pub fn get_program_parameter_bool(&self, program: &Program, pname: Enum) -> Bool {
        self.get_program_parameter_int(program, pname) as Bool
    }

    pub fn get_program_parameter_int(&self, program: &Program, pname: Enum) -> Int {
        let mut result = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetProgramiv(*program, pname, result.as_mut_ptr());
        }
        unsafe { result.assume_init() }
    }

    pub fn get_shader_info_log(&self, shader: &Shader) -> String {
        let mut info_log_length = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetShaderiv(*shader, gl::INFO_LOG_LENGTH, info_log_length.as_mut_ptr());
        }
        let info_log_length = unsafe { info_log_length.assume_init() };
        let mut info_log_bytes =
            vec![std::mem::MaybeUninit::<u8>::uninit(); info_log_length as usize];
        unsafe {
            gl::GetShaderInfoLog(
                *shader,
                info_log_bytes.len() as SizeI,
                std::ptr::null_mut(),
                info_log_bytes.as_mut_ptr() as *mut _,
            )
        };
        String::from_utf8(unsafe { std::mem::transmute(info_log_bytes) }).unwrap()
    }

    pub fn get_shader_parameter_bool(&self, shader: &Shader, pname: Enum) -> Bool {
        self.get_shader_parameter_int(shader, pname) as Bool
    }

    pub fn get_shader_parameter_int(&self, shader: &Shader, pname: Enum) -> Int {
        let mut result = std::mem::MaybeUninit::uninit();
        unsafe {
            gl::GetShaderiv(*shader, pname, result.as_mut_ptr());
        }
        unsafe { result.assume_init() }
    }

    pub fn link_program(&self, program: &Program) {
        unsafe {
            gl::LinkProgram(*program);
        }
    }

    pub fn shader_source(&self, shader: &Shader, source: &str) {
        unsafe {
            gl::ShaderSource(
                *shader,
                1,
                [source.as_ptr() as *const Char].as_ptr() as _,
                [source.len() as *const Int].as_ptr() as _,
            );
        }
    }

    pub fn use_program(&self, program: &Program) {
        unsafe {
            gl::UseProgram(*program);
        }
    }
}
