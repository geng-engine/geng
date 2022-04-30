use super::*;

pub struct ShaderLib {
    ugli: Ugli,
    files: RefCell<HashMap<String, String>>,
    shader_prefix: RefCell<Option<String>>,
}

impl ShaderLib {
    pub(crate) fn new_impl(ugli: &Ugli, options: &ContextOptions) -> Self {
        let lib = Self {
            ugli: ugli.clone(),
            files: RefCell::new(HashMap::new()),
            shader_prefix: RefCell::new(options.shader_prefix.clone()),
        };
        let mut prelude = include_str!("include/prelude.glsl").to_owned();
        if options.antialias {
            prelude = "#define GENG_ANTIALIAS\n".to_owned() + &prelude;
        }
        lib.add("prelude", &prelude);
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
        if let Some(prefix) = &*self.shader_prefix.borrow() {
            result.push_str(prefix);
            result.push('\n');
        }
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
