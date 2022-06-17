use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, clap::Args, Default)]
pub struct CliArgs {
    /// Turn vertical synchronization on/off
    #[clap(long, name = "BOOL")]
    pub vsync: Option<bool>,
    /// Turn antialiasing on/off
    #[clap(long, name = "BOOL")]
    pub antialias: Option<bool>,
}

impl ContextOptions {
    pub fn from_args(args: &CliArgs) -> Self {
        let mut options = Self::default();
        if let Some(vsync) = args.vsync {
            options.vsync = vsync;
        }
        if let Some(antialias) = args.antialias {
            options.antialias = antialias;
        }
        options
    }
}
