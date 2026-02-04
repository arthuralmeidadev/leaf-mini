use crate::cli::{Cli as Args, Commands};
use crate::pipeline;

pub fn handle_command(args: Args) {
    match args.command {
        Commands::Run(run_args) => pipeline::call_run(run_args),
    }
}
