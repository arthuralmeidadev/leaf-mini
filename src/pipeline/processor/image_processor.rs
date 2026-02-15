use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat, ImageReader};

use crate::pipeline::{
    config::{FormatSpecificImageSettings, ImageFormat as ConfigImageFormat, PipelineConfig},
    processor::ProcessData,
};

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

    fn read_dynamic_image(&self) -> Result<DynamicImage> {
        Ok(ImageReader::open(&self.path)?.decode()?)
    }

    fn handle_png(&self, img: DynamicImage) {
        todo!()
    }

    fn handle_jpeg(&self, img: DynamicImage, settings: &FormatSpecificImageSettings) {}
}

impl ProcessData for ImageProcessor {
    fn process_data(&self) -> Result<Vec<u8>> {
        let img = self.read_dynamic_image()?;

        if let Some(format_specific_settings) = self
            .config
            .image_settings
            .as_ref()
            .and_then(|image_settings| image_settings.format_specific_settings.as_ref())
        {
            match self.format {
                ImageFormat::Png => self.handle_png(img),
                ImageFormat::Jpeg | _ => {
                    let jpeg_settings = format_specific_settings
                        .iter()
                        .find(|setting| matches!(setting.format, ConfigImageFormat::JPEG))
                        .context("No settings for jpeg type.")?;

                    self.handle_jpeg(img, jpeg_settings);
                }
            }
        };

        Ok(vec![])
    }
}
