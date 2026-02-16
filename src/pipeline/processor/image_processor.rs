use std::{io::Cursor, path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat, ImageReader, codecs::jpeg::JpegEncoder};

use crate::pipeline::{
    config::{ImageCompression, ImageFormat as ConfigImageFormat, ImageTransform, PipelineConfig},
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

    fn handle_image(
        &self,
        img: DynamicImage,
        compression: &ImageCompression,
        transform: Option<&ImageTransform>,
        convert_to: Option<&ConfigImageFormat>,
    ) -> Result<Vec<u8>> {
        let mut output_bytes = Vec::new();

        if matches!(compression, ImageCompression::Lossy) {
            let cursor = Cursor::new(&mut output_bytes);
            let encoder = JpegEncoder::new_with_quality(cursor, 75u8);

            img.write_with_encoder(encoder)?;
        }

        Ok(output_bytes)
    }
}

impl ProcessData for ImageProcessor {
    fn process_data(&self) -> Result<Vec<u8>> {
        let img = self.read_dynamic_image()?;
        let image_settings = self.config.image_settings.as_ref().context("")?;
        let default_settings = &image_settings.general;
        let specific_settings =
            image_settings
                .format_specific_settings
                .as_ref()
                .and_then(|format_specific_settings| {
                    format_specific_settings.iter().find(|specific_settings| {
                        match (&specific_settings.format, self.format) {
                            (ConfigImageFormat::PNG, ImageFormat::Png)
                            | (ConfigImageFormat::JPEG, ImageFormat::Jpeg)
                            | (ConfigImageFormat::TIFF, ImageFormat::Tiff)
                            | (ConfigImageFormat::BITMAP, ImageFormat::Bmp) => true,
                            _ => false,
                        }
                    })
                });

        let (compression, transform, convert_to) = match specific_settings {
            Some(settings) => (
                &settings.compression,
                &settings.transform,
                &settings.convert_to,
            ),
            None => (
                &default_settings.compression,
                &default_settings.transform,
                &default_settings.convert_to,
            ),
        };

        let result =
            self.handle_image(img, compression, transform.as_ref(), convert_to.as_ref())?;

        Ok(result)
    }
}
