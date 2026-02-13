pub mod config;
pub mod loader;
pub mod processor;

use std::{
    fs,
    process::exit,
    sync::{Arc, mpsc},
};

use crate::{
    cli::RunArgs,
    pipeline::{config::PipelineConfig, processor::ProcessData},
};

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

    let Ok(file_entries) = loader::load_file_entries(&run_args.input_dir) else {
        exit(1)
    };

    let pipeline_config = Arc::new(PipelineConfig::read_config(run_args.config));
    use rayon::prelude::*;

    let (tx, rx) = mpsc::channel();

    file_entries.par_iter().for_each(|entry| {
        let _ = tx.clone().send(
            entry
                .get_processor(Arc::clone(&pipeline_config))
                .process_data(),
        );
    });

    drop(tx);

    let _: Vec<_> = rx.iter().collect();
}
