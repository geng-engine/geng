use super::*;

pub struct Program {
    pub(crate) ugli: Rc<Ugli>,
    pub(crate) handle: raw::Program,
    pub(crate) attributes: HashMap<String, AttributeInfo>,
    pub(crate) uniforms: HashMap<String, UniformInfo>,
    phantom_data: PhantomData<*mut ()>,
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub(crate) location: raw::UInt,
    pub(crate) info: raw::ActiveInfo,
}

#[derive(Debug)]
pub struct UniformInfo {
    pub(crate) location: raw::UniformLocation,
    pub(crate) info: raw::ActiveInfo,
}

impl Drop for Program {
    fn drop(&mut self) {
        self.ugli.inner.delete_program(&self.handle);
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Program link failed:\n{}", log)]
pub struct ProgramLinkError {
    pub log: String,
}

impl Program {
    pub fn new(ugli: &Rc<Ugli>, shaders: &[&Shader]) -> Result<Self, ProgramLinkError> {
        let gl = &ugli.inner;
        let mut program = Program {
            ugli: ugli.clone(),
            handle: gl.create_program().unwrap(),
            uniforms: HashMap::new(),
            attributes: HashMap::new(),
            phantom_data: PhantomData,
        };
        for shader in shaders {
            gl.attach_shader(&program.handle, &shader.handle);
        }
        gl.link_program(&program.handle);
        for shader in shaders {
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
            assert!(location >= 0);
            program.attributes.insert(
                name,
                AttributeInfo {
                    location: location as raw::UInt,
                    info,
                },
            );
        }

        // Get uniforms
        let uniform_count =
            gl.get_program_parameter_int(&program.handle, raw::ACTIVE_UNIFORMS) as usize;
        for index in 0..uniform_count {
            let info = gl.get_active_uniform(&program.handle, index as raw::UInt);
            let name = info.name.clone();
            let location = gl.get_uniform_location(&program.handle, &name).unwrap();
            program
                .uniforms
                .insert(name, UniformInfo { location, info });
        }

        ugli.debug_check();
        Ok(program)
    }
    pub(crate) fn bind(&self) {
        self.ugli.inner.use_program(&self.handle);
    }
}
