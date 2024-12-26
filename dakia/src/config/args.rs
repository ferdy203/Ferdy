use clap::Parser;

/// A programmable, configurable, and extensible API Gateway!
#[derive(Parser, Debug)]
#[clap(about = "A programmable, configurable, and extensible API Gateway!", long_about = None)]
pub struct DakiaArgs {
    /// Path to Dakia's local directory for storing configuration, interceptors, filters, extensions  and runtime data.
    #[clap(long)]
    pub dp: Option<String>,

    /// Watch for changes in configuration files, interceptors, filters and extensions and automatically apply updates.
    #[clap(short, long, default_value_t = false)]
    pub watch: bool,

    /// Reload configuration files and update runtime settings.
    /// May trigger a graceful restart if required.
    #[clap(long, default_value_t = false)]
    pub reload: bool,

    /// Test the server configuration without starting the application.
    #[clap(short, long, default_value_t = false)]
    pub test: bool,

    /// Display the current version of the API Gateway and exit.
    #[clap(short, long, default_value_t = false)]
    pub version: bool,

    /// Enable verbose logging for more detailed output.
    /// This is useful for debugging and monitoring.
    #[clap(long, default_value_t = false)]
    pub verbose: bool,

    /// Enable debug mode to output additional debugging information.
    /// Use this to troubleshoot issues during development or runtime.
    #[clap(long, default_value_t = false)]
    pub debug: bool,
}
