use clap::Parser;

/// A programmable, configurable and extensible API Gateway!
#[derive(Parser, Debug)]
#[clap(about = "A programmable, configurable and extensible API Gateway!", long_about = None)]
pub struct DakiaArgs {
    /// dakia config path
    #[clap(short, long)]
    pub dcp: Option<String>,
}
