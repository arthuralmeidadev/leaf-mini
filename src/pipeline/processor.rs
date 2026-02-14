pub mod image_processor;

use std::{error::Error, path::PathBuf, sync::Arc};

use crate::pipeline::{config::PipelineConfig, processor::image_processor::ImageProcessor};

pub struct FileProcessor;

pub trait ProcessData {
    fn process_data(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>;
}

impl FileProcessor {
    pub fn new(
        path: PathBuf,
        config: Arc<PipelineConfig>,
    ) -> Option<Box<dyn ProcessData + Send + Sync>> {
        let ext = path
            .extension()
            .and_then(|ext| Some(ext.to_str().and_then(|ext| Some(ext.to_lowercase()))))??;

        match ext.as_str() {
            "png" | "jpeg" | "jpg" | "tiff" | "bmp" => {
                Some(Box::new(ImageProcessor::new(path, config, &ext)))
            }
            "mp3" | "m4a" | "wav" | "ogg" => todo!(),
            "json" => todo!(),
            "csv" => todo!(),
            _ => todo!(),
        }
    }
}
