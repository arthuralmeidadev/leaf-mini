use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Operation {
    Run,
}

#[derive(Parser, Debug)]
#[command(author="rthi_dev - Arthur Almeida", name = "Leaf Mini", version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Run {
        #[arg(short, long)]
        config: PathBuf,

        #[arg(long, default_value = ".")]
        input_dir: PathBuf,

        #[arg(long, default_value = ".")]
        output: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run {
            config,
            input_dir,
            output,
        } => {
            println!(
                "{}",
                config
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
            );
        }
    }
}
