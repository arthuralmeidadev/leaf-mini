use std::{path::PathBuf, sync::Arc};

use crate::pipeline::{
    config::PipelineConfig,
    processor::{FileProcessor, ProcessData},
};

pub fn load_file_processors(
    input_dir: &PathBuf,
    config: Arc<PipelineConfig>,
) -> Result<Vec<Box<dyn ProcessData + Send + Sync>>, std::io::Error> {
    let entries = input_dir.read_dir()?;

    Ok(entries.fold(Vec::new(), |mut acc, entry| {
        let Ok(entry) = entry else {
            return acc;
        };

        if let Some(file_entry) = FileProcessor::new(entry.path(), Arc::clone(&config)) {
            acc.push(file_entry);
        };

        acc
    }))
}
