use super::*;

#[derive(Debug, Copy, Clone)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Shader {
    pub(crate) ugli: Rc<Ugli>,
    pub(crate) handle: raw::Shader,
    phantom_data: PhantomData<*mut ()>,
}

impl Drop for Shader {
    fn drop(&mut self) {
        let gl = &self.ugli.inner;
        gl.delete_shader(&self.handle);
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Shader compilation failed:\n{log}")]
pub struct ShaderCompilationError {
    pub log: String,
}

impl Shader {
    pub fn new(
        ugli: &Rc<Ugli>,
        shader_type: ShaderType,
        source: &str,
    ) -> Result<Self, ShaderCompilationError> {
        let gl = &ugli.inner;
        let shader = Self {
            ugli: ugli.clone(),
            handle: gl
                .create_shader(match shader_type {
                    ShaderType::Vertex => raw::VERTEX_SHADER,
                    ShaderType::Fragment => raw::FRAGMENT_SHADER,
                })
                .unwrap(),
            phantom_data: PhantomData,
        };
        gl.shader_source(&shader.handle, source);
        gl.compile_shader(&shader.handle);
        let compile_status = gl.get_shader_parameter_bool(&shader.handle, raw::COMPILE_STATUS);
        if compile_status == raw::FALSE {
            return Err(ShaderCompilationError {
                log: gl.get_shader_info_log(&shader.handle),
            });
        }
        ugli.debug_check();
        Ok(shader)
    }
}
