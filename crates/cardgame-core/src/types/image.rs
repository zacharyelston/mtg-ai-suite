//! Image-related types

use serde::{Deserialize, Serialize};

/// A processed image ready for recognition.
#[derive(Debug, Clone)]
pub struct ProcessedImage {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub original_width: u32,
    pub original_height: u32,
    pub quality_score: f64,
    pub was_resized: bool,
}

impl ProcessedImage {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            data,
            width,
            height,
            original_width: width,
            original_height: height,
            quality_score: 1.0,
            was_resized: false,
        }
    }
}

/// A bounding box in an image.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
}

impl BoundingBox {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            rotation: 0.0,
        }
    }
}

/// Pixel-based bounding box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PixelBoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
