use bevy::{math::U8Vec2, prelude::*};

#[derive(Resource, Clone)]
pub struct CanvasConfig {
    pub clear_colour: [u8; 4],
    pub canvas_z_index: f32,
    pub canvas_size: UVec2,
    pub num_chunks: U8Vec2,
}

impl CanvasConfig {
    pub fn new(clear_colour: [u8; 4], canvas_z_index: f32, canvas_size: UVec2, chunks: U8Vec2) -> Self {
        debug_assert!(canvas_size.x > 0);
        debug_assert!(canvas_size.y > 0);
        debug_assert!(chunks.x > 0);
        debug_assert!(chunks.y > 0);
        debug_assert!(canvas_size.x % chunks.x as u32 == 0);
        debug_assert!(canvas_size.y % chunks.y as u32 == 0);
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
    pub fn canvas_size(&self) -> UVec2 {
        self.canvas_size
    }

    #[inline]
    pub fn chunk_size(&self) -> UVec2 {
        UVec2::new(
            self.canvas_size.x / self.num_chunks.x as u32,
            self.canvas_size.y / self.num_chunks.y as u32,
        )
    }

    #[inline]
    pub fn pixels_per_chunk(&self) -> usize {
        let chunk_size = self.chunk_size();
        (chunk_size.x as usize) * (chunk_size.y as usize)
    }

    #[inline]
    pub fn num_chunks(&self) -> U8Vec2 {
        self.num_chunks
    }

    #[inline]
    pub fn total_chunks(&self) -> usize {
        (self.num_chunks.x as usize) * (self.num_chunks.y as usize)
    }
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            clear_colour: [255, 255, 255, 255],
            canvas_z_index: 0.0,
            canvas_size: UVec2::new(1024, 512),
            num_chunks: U8Vec2::new(8, 4),
        }
    }
}
