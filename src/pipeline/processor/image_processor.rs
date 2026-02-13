use std::{error::Error, sync::Arc};

use crate::pipeline::{
    config::PipelineConfig,
    loader::FileEntry,
    processor::{CreateFileProcessor, ProcessData},
};

pub struct ImageProcessor;

impl CreateFileProcessor for ImageProcessor {
    fn new(file_entry: FileEntry, config: Arc<PipelineConfig>) -> Self {
        todo!()
    }
}

impl ProcessData for ImageProcessor {
    fn process_data(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        Ok(vec![])
    }
}
