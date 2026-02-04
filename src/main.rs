pub mod cli;
pub mod dispatcher;
pub mod pipeline;

use clap::Parser;

use crate::cli::Cli as Args;

fn main() {
    let args = Args::parse();
    dispatcher::handle_command(args);
}
