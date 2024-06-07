use super::*;

pub struct Program {
    pub(crate) ugli: Ugli,
    pub(crate) cache_key: u64,
    pub(crate) handle: raw::Program,
    pub(crate) attributes: HashMap<String, AttributeInfo>,
    pub(crate) uniforms: HashMap<String, UniformInfo>,
    phantom_data: PhantomData<*mut ()>,
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub(crate) location: raw::UInt,
    #[allow(dead_code)]
    pub(crate) info: raw::ActiveInfo,
}

#[derive(Debug, Clone)]
pub struct UniformInfo {
    pub(crate) location: raw::UniformLocation,
    pub(crate) name: String,
    // pub(crate) info: raw::ActiveInfo,
    pub(crate) default: Option<UniformValue>,
}

impl Drop for Program {
    fn drop(&mut self) {
        self.ugli.inner.raw.delete_program(&self.handle);
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Program link failed:\n{log}")]
pub struct ProgramLinkError {
    pub log: String,
}

impl Program {
    pub fn new<'a>(
        ugli: &Ugli,
        shaders: impl IntoIterator<Item = &'a Shader>,
    ) -> Result<Self, ProgramLinkError> {
        let shaders: Vec<&Shader> = shaders.into_iter().collect();
        let gl = &ugli.inner.raw;
        let mut program = Program {
            cache_key: {
                use std::sync::atomic::{AtomicU64, Ordering};
                static NEXT: AtomicU64 = AtomicU64::new(0);
                NEXT.fetch_add(1, Ordering::SeqCst)
            },
            ugli: ugli.clone(),
            handle: gl.create_program().expect("Failed to create program"),
            uniforms: HashMap::new(),
            attributes: HashMap::new(),
            phantom_data: PhantomData,
        };
        for shader in &shaders {
            gl.attach_shader(&program.handle, &shader.handle);
        }
        gl.link_program(&program.handle);
        for shader in &shaders {
            gl.detach_shader(&program.handle, &shader.handle);
        }

        // Check for errors
        let link_status = gl.get_program_parameter_bool(&program.handle, raw::LINK_STATUS);
        if link_status == raw::FALSE {
            return Err(ProgramLinkError {
                log: gl.get_program_info_log(&program.handle),
            });
        }

        // Get attributes
        let attribute_count =
            gl.get_program_parameter_int(&program.handle, raw::ACTIVE_ATTRIBUTES) as usize;
        for index in 0..attribute_count {
            let info = gl.get_active_attrib(&program.handle, index as raw::UInt);
            let name = info.name.clone();
            let location = gl.get_attrib_location(&program.handle, &name);
            // TODO: why can't this be an assert?
            if location >= 0 {
                program.attributes.insert(
                    name,
                    AttributeInfo {
                        location: location as raw::UInt,
                        info,
                    },
                );
            }
        }

        // Get uniforms
        let uniform_count =
            gl.get_program_parameter_int(&program.handle, raw::ACTIVE_UNIFORMS) as usize;
        for index in 0..uniform_count {
            let info = gl.get_active_uniform(&program.handle, index as raw::UInt);
            for index in 0..info.size {
                let name = match info.size {
                    1 => info.name.clone(),
                    _ => format!("{}[{index}]", info.name.strip_suffix("[0]").unwrap()),
                };
                if let Some(location) = gl.get_uniform_location(&program.handle, &name) {
                    let default = UniformValue::get_value(&program, &location, &info);
                    // info!("{:?}", name);
                    program.uniforms.insert(
                        name.clone(),
                        UniformInfo {
                            location,
                            name,
                            // info,
                            default,
                        },
                    );
                }
            }
        }

        ugli.debug_check();
        Ok(program)
    }
    pub fn uniform_info(&self, name: &str) -> Option<UniformInfo> {
        self.uniforms.get(name).cloned()
    }
    pub(crate) fn bind(&self) {
        self.ugli.inner.raw.use_program(&self.handle);
    }
}
