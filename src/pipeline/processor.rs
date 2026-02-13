pub mod image_processor;

use std::{error::Error, sync::Arc};

use crate::pipeline::{config::PipelineConfig, loader::FileEntry};

pub trait CreateFileProcessor {
    fn new(file_entry: FileEntry, config: Arc<PipelineConfig>) -> Self;
}

pub trait ProcessData {
    fn process_data(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>;
}
