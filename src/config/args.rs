use clap::Parser;

/// A programmable, configurable and extensible API Gateway!
#[derive(Parser, Debug)]
#[clap(about = "A programmable, configurable and extensible API Gateway!", long_about = None)]
pub struct DakiaArgs {
    /// path for dakia local directory
    #[clap(short, long)]
    pub dp: Option<String>,

    /// debug mode
    pub debug: Option<bool>,
}
