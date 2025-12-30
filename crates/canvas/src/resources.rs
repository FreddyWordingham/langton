use bevy::{math::U8Vec2, prelude::*, render::extract_resource::ExtractResource};

use super::types::{CanvasUploadOp, DirtyRect};

#[derive(Resource, Default)]
pub struct CanvasImageHandles {
    handles: Vec<Handle<Image>>,
}

impl CanvasImageHandles {
    pub fn new(num_chunks: U8Vec2, handles: Vec<Handle<Image>>) -> Self {
        debug_assert!(num_chunks.x > 0);
        debug_assert!(num_chunks.y > 0);

        let total_chunks = (num_chunks.x as usize) * (num_chunks.y as usize);
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
    num_chunks: U8Vec2,
    chunk_data: Vec<Vec<u32>>,
}

impl CanvasCpuChunks {
    pub fn new(num_chunks: U8Vec2, chunk_size: UVec2, default_px: u32) -> Self {
        debug_assert!(num_chunks.x > 0);
        debug_assert!(num_chunks.y > 0);
        debug_assert!(chunk_size.x > 0);
        debug_assert!(chunk_size.y > 0);

        let total_chunks = (num_chunks.x as usize) * (num_chunks.y as usize);
        let pixels_per_chunk = (chunk_size.x as usize) * (chunk_size.y as usize);

        // Initialise chunk data
        let mut chunk_data = Vec::with_capacity(total_chunks);
        for _ in 0..total_chunks {
            chunk_data.push(vec![default_px; pixels_per_chunk]);
        }

        Self { num_chunks, chunk_data }
    }

    #[inline]
    fn index(&self, chunk_key: &U8Vec2) -> usize {
        chunk_key.y as usize * self.num_chunks.x as usize + chunk_key.x as usize
    }

    pub fn chunk(&self, index: usize) -> &[u32] {
        debug_assert!(index < self.chunk_data.len());
        &self.chunk_data[index]
    }

    #[inline]
    pub fn chunk_mut(&mut self, chunk_key: &U8Vec2) -> &mut [u32] {
        debug_assert!(chunk_key.x < self.num_chunks.x);
        debug_assert!(chunk_key.y < self.num_chunks.y);

        let chunk_index = self.index(chunk_key);
        &mut self.chunk_data[chunk_index]
    }
}

/// Dirty tracking per chunk.
#[derive(Resource)]
pub struct CanvasDirtyRects {
    num_chunks: U8Vec2,
    chunk_size: UVec2,
    rects: Vec<DirtyRect>,
}

impl CanvasDirtyRects {
    pub fn new(num_chunks: U8Vec2, chunk_size: UVec2) -> Self {
        debug_assert!(num_chunks.x > 0);
        debug_assert!(num_chunks.y > 0);
        debug_assert!(chunk_size.x > 0);
        debug_assert!(chunk_size.y > 0);

        let total_chunks = (num_chunks.x as usize) * (num_chunks.y as usize);

        Self {
            num_chunks,
            chunk_size,
            rects: vec![DirtyRect::default(); total_chunks],
        }
    }

    #[inline(always)]
    fn index(&self, chunk_key: &U8Vec2) -> usize {
        chunk_key.y as usize * self.num_chunks.x as usize + chunk_key.x as usize
    }

    /// Mark a rect in chunk-local pixel coordinates as dirty.
    #[inline(always)]
    pub fn mark_rect(&mut self, chunk_key: &U8Vec2, min: UVec2, size: UVec2) {
        if size.x == 0 || size.y == 0 {
            return;
        }

        let max_bound = self.chunk_size - UVec2::ONE;

        // Clamp min into bounds first
        let min = min.min(max_bound);

        // Compute inclusive max, then clamp
        let max = (min + size - UVec2::ONE).min(max_bound);

        let index = self.index(chunk_key);
        let rect = &mut self.rects[index];

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
