use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CanvasConfig {
    pub clear_colour: [u8; 4],
    pub canvas_z_index: f32,
    pub canvas_size: (u32, u32),
    pub num_chunks: (u8, u8),
}

impl CanvasConfig {
    pub fn new(clear_colour: [u8; 4], canvas_z_index: f32, canvas_size: (u32, u32), chunks: (u8, u8)) -> Self {
        debug_assert!(canvas_size.0 > 0);
        debug_assert!(canvas_size.1 > 0);
        debug_assert!(chunks.0 > 0);
        debug_assert!(chunks.1 > 0);
        debug_assert!(canvas_size.0 % chunks.0 as u32 == 0);
        debug_assert!(canvas_size.1 % chunks.1 as u32 == 0);
        Self {
            clear_colour,
            canvas_z_index,
            canvas_size,
            num_chunks: chunks,
        }
    }

    #[inline]
    pub fn clear_colour(&self) -> &[u8; 4] {
        &self.clear_colour
    }

    #[inline]
    pub fn canvas_z_index(&self) -> f32 {
        self.canvas_z_index
    }

    #[inline]
    pub fn chunk_size(&self) -> (u32, u32) {
        (
            self.canvas_size.0 / self.num_chunks.0 as u32,
            self.canvas_size.1 / self.num_chunks.1 as u32,
        )
    }

    #[inline]
    pub fn pixels_per_chunk(&self) -> usize {
        let (chunk_width, chunk_height) = self.chunk_size();
        (chunk_width as usize) * (chunk_height as usize)
    }

    #[inline]
    pub fn num_chunks(&self) -> (u8, u8) {
        self.num_chunks
    }

    #[inline]
    pub fn total_chunks(&self) -> usize {
        (self.num_chunks.0 as usize) * (self.num_chunks.1 as usize)
    }
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            clear_colour: [255, 255, 255, 255],
            canvas_z_index: 0.0,
            canvas_size: (800, 600),
            num_chunks: (8, 6),
        }
    }
}
