use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, clap::Args, Default)]
pub struct CliArgs {
    /// Turn vertical synchronization on/off
    #[clap(long, value_name = "BOOL")]
    pub vsync: Option<bool>,
    /// Turn antialiasing on/off
    #[clap(long, value_name = "BOOL")]
    pub antialias: Option<bool>,
    /// Start with given window width (also requires window-height)
    #[clap(long, value_name = "PIXELS")]
    pub window_width: Option<usize>,
    /// Start with given window height (also requires window-width)
    #[clap(long, value_name = "PIXELS")]
    pub window_height: Option<usize>,
    /// Start in fullscreen
    #[clap(long, value_name = "BOOL")]
    pub fullscreen: Option<bool>,
    /// Enable/disable hot reloading of assets
    #[clap(long, value_name = "BOOL")]
    pub hot_reload: Option<bool>,
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
        if let (Some(window_width), Some(window_height)) = (args.window_width, args.window_height) {
            options.window_size = Some(vec2(window_width, window_height));
        }
        if let Some(fullscreen) = args.fullscreen {
            options.fullscreen = fullscreen;
        }
        if let Some(hot_reload) = args.hot_reload {
            options.hot_reload = hot_reload;
        }
        options
    }
}
