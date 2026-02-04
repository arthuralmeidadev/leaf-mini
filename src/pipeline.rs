pub mod config;

use std::{fs, process::exit};

use crate::{cli::RunArgs, pipeline::config::PipelineConfig};

pub fn call_run(run_args: RunArgs) {
    if !run_args.config.is_file() {
        println!(
            "The provided configuration path \"{}\" is not a file or does not exist",
            run_args.config.to_str().unwrap_or_default()
        );

        exit(1)
    }

    if run_args
        .config
        .extension()
        .is_none_or(|ext| ext.to_str().is_none_or(|ext| ext != "toml"))
    {
        println!("Unsupported configuration file format. Expected \"*.toml\"");
        exit(1)
    }

    if !run_args.input_dir.is_dir() {
        println!(
            "The provided input path \"{}\" is not a valid directory",
            run_args.input_dir.to_str().unwrap_or_default()
        );

        exit(1)
    }

    if !run_args.output_dir.is_dir() {
        if !run_args.ensure_output {
            println!(
                "The provided output path \"{}\" is not a valid directory",
                run_args.output_dir.to_str().unwrap_or_default()
            );

            exit(1)
        }

        println!("Output directory does not exist. Creating.");
        fs::create_dir_all(run_args.output_dir)
            .expect("Error while creating path to output directory");
    }

    let pipeline_config = PipelineConfig::read_config(run_args.config);
}
