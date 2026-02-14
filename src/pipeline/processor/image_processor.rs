use std::{error::Error, path::PathBuf, sync::Arc};

use image::{DynamicImage, ImageError, ImageFormat, ImageReader};

use crate::pipeline::{config::PipelineConfig, processor::ProcessData};

pub struct ImageProcessor {
    path: PathBuf,
    config: Arc<PipelineConfig>,
    format: ImageFormat,
}

impl ImageProcessor {
    pub fn new(path: PathBuf, config: Arc<PipelineConfig>, ext: &str) -> Self {
        let format = match ext {
            "png" => ImageFormat::Png,
            "tiff" => ImageFormat::Tiff,
            "bmp" => ImageFormat::Bmp,
            "jpg" | "jpeg" | _ => ImageFormat::Jpeg,
        };

        Self {
            path,
            config,
            format,
        }
    }

    fn read_dynamic_image(&self) -> Result<DynamicImage, ImageError> {
        ImageReader::open(&self.path)?.decode()
    }

    fn handle_png(&self, img: DynamicImage) {
        todo!()
    }

    fn handle_jpeg(&self, img: DynamicImage) {
        todo!()
    }
}

impl ProcessData for ImageProcessor {
    fn process_data(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let img = self.read_dynamic_image()?;

        if self
            .config
            .image_settings
            .as_ref()
            .is_some_and(|image_settings| image_settings.format_specific_settings.is_some())
        {
            match self.format {
                ImageFormat::Png => self.handle_png(img),
                ImageFormat::Jpeg | _ => self.handle_jpeg(img),
            }
        }

        Ok(vec![])
    }
}
