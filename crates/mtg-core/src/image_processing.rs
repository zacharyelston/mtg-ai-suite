//! Image processing utilities
//!
//! This module provides image preprocessing functions for card recognition.

use crate::error::{Error, Result};
use image::{DynamicImage, GenericImageView, ImageFormat};
use tracing::instrument;

/// Minimum image dimensions for reliable recognition
pub const MIN_WIDTH: u32 = 640;
pub const MIN_HEIGHT: u32 = 480;

/// Maximum image dimensions (will be resized if larger)
pub const MAX_WIDTH: u32 = 1920;
pub const MAX_HEIGHT: u32 = 1080;

/// Image processing configuration
#[derive(Debug, Clone)]
pub struct ImageConfig {
    /// Maximum width after processing
    pub max_width: u32,

    /// Maximum height after processing
    pub max_height: u32,

    /// JPEG quality for output (1-100)
    pub jpeg_quality: u8,

    /// Whether to apply contrast enhancement
    pub enhance_contrast: bool,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            max_width: MAX_WIDTH,
            max_height: MAX_HEIGHT,
            jpeg_quality: 90,
            enhance_contrast: true,
        }
    }
}

/// Processed image result
#[derive(Debug)]
pub struct ProcessedImage {
    /// The processed image
    pub image: DynamicImage,

    /// Original dimensions
    pub original_width: u32,
    pub original_height: u32,

    /// Final dimensions
    pub width: u32,
    pub height: u32,

    /// Whether the image was resized
    pub was_resized: bool,
}

impl ProcessedImage {
    /// Convert to JPEG bytes
    pub fn to_jpeg(&self, _quality: u8) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);

        self.image
            .write_to(&mut cursor, ImageFormat::Jpeg)
            .map_err(|e| Error::ImageDecodeFailed {
                reason: e.to_string(),
            })?;

        Ok(buffer)
    }

    /// Convert to PNG bytes
    pub fn to_png(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);

        self.image
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| Error::ImageDecodeFailed {
                reason: e.to_string(),
            })?;

        Ok(buffer)
    }
}

/// Process an image for card recognition
///
/// # Arguments
/// * `data` - Raw image bytes
/// * `config` - Processing configuration
///
/// # Returns
/// Processed image ready for OCR/recognition
#[instrument(skip(data))]
pub fn process_image(data: &[u8], config: &ImageConfig) -> Result<ProcessedImage> {
    // Decode image
    let img = image::load_from_memory(data).map_err(|e| Error::ImageDecodeFailed {
        reason: e.to_string(),
    })?;

    let (original_width, original_height) = img.dimensions();

    // Check minimum size
    if original_width < MIN_WIDTH || original_height < MIN_HEIGHT {
        return Err(Error::ImageTooSmall {
            width: original_width,
            height: original_height,
            min_width: MIN_WIDTH,
            min_height: MIN_HEIGHT,
        });
    }

    // Resize if needed
    let (img, was_resized) =
        if original_width > config.max_width || original_height > config.max_height {
            let resized = img.resize(
                config.max_width,
                config.max_height,
                image::imageops::FilterType::Lanczos3,
            );
            (resized, true)
        } else {
            (img, false)
        };

    let (width, height) = img.dimensions();

    // Apply contrast enhancement if enabled
    let img = if config.enhance_contrast {
        enhance_contrast(&img)
    } else {
        img
    };

    Ok(ProcessedImage {
        image: img,
        original_width,
        original_height,
        width,
        height,
        was_resized,
    })
}

/// Decode image from bytes without processing
pub fn decode_image(data: &[u8]) -> Result<DynamicImage> {
    image::load_from_memory(data).map_err(|e| Error::ImageDecodeFailed {
        reason: e.to_string(),
    })
}

/// Enhance image contrast for better OCR
fn enhance_contrast(img: &DynamicImage) -> DynamicImage {
    // Simple contrast enhancement using adjustments
    // In a full implementation, this would use more sophisticated algorithms
    img.adjust_contrast(10.0)
}

/// Crop image to the card name region (top portion)
///
/// Card names are typically in the top 15% of the card
pub fn crop_to_name_region(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let name_height = (height as f32 * 0.15) as u32;

    img.crop_imm(0, 0, width, name_height)
}

/// Crop image to the card text region (middle portion)
///
/// Card text is typically in the middle 40% of the card
pub fn crop_to_text_region(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let start_y = (height as f32 * 0.45) as u32;
    let text_height = (height as f32 * 0.40) as u32;

    img.crop_imm(0, start_y, width, text_height)
}

/// Convert image to grayscale
pub fn to_grayscale(img: &DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(img.to_luma8())
}

/// Calculate image quality score (0.0 to 1.0)
///
/// Based on sharpness, contrast, and brightness
pub fn calculate_quality_score(img: &DynamicImage) -> f64 {
    let gray = img.to_luma8();
    let (width, height) = gray.dimensions();

    if width == 0 || height == 0 {
        return 0.0;
    }

    // Calculate variance (measure of contrast/sharpness)
    let pixels: Vec<f64> = gray.pixels().map(|p| p.0[0] as f64).collect();
    let mean: f64 = pixels.iter().sum::<f64>() / pixels.len() as f64;
    let variance: f64 =
        pixels.iter().map(|p| (p - mean).powi(2)).sum::<f64>() / pixels.len() as f64;

    // Normalize variance to 0-1 range (assuming max variance of ~6500 for 8-bit image)
    let normalized_variance = (variance / 6500.0).min(1.0);

    // Check brightness (should be in middle range)
    let brightness_score = 1.0 - ((mean - 127.5) / 127.5).abs();

    // Combine scores
    (normalized_variance * 0.7 + brightness_score * 0.3).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_image(width: u32, height: u32) -> Vec<u8> {
        let img = DynamicImage::new_rgb8(width, height);
        let mut buffer = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut buffer);
        img.write_to(&mut cursor, ImageFormat::Png).unwrap();
        buffer
    }

    #[test]
    fn test_process_image_too_small() {
        let data = create_test_image(100, 100);
        let config = ImageConfig::default();
        let result = process_image(&data, &config);

        assert!(matches!(result, Err(Error::ImageTooSmall { .. })));
    }

    #[test]
    fn test_process_image_resize() {
        let data = create_test_image(3000, 2000);
        let config = ImageConfig::default();
        let result = process_image(&data, &config).unwrap();

        assert!(result.was_resized);
        assert!(result.width <= config.max_width);
        assert!(result.height <= config.max_height);
    }

    #[test]
    fn test_process_image_no_resize() {
        let data = create_test_image(800, 600);
        let config = ImageConfig::default();
        let result = process_image(&data, &config).unwrap();

        assert!(!result.was_resized);
        assert_eq!(result.width, 800);
        assert_eq!(result.height, 600);
    }

    #[test]
    fn test_crop_to_name_region() {
        let img = DynamicImage::new_rgb8(1000, 1000);
        let cropped = crop_to_name_region(&img);

        assert_eq!(cropped.width(), 1000);
        assert_eq!(cropped.height(), 150); // 15% of 1000
    }

    #[test]
    fn test_quality_score_range() {
        let img = DynamicImage::new_rgb8(100, 100);
        let score = calculate_quality_score(&img);

        assert!(score >= 0.0);
        assert!(score <= 1.0);
    }
}
