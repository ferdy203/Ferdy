use clap::Parser;

/// A programmable, configurable and extensible API Gateway!
#[derive(Parser, Debug)]
#[clap(about = "A programmable, configurable and extensible API Gateway!", long_about = None)]
pub struct DakiaArgs {
    /// path for dakia local directory
    #[clap(long)]
    pub dp: Option<String>,

    /// watch for changes made to the config files
    #[clap(long)]
    pub watch: Option<bool>,

    /// reload config files and update run time configuration, it may trigger graceful restart if requird
    #[clap(long)]
    pub reload: Option<bool>,

    /// enable debug mode
    #[clap(long)]
    pub debug: Option<bool>,
}
