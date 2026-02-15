use std::{fs, path::PathBuf, process::exit};

use serde::Deserialize;

#[derive(Deserialize)]
pub enum ImageCompression {
    Lossy,
    Lossless,
}

#[derive(Deserialize)]
pub enum ImageTransform {
    Crop(Option<f64>, Option<f64>),
    Resize(Option<f64>, Option<f64>),
}

#[derive(Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    TIFF,
    BITMAP,
}

#[derive(Deserialize)]
pub enum AudioFormat {
    MP3,
    M4A,
    WAV,
    OGG,
}

#[derive(Deserialize)]
pub struct GeneralImageSettings {
    pub compression: ImageCompression,
    pub transform: Option<ImageTransform>,
    pub convert_to: Option<ImageFormat>,
}

#[derive(Deserialize)]
pub struct FormatSpecificImageSettings {
    pub format: ImageFormat,
    pub compression: ImageCompression,
    pub transform: Option<ImageTransform>,
    pub convert_to: Option<ImageFormat>,
}

#[derive(Deserialize)]
pub struct ImageSettings {
    pub general: GeneralImageSettings,
    pub format_specific_settings: Option<Vec<FormatSpecificImageSettings>>,
}

#[derive(Deserialize)]
pub enum FileEncryptionAlgorithm {
    RSA2048,
    RSA4096,
    ED25519,
}

#[derive(Deserialize)]
pub struct EncryptionSettings {
    pub algorithm: FileEncryptionAlgorithm,
    pub public_key: String,
}

#[derive(Deserialize)]
pub enum FileIdentity {}

#[derive(Deserialize)]
pub struct PipelineConfig {
    pub create_subdirectories: bool,
    pub image_settings: Option<ImageSettings>,
    pub encryption_settings: Option<EncryptionSettings>,
}

impl PipelineConfig {
    pub fn read_config(path: PathBuf) -> Self {
        let Ok(file_content) = fs::read_to_string(path) else {
            println!("Error while reading the contents of configuration file");
            exit(1)
        };

        let Ok(pipeline_config) = toml::from_str::<Self>(&file_content) else {
            println!("Error while parsing the contents of configuration file");
            exit(1)
        };

        pipeline_config
    }
}
