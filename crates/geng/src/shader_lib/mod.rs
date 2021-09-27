use super::*;

pub struct ShaderLib {
    ugli: Rc<Ugli>,
    files: RefCell<HashMap<String, String>>,
}

impl ShaderLib {
    pub fn new(ugli: &Rc<Ugli>) -> Self {
        let lib = Self {
            ugli: ugli.clone(),
            files: RefCell::new(HashMap::new()),
        };
        lib.add("prelude", include_str!("include/prelude.glsl"));
        lib
    }

    pub fn add(&self, file_name: &str, source: &str) {
        self.files
            .borrow_mut()
            .insert(file_name.to_owned(), source.to_owned());
    }

    fn preprocess(&self, source: &str) -> Result<String, anyhow::Error> {
        let mut result = String::new();
        for line in source.lines() {
            if line.starts_with("#include") {
                let mut iter = line.trim().split_whitespace();
                iter.next();
                let file = iter.next().expect("Expected path to include");
                assert!(iter.next().is_none(), "Unexpected token");
                assert!(
                    file.starts_with('<') && file.ends_with('>'),
                    "include path should be enclosed in angular brackets"
                );
                let file = file.trim_start_matches('<').trim_end_matches('>');
                if let Some(file) = self.files.borrow().get(file) {
                    result.push_str(&self.preprocess(file)?);
                } else {
                    anyhow::bail!("{:?} not found in shader library", file);
                }
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }
        Ok(result)
    }
    pub fn process(
        &self,
        shader_type: ugli::ShaderType,
        source: &str,
    ) -> Result<String, anyhow::Error> {
        let mut result = String::new();
        #[cfg(not(target_arch = "wasm32"))]
        result.push_str("#version 100\n");
        result.push_str("precision highp int;\nprecision highp float;\n");
        result.push_str(match shader_type {
            ugli::ShaderType::Vertex => "#define VERTEX_SHADER\n",
            ugli::ShaderType::Fragment => "#define FRAGMENT_SHADER\n",
        });
        result.push_str(&self.preprocess("#include <prelude>")?);
        result.push_str(&self.preprocess(source)?);
        Ok(result)
    }
    pub fn compile(&self, source: &str) -> Result<ugli::Program, anyhow::Error> {
        Ok(ugli::Program::new(
            &self.ugli,
            &[
                &ugli::Shader::new(
                    &self.ugli,
                    ugli::ShaderType::Vertex,
                    &self.process(ugli::ShaderType::Vertex, source)?,
                )?,
                &ugli::Shader::new(
                    &self.ugli,
                    ugli::ShaderType::Fragment,
                    &self.process(ugli::ShaderType::Fragment, source)?,
                )?,
            ],
        )?)
    }
}
