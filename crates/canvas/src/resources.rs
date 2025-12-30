use bevy::{prelude::*, render::extract_resource::ExtractResource};

use super::types::{CanvasUploadOp, ChunkKey, DirtyRect};

#[derive(Resource, Default)]
pub struct CanvasImageHandles {
    handles: Vec<Handle<Image>>,
}

impl CanvasImageHandles {
    pub fn new(num_chunks: (u8, u8), handles: Vec<Handle<Image>>) -> Self {
        debug_assert!(num_chunks.0 > 0);
        debug_assert!(num_chunks.1 > 0);

        let total_chunks = (num_chunks.0 as usize) * (num_chunks.1 as usize);
        debug_assert_eq!(handles.len(), total_chunks);

        Self { handles }
    }

    #[inline(always)]
    pub fn handle(&self, index: usize) -> &Handle<Image> {
        &self.handles[index]
    }
}

/// CPU backing store: row-major u32 pixels per chunk.
#[derive(Resource)]
pub struct CanvasCpuChunks {
    num_chunks: (u8, u8),
    chunk_size: (u32, u32),
    chunk_data: Vec<Vec<u32>>,
}

impl CanvasCpuChunks {
    pub fn new(num_chunks: (u8, u8), chunk_size: (u32, u32), default_px: u32) -> Self {
        debug_assert!(chunk_size.0 > 0);
        debug_assert!(chunk_size.1 > 0);
        debug_assert!(num_chunks.0 > 0);
        debug_assert!(num_chunks.1 > 0);

        let total_chunks = (num_chunks.0 as usize) * (num_chunks.1 as usize);
        let pixels_per_chunk = (chunk_size.0 as usize) * (chunk_size.1 as usize);

        // Initialise chunk data
        let mut chunk_data = Vec::with_capacity(total_chunks);
        for _ in 0..total_chunks {
            chunk_data.push(vec![default_px; pixels_per_chunk]);
        }

        Self {
            num_chunks,
            chunk_size,
            chunk_data,
        }
    }
}

/// Dirty tracking per chunk.
#[derive(Resource)]
pub struct CanvasDirtyRects {
    num_chunks: (u8, u8),
    rects: Vec<DirtyRect>,
}

impl CanvasDirtyRects {
    pub fn new(num_chunks: (u8, u8)) -> Self {
        debug_assert!(num_chunks.0 > 0);
        debug_assert!(num_chunks.1 > 0);

        let total_chunks = (num_chunks.0 as usize) * (num_chunks.1 as usize);

        Self {
            num_chunks,
            rects: vec![DirtyRect::default(); total_chunks],
        }
    }

    #[inline(always)]
    fn index(&self, chunk_key: &ChunkKey) -> usize {
        (chunk_key.1 as usize) * (self.num_chunks.0 as usize) + (chunk_key.0 as usize)
    }

    /// Mark a rect in chunk-local pixel coordinates as dirty.
    #[inline(always)]
    pub fn mark_rect(&mut self, chunk_key: &ChunkKey, min: UVec2, size: UVec2) {
        if size.x == 0 || size.y == 0 {
            return;
        }

        let index = self.index(chunk_key);
        let rect = &mut self.rects[index];

        let max = min + size - UVec2::ONE;
        if !rect.dirty {
            rect.dirty = true;
            rect.min = min;
            rect.max = max;
        } else {
            rect.min = rect.min.min(min);
            rect.max = rect.max.max(max);
        }
    }

    #[inline(always)]
    pub fn take(&mut self, chunk_index: usize) -> Option<(UVec2, UVec2)> {
        let rect = &mut self.rects[chunk_index];
        if !rect.dirty {
            return None;
        }
        rect.dirty = false;
        Some((rect.min, rect.max))
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.rects.len()
    }
}

/// Render World resource holding pending canvas upload operations.
#[derive(Resource, Default, Clone)]
pub struct CanvasUploadOps {
    pub ops: Vec<CanvasUploadOp>,
}

impl ExtractResource for CanvasUploadOps {
    type Source = CanvasUploadOps;

    #[inline(always)]
    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}
