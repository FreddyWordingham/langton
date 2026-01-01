use bevy::prelude::*;

/// Draw a single pixel to the canvas.
#[derive(Message)]
pub struct DrawPixel {
    pub pos: UVec2,    // Canvas coords, bottom-left origin
    pub rgba_u32: u32, // One packed RGBA8 pixel value
}

/// Draw a rectangular region to the canvas.
/// The region will wrap toroidally if it exceeds canvas bounds.
#[derive(Message)]
pub struct DrawRect {
    pub start: UVec2,       // Canvas coords, bottom-left origin
    pub size: UVec2,        // Rect region dimensions (width, height)
    pub rgba_u32: Vec<u32>, // Length == w*h (one u32 per pixel)
}

/// Draw a contiguous horizontal span to the canvas.
/// If the span exceeds the canvas width, it will continue to the next row upwards.
/// If the span exceeds the canvas height, it will wrap to the bottom.
#[derive(Message)]
pub struct DrawSpan {
    pub start: UVec2,       // Canvas coords, bottom-left origin
    pub rgba_u32: Vec<u32>, // Row-major stream (one u32 per pixel)
}
