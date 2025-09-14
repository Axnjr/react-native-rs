#[cfg(feature = "image-processing")]
use eyre::Result;
#[cfg(feature = "image-processing")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "image-processing")]
use std::time::Instant;

#[cfg(all(feature = "image-processing", feature = "ts-rs"))]
use ts_rs::TS;

#[cfg(feature = "image-processing")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct ImageProcessingInput {
    pub data: String, // Base64 encoded image data
    pub filter: ImageFilter,
    pub intensity: Option<f32>,
}

#[cfg(feature = "image-processing")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub enum ImageFilter {
    #[serde(rename = "grayscale")]
    Grayscale,
    #[serde(rename = "blur")]
    Blur,
    #[serde(rename = "brighten")]
    Brighten,
    #[serde(rename = "contrast")]
    Contrast,
}

#[cfg(feature = "image-processing")]
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ts-rs", derive(TS))]
#[cfg_attr(feature = "ts-rs", ts(export))]
pub struct ImageProcessingResult {
    pub data: String, // Base64 encoded processed image
    pub filter: ImageFilter,
    pub original_size: (u32, u32),
    pub computation_time_ms: u64,
}

#[cfg(feature = "image-processing")]
pub async fn process_image(input: &ImageProcessingInput) -> Result<ImageProcessingResult> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use image::{ImageFormat, DynamicImage};
    
    let start = Instant::now();
    
    // Decode base64 image data
    let image_data = STANDARD.decode(&input.data)?;
    let img = image::load_from_memory(&image_data)?;
    let original_size = (img.width(), img.height());
    
    // Apply the specified filter
    let processed_img = match input.filter {
        ImageFilter::Grayscale => img.grayscale(),
        ImageFilter::Blur => {
            let sigma = input.intensity.unwrap_or(1.0);
            img.blur(sigma)
        },
        ImageFilter::Brighten => {
            let brightness = (input.intensity.unwrap_or(0.1) * 255.0) as i32;
            img.brighten(brightness)
        },
        ImageFilter::Contrast => {
            let contrast = input.intensity.unwrap_or(1.2);
            img.adjust_contrast(contrast)
        },
    };
    
    // Encode back to base64
    let mut output_buffer = Vec::new();
    processed_img.write_to(&mut std::io::Cursor::new(&mut output_buffer), ImageFormat::Png)?;
    let processed_data = STANDARD.encode(&output_buffer);
    
    let computation_time_ms = start.elapsed().as_millis() as u64;
    
    Ok(ImageProcessingResult {
        data: processed_data,
        filter: input.filter.clone(),
        original_size,
        computation_time_ms,
    })
}

// Stub implementation when image-processing feature is not enabled
#[cfg(not(feature = "image-processing"))]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "image-processing"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProcessingInput;

#[cfg(not(feature = "image-processing"))]
pub async fn process_image(_input: &ImageProcessingInput) -> Result<String, eyre::Error> {
    Err(eyre::eyre!("Image processing feature not enabled"))
}
