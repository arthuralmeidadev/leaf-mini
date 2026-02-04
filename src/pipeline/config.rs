use std::{fs, path::PathBuf, process::exit};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PipelineConfig {}

impl PipelineConfig {
    pub fn read_config(path: PathBuf) -> Self {
        let Ok(file_content) = fs::read_to_string(path) else {
            println!("Error while reading the contents of configuration file");
            exit(1)
        };

        let Ok(pipeline_config) = toml::from_str::<Self>(&file_content) else {
            println!("Error while parsing the contents of configuration file");
            exit(1)
        };

        pipeline_config
    }
}
