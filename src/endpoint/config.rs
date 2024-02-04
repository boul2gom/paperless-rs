use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::PaperlessClient;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    id: u64,
    user_args: Option<String>,
    application_logo: Option<String>,
    application_title: Option<String>,
    output_type: Option<OutputType>,
    mode: Option<Mode>,
    clean: Option<Clean>,
    rotate_page_threshold: Option<u64>,
    ocr_arguments: Option<String>,
    language: Option<String>,
    skip_archive_file: Option<SkipArchiveFile>,
    deskew: Option<bool>,
    max_image_pixels: Option<u64>,
    pages: Option<u64>,
    images_dpi: Option<u64>,
    rotate_pages: Option<bool>,
    color_conversion_strategy: Option<ColorStrategy>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OutputType {
    PDF,
    PDFA,
    PDFA1,
    PDFA2,
    PDFA3,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    Skip,
    Redo,
    Force,
    SkipNoArchive,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Clean {
    Clean,
    CleanFinal,
    None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SkipArchiveFile {
    Never,
    Always,
    WithText,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorStrategy {
    RGB,
    Gray,
    CMYK,
    LeaveColorUnchanged,
    UseDeviceIndependentColor,
}

impl PaperlessClient {
    pub async fn fetch_configuration(&self) -> Result<Configuration, Box<dyn std::error::Error>> {
        let url = format!("{}/config/", self.base_url);

        let request_builder = self.prepare_endpoint(Method::GET, url).await?;
        self.call_endpoint(request_builder).await
    }
}
