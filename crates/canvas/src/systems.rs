use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use super::{
    components::CanvasImage,
    config::CanvasConfig,
    resources::{CanvasCpuChunks, CanvasDirtyRects, CanvasImageHandles, CanvasUploadOps},
};

pub fn spawn_canvas_images(mut commands: Commands, config: Res<CanvasConfig>, mut images: ResMut<Assets<Image>>) {
    let (horizontal_chunks, vertical_chunks) = config.num_chunks();
    let (chunk_width, chunk_height) = config.chunk_size();
    let pixels_per_chunk = config.pixels_per_chunk();

    // Create default RGBA8 data for one chunk
    let [r, g, b, a] = *config.clear_colour();
    let mut data = vec![0u8; pixels_per_chunk * 4];
    for px in data.chunks_exact_mut(4) {
        px.copy_from_slice(&[r, g, b, a]);
    }

    let mut image_handles = Vec::with_capacity(config.total_chunks());
    for y in 0..vertical_chunks {
        for x in 0..horizontal_chunks {
            // Create image for this chunk
            let mut image = Image::new(
                Extent3d {
                    width: chunk_width,
                    height: chunk_height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                data.clone(),
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            );
            image.texture_descriptor.usage |= TextureUsages::COPY_DST;
            image.sampler = ImageSampler::nearest();

            // Store image handle
            let handle = images.add(image);
            image_handles.push(handle.clone());

            // Spawn a sprite entity to render this chunk
            commands.spawn((
                CanvasImage,
                Sprite::from_image(handle),
                Transform::from_translation(Vec3::new(
                    (x as f32 - ((horizontal_chunks - 1) as f32 / 2.0)) * chunk_width as f32,
                    (y as f32 - ((vertical_chunks - 1) as f32 / 2.0)) * chunk_height as f32,
                    config.canvas_z_index(),
                ))
                .with_scale(Vec3::new(1.0, -1.0, 1.0)),
            ));
        }
    }

    // Store handles and create CPU chunk storage
    commands.insert_resource(CanvasImageHandles::new((horizontal_chunks, vertical_chunks), image_handles));

    // Create CPU chunk storage
    commands.insert_resource(CanvasCpuChunks::new(
        (horizontal_chunks, vertical_chunks),
        (chunk_width, chunk_height),
        u32::from_le_bytes(*config.clear_colour()),
    ));

    // Create dirty rects and upload ops resources
    commands.insert_resource(CanvasDirtyRects::new(config.num_chunks()));
    commands.insert_resource(CanvasUploadOps::default());
}
