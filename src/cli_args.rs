use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, clap::Args, Default)]
#[group(id = "geng")]
pub struct CliArgs {
    #[clap(flatten)]
    pub window: window::CliArgs,
    /// Enable/disable hot reloading of assets
    #[clap(long, value_name = "BOOL")]
    pub hot_reload: Option<bool>,
}

impl ContextOptions {
    pub fn with_cli(&mut self, args: &CliArgs) {
        self.window.with_cli(&args.window);
        if let Some(hot_reload) = args.hot_reload {
            self.hot_reload = hot_reload;
        }
    }
}
