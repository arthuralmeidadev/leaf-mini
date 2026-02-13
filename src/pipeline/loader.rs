use std::{path::PathBuf, sync::Arc};

use crate::pipeline::{
    config::PipelineConfig,
    processor::{CreateFileProcessor, ProcessData, image_processor::ImageProcessor},
};

#[derive(Clone)]
pub enum FileFormat {
    IMAGE,
    AUDIO,
    JSON,
    CSV,
}

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub format: FileFormat,
}

impl FileEntry {
    fn new(path: PathBuf) -> Option<Self> {
        use FileFormat::*;
        let Some(Some(ext)) = path
            .extension()
            .and_then(|ext| Some(ext.to_str().and_then(|ext| Some(ext.to_lowercase()))))
        else {
            return None;
        };

        match ext.as_str() {
            "png" | "jpeg" | "jpg" | "tiff" | "bmp" => Some(FileEntry {
                path,
                format: IMAGE,
            }),
            "mp3" | "m4a" | "wav" | "ogg" => Some(FileEntry {
                path,
                format: AUDIO,
            }),
            "json" => Some(FileEntry { path, format: JSON }),
            "csv" => Some(FileEntry { path, format: CSV }),
            _ => None,
        }
    }

    pub fn get_processor(&self, config: Arc<PipelineConfig>) -> Box<dyn ProcessData> {
        match self.format {
            FileFormat::IMAGE => Box::new(ImageProcessor::new(self.clone(), config)),
            FileFormat::AUDIO => todo!(),
            FileFormat::JSON => todo!(),
            FileFormat::CSV => todo!(),
        }
    }
}

pub fn load_file_entries(input_dir: &PathBuf) -> Result<Vec<FileEntry>, std::io::Error> {
    let entries = input_dir.read_dir()?;

    Ok(entries.fold(Vec::new(), |mut acc, entry| {
        let Ok(entry) = entry else {
            return acc;
        };

        if let Some(file_entry) = FileEntry::new(entry.path()) {
            acc.push(file_entry);
        };

        acc
    }))
}
