use bevy::prelude::*;

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub struct ChunkKey(pub u8, pub u8);

#[derive(Clone)]
pub struct CanvasUploadOp {
    pub handle: Handle<Image>,
    pub start: UVec2,
    pub size: UVec2,
    pub bytes_per_row: u32,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct DirtyRect {
    pub min: UVec2,
    pub max: UVec2, // Inclusive
    pub dirty: bool,
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self {
            min: UVec2::ZERO,
            max: UVec2::ZERO,
            dirty: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SubRect {
    pub src_start: UVec2, // In original rect coords
    pub dst_start: UVec2, // Canvas coords (no wrap)
    pub size: UVec2,      // Width and height
}
