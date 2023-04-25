use std::collections::HashMap;
use ugli::Ugli;

pub struct Library {
    ugli: Ugli,
    files: HashMap<String, String>,
    prefix: Option<(String, String)>, // TODO remove?
}

impl Library {
    pub fn empty(ugli: &Ugli) -> Self {
        Self {
            ugli: ugli.clone(),
            files: HashMap::new(),
            prefix: None,
        }
    }
    pub fn new(ugli: &Ugli, antialias: bool, prefix: Option<(String, String)>) -> Self {
        let mut library = Self::empty(ugli);
        let mut prelude = include_str!("prelude.glsl").to_owned();
        if antialias {
            prelude = "#define GENG_ANTIALIAS\n".to_owned() + &prelude;
        }
        fn default_prefix() -> (String, String) {
            let common_glsl = "#extension GL_OES_standard_derivatives : enable\nprecision highp int;\nprecision highp float;\n";
            if cfg!(target_arch = "wasm32") {
                (
                    format!("{common_glsl}#define VERTEX_SHADER\n"),
                    format!("{common_glsl}#define FRAGMENT_SHADER\n"),
                )
            } else {
                (
                    format!("#version 100\n{common_glsl}#define VERTEX_SHADER\n"),
                    format!("#version 100\n{common_glsl}#define FRAGMENT_SHADER\n"),
                )
            }
        }
        library.prefix = Some(prefix.unwrap_or_else(default_prefix));
        library.add("prelude", &prelude);
        library
    }

    pub fn add(&mut self, file_name: &str, source: &str) {
        self.files.insert(file_name.to_owned(), source.to_owned());
    }

    fn preprocess(&self, source: &str) -> Result<String, anyhow::Error> {
        let mut result = String::new();
        for line in source.lines() {
            if line.starts_with("#include") {
                let mut iter = line.split_whitespace();
                iter.next();
                let file = iter.next().expect("Expected path to include");
                assert!(iter.next().is_none(), "Unexpected token");
                assert!(
                    file.starts_with('<') && file.ends_with('>'),
                    "include path should be enclosed in angular brackets"
                );
                let file = file.trim_start_matches('<').trim_end_matches('>');
                if let Some(file) = self.files.get(file) {
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
        if let Some((vertex_prefix, fragment_prefix)) = &self.prefix {
            result.push_str(match shader_type {
                ugli::ShaderType::Vertex => vertex_prefix,
                ugli::ShaderType::Fragment => fragment_prefix,
            });
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
        let shader = |shader_type| -> anyhow::Result<ugli::Shader> {
            Ok(ugli::Shader::new(
                &self.ugli,
                shader_type,
                &self.process(shader_type, source)?,
            )?)
        };
        Ok(ugli::Program::new(
            &self.ugli,
            [
                &shader(ugli::ShaderType::Vertex)?,
                &shader(ugli::ShaderType::Fragment)?,
            ],
        )?)
    }
}
