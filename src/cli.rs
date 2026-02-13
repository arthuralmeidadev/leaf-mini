use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum Operation {
    Run,
}

#[derive(Parser, Debug)]
#[command(author="rthi_dev - Arthur Almeida", name = "Leaf Mini", version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(long, short)]
    pub config: PathBuf,

    #[arg(long, short, default_value = ".")]
    pub input_dir: PathBuf,

    #[arg(long, short, default_value = ".")]
    pub output_dir: PathBuf,

    #[arg(long, short)]
    pub ensure_output: bool,

    #[arg(long, short = 'x')]
    pub remove_input: bool,

    #[arg(long, short)]
    pub zip_output: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run(RunArgs),
}
