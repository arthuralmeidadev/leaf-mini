use std::{error::Error, path::PathBuf, sync::Arc};

use crate::pipeline::{config::PipelineConfig, processor::ProcessData};

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn new(path: PathBuf, config: Arc<PipelineConfig>) -> Self {
        todo!()
    }
}

impl ProcessData for ImageProcessor {
    fn process_data(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        Ok(vec![])
    }
}
