use std::{io::Error, path::PathBuf, sync::Arc};

use crate::pipeline::{
    config::PipelineConfig,
    processor::{FileProcessor, ProcessData},
};

pub fn load_file_processors(
    input_dir: &PathBuf,
    config: Arc<PipelineConfig>,
) -> Result<Vec<Box<dyn ProcessData + Send + Sync>>, Error> {
    let entries = input_dir.read_dir()?;

    Ok(entries
        .flatten()
        .filter_map(|entry| FileProcessor::new(entry.path(), Arc::clone(&config)))
        .collect())
}
