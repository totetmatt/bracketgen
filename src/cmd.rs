use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {

    #[clap(value_parser)]
    handles: Vec<String>,
}
