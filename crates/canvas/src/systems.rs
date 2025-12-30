use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    math::U8Vec2,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            Extent3d, Origin3d, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect, TextureDimension, TextureFormat,
            TextureUsages,
        },
        renderer::RenderQueue,
        texture::GpuImage,
    },
};

use super::{
    components::CanvasImage,
    config::CanvasConfig,
    messages::{DrawPixel, DrawRect, DrawSpan},
    resources::{CanvasCpuChunks, CanvasDirtyRects, CanvasImageHandles, CanvasUploadOps},
    types::{CanvasUploadOp, SubRect},
};

pub fn spawn_canvas_images(mut commands: Commands, config: Res<CanvasConfig>, mut images: ResMut<Assets<Image>>) {
    let num_chunks = config.num_chunks();
    let chunk_size = config.chunk_size();
    let pixels_per_chunk = config.pixels_per_chunk();

    // Create default RGBA8 data for one chunk
    let [r, g, b, a] = *config.clear_colour();
    let mut data = vec![0u8; pixels_per_chunk * 4];
    for px in data.chunks_exact_mut(4) {
        px.copy_from_slice(&[r, g, b, a]);
    }

    let mut image_handles = Vec::with_capacity(config.total_chunks());
    for y in 0..num_chunks.y {
        for x in 0..num_chunks.x {
            // Create image for this chunk
            let mut image = Image::new(
                Extent3d {
                    width: chunk_size.x,
                    height: chunk_size.y,
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
                    (x as f32 - ((num_chunks.x - 1) as f32 / 2.0)) * chunk_size.x as f32,
                    (y as f32 - ((num_chunks.y - 1) as f32 / 2.0)) * chunk_size.y as f32,
                    config.canvas_z_index(),
                ))
                .with_scale(Vec3::new(1.0, -1.0, 1.0)),
            ));
        }
    }

    // Store handles and create CPU chunk storage
    commands.insert_resource(CanvasImageHandles::new(num_chunks, image_handles));

    // Create CPU chunk storage
    let [r, g, b, a] = *config.clear_colour();
    let default_px = u32::from_ne_bytes([r, g, b, a]);
    commands.insert_resource(CanvasCpuChunks::new(num_chunks, chunk_size, default_px));

    // Create dirty rects and upload ops resources
    commands.insert_resource(CanvasDirtyRects::new(num_chunks, chunk_size));
    commands.insert_resource(CanvasUploadOps::default());
}

pub fn collect_ops(
    config: Res<CanvasConfig>,
    mut draw_pixel_msg: MessageReader<DrawPixel>,
    mut draw_rect_msg: MessageReader<DrawRect>,
    mut draw_span_msg: MessageReader<DrawSpan>,
    canvas_image_handles: Res<CanvasImageHandles>,
    mut canvas_cpu_chunks: ResMut<CanvasCpuChunks>,
    mut canvas_dirty_rects: ResMut<CanvasDirtyRects>,
    mut canvas_upload_ops: ResMut<CanvasUploadOps>,
) {
    let canvas_size = config.canvas_size();
    let chunk_size = config.chunk_size();

    // Pixels
    for DrawPixel { pos, rgba_u32 } in draw_pixel_msg.read() {
        blit_pixel(
            &mut canvas_cpu_chunks,
            &mut canvas_dirty_rects,
            canvas_size,
            chunk_size,
            *pos,
            *rgba_u32,
        );
    }

    // Rects
    for DrawRect { start, size, rgba_u32 } in draw_rect_msg.read() {
        if size.x == 0 || size.y == 0 {
            continue;
        }

        let expected = (size.x * size.y) as usize;
        if rgba_u32.len() != expected {
            warn!(
                "DrawRect rgba_u32 length mismatch (expected {}, got {})",
                expected,
                rgba_u32.len()
            );
            continue;
        }

        blit_rect_torus_into_chunks(
            &mut canvas_cpu_chunks,
            &mut canvas_dirty_rects,
            *start,
            *size,
            rgba_u32,
            canvas_size,
            chunk_size,
        );
    }

    // Spans
    for DrawSpan { start, rgba_u32 } in draw_span_msg.read() {
        if rgba_u32.is_empty() {
            continue;
        }

        blit_span_row_major(
            &mut canvas_cpu_chunks,
            &mut canvas_dirty_rects,
            canvas_size,
            chunk_size,
            *start,
            rgba_u32,
        );
    }

    build_upload_ops(
        &canvas_image_handles,
        &canvas_cpu_chunks,
        &mut canvas_dirty_rects,
        &mut canvas_upload_ops,
        chunk_size,
    );
}

pub fn apply_canvas_uploads(
    mut uploads: ResMut<CanvasUploadOps>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_queue: Res<RenderQueue>,
) {
    if uploads.ops.is_empty() {
        return;
    }

    for op in uploads.ops.drain(..) {
        let Some(gpu) = gpu_images.get(op.handle.id()) else {
            // image not ready on GPU yet
            continue;
        };

        debug_assert!(op.bytes_per_row % 256 == 0);
        debug_assert_eq!(op.bytes.len(), (op.bytes_per_row as usize) * (op.size.y as usize));
        debug_assert!(op.start.x + op.size.x <= gpu.size.width);
        debug_assert!(op.start.y + op.size.y <= gpu.size.height);

        render_queue.write_texture(
            TexelCopyTextureInfo {
                texture: &gpu.texture,
                mip_level: 0,
                origin: Origin3d {
                    x: op.start.x,
                    y: op.start.y,
                    z: 0,
                },
                aspect: TextureAspect::All,
            },
            &op.bytes,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(op.bytes_per_row),
                rows_per_image: Some(op.size.y),
            },
            Extent3d {
                width: op.size.x,
                height: op.size.y,
                depth_or_array_layers: 1,
            },
        );
    }
}

// -- Helpers --

#[inline]
fn blit_pixel(
    cpu: &mut CanvasCpuChunks,
    dirty: &mut CanvasDirtyRects,
    canvas_size: UVec2,
    chunk_size: UVec2,
    pos: UVec2,
    rgba_u32: u32,
) {
    debug_assert!(canvas_size.x > 0 && canvas_size.y > 0);
    debug_assert!(chunk_size.x > 0 && chunk_size.y > 0);

    // Toroidal wrap
    let p = UVec2::new(pos.x % canvas_size.x, pos.y % canvas_size.y);

    // Chunk coord
    let chunk_xy = UVec2::new(p.x / chunk_size.x, p.y / chunk_size.y);
    debug_assert!(chunk_xy.x < 256 && chunk_xy.y < 256);

    let chunk_key = U8Vec2::new(chunk_xy.x as u8, chunk_xy.y as u8);

    // Chunk-local coord
    let chunk_min = UVec2::new(chunk_xy.x * chunk_size.x, chunk_xy.y * chunk_size.y);
    let local = p - chunk_min;

    // Write
    let dst = cpu.chunk_mut(&chunk_key);
    let stride = chunk_size.x as usize;
    let idx = local.y as usize * stride + local.x as usize;
    debug_assert!(idx < dst.len());
    dst[idx] = rgba_u32;

    // Dirty 1x1
    dirty.mark_rect(&chunk_key, local, UVec2::ONE);
}

#[inline]
fn blit_rect_torus_into_chunks(
    canvas_cpu_chunks: &mut CanvasCpuChunks,
    canvas_dirty_rects: &mut CanvasDirtyRects,
    start: UVec2,
    size: UVec2,
    src_u32: &[u32],
    canvas_size: UVec2,
    chunk_size: UVec2,
) {
    let (sub_rects, n) = split_torus_rect(start, size, canvas_size);
    for sub_rect in sub_rects[..n].iter().copied() {
        blit_subrect_into_chunks(canvas_cpu_chunks, canvas_dirty_rects, size, src_u32, sub_rect, chunk_size);
    }
}

/// Returns up to 4 sub-rects (count returned) that are fully inside the canvas bounds.
/// Each sub-rect knows where to read from the original `size` source buffer via `src_start`.
#[inline]
fn split_torus_rect(start: UVec2, size: UVec2, canvas_size: UVec2) -> ([SubRect; 4], usize) {
    let mut out = [SubRect {
        dst_start: UVec2::ZERO,
        size: UVec2::ZERO,
        src_start: UVec2::ZERO,
    }; 4];

    if size.x == 0 || size.y == 0 || canvas_size.x == 0 || canvas_size.y == 0 {
        return (out, 0);
    }

    // Currently only supports wrapping at most once per axis
    debug_assert!(size.x <= canvas_size.x, "Rect width exceeds canvas width");
    debug_assert!(size.y <= canvas_size.y, "Rect height exceeds canvas height");

    let wrapped_start = UVec2::new(start.x % canvas_size.x, start.y % canvas_size.y);
    let remain = canvas_size - wrapped_start;

    let in_bounds = UVec2::new(size.x.min(remain.x), size.y.min(remain.y));
    let wrapped = size - in_bounds;

    let mut n = 0usize;

    out[n] = SubRect {
        src_start: UVec2::ZERO,
        dst_start: wrapped_start,
        size: in_bounds,
    };
    n += 1;

    if wrapped.x != 0 {
        out[n] = SubRect {
            src_start: UVec2::new(in_bounds.x, 0),
            dst_start: UVec2::new(0, wrapped_start.y),
            size: UVec2::new(wrapped.x, in_bounds.y),
        };
        n += 1;
    }

    if wrapped.y != 0 {
        out[n] = SubRect {
            src_start: UVec2::new(0, in_bounds.y),
            dst_start: UVec2::new(wrapped_start.x, 0),
            size: UVec2::new(in_bounds.x, wrapped.y),
        };
        n += 1;
    }

    if wrapped.x != 0 && wrapped.y != 0 {
        out[n] = SubRect {
            src_start: in_bounds,
            dst_start: UVec2::ZERO,
            size: wrapped,
        };
        n += 1;
    }

    (out, n)
}

#[inline]
fn blit_subrect_into_chunks(
    canvas_cpu_chunks: &mut CanvasCpuChunks,
    canvas_dirty_rects: &mut CanvasDirtyRects,
    orig_size: UVec2,
    src_u32: &[u32],
    sub_rect: SubRect,
    chunk_size: UVec2, // (chunk_w, chunk_h)
) {
    let chunk_w = chunk_size.x;
    let chunk_h = chunk_size.y;
    debug_assert!(chunk_w > 0 && chunk_h > 0);

    if sub_rect.size.x == 0 || sub_rect.size.y == 0 {
        return;
    }

    let sub_min = sub_rect.dst_start;
    let sub_max = sub_rect.dst_start + sub_rect.size; // exclusive

    // Which chunks does this sub-rect touch?
    let chunk_start = UVec2::new(sub_min.x / chunk_w, sub_min.y / chunk_h);
    let sub_max_minus_one = sub_max - UVec2::ONE;
    let chunk_end = UVec2::new(sub_max_minus_one.x / chunk_w, sub_max_minus_one.y / chunk_h);

    let src_stride = orig_size.x as usize;

    for cy in chunk_start.y..=chunk_end.y {
        for cx in chunk_start.x..=chunk_end.x {
            let chunk_key = U8Vec2::new(cx as u8, cy as u8);

            let chunk_min = UVec2::new(cx * chunk_w, cy * chunk_h);
            let chunk_max = chunk_min + chunk_size; // exclusive

            let intersection_min = UVec2::new(sub_min.x.max(chunk_min.x), sub_min.y.max(chunk_min.y));
            let intersection_max = UVec2::new(sub_max.x.min(chunk_max.x), sub_max.y.min(chunk_max.y));
            let intersection_size = intersection_max - intersection_min;
            if intersection_size.x == 0 || intersection_size.y == 0 {
                continue;
            }

            let local_min = intersection_min - chunk_min;

            let copy = intersection_size.x as usize;
            let rows = intersection_size.y as usize;

            let src_min = sub_rect.src_start + (intersection_min - sub_rect.dst_start);

            let dst_u32 = canvas_cpu_chunks.chunk_mut(&chunk_key);

            // Row stride is chunk_w
            let dst_stride = chunk_w as usize;

            for row in 0..rows {
                let src_index = (src_min.y as usize + row) * src_stride + (src_min.x as usize);
                let dst_index = (local_min.y as usize + row) * dst_stride + (local_min.x as usize);
                debug_assert!(src_index + copy <= src_u32.len());
                debug_assert!(dst_index + copy <= dst_u32.len());

                dst_u32[dst_index..dst_index + copy].copy_from_slice(&src_u32[src_index..src_index + copy]);
            }

            canvas_dirty_rects.mark_rect(&chunk_key, local_min, intersection_size);
        }
    }
}

/// Convert dirty rects to upload ops (CPU snapshot of touched rows)
#[inline]
fn build_upload_ops(
    canvas_image_handles: &CanvasImageHandles,
    canvas_cpu_chunks: &CanvasCpuChunks,
    canvas_dirty_rects: &mut CanvasDirtyRects,
    canvas_upload_ops: &mut CanvasUploadOps,
    chunk_size: UVec2,
) {
    canvas_upload_ops.ops.clear();

    let chunk_w = chunk_size.x;
    let chunk_h = chunk_size.y;

    const ROW_ALIGN_PX: u32 = 64;

    for chunk_index in 0..canvas_dirty_rects.len() {
        let Some((min, max)) = canvas_dirty_rects.take(chunk_index) else {
            continue;
        };

        let min_ex = min;
        let max_ex = max + UVec2::ONE;

        // Safety clamp (helps if a bug elsewhere overmarks)
        let min_ex = UVec2::new(
            min_ex.x.min(chunk_w.saturating_sub(1)),
            min_ex.y.min(chunk_h.saturating_sub(1)),
        );
        let max_ex = UVec2::new(max_ex.x.min(chunk_w), max_ex.y.min(chunk_h));

        // Pad X to 64px for bytes_per_row alignment
        let padding_min_x = (min_ex.x / ROW_ALIGN_PX) * ROW_ALIGN_PX;
        let padding_max_x = (((max_ex.x + ROW_ALIGN_PX - 1) / ROW_ALIGN_PX) * ROW_ALIGN_PX).min(chunk_w);

        let padded_width = padding_max_x.saturating_sub(padding_min_x);
        if padded_width == 0 {
            continue;
        }

        let height = max_ex.y.saturating_sub(min_ex.y);
        if height == 0 {
            continue;
        }

        let bytes_per_row = padded_width * 4;
        debug_assert_eq!(bytes_per_row % 256, 0);

        let handle = canvas_image_handles.handle(chunk_index).clone();
        let chunk = canvas_cpu_chunks.chunk(chunk_index);

        let mut bytes = Vec::with_capacity((bytes_per_row as usize) * (height as usize));

        let row_stride = chunk_w as usize;

        for y in min_ex.y..max_ex.y {
            let row_start = (y as usize) * row_stride;
            let x0 = padding_min_x as usize;
            let x1 = (padding_min_x + padded_width) as usize;

            let src = &chunk[row_start + x0..row_start + x1];
            bytes.extend_from_slice(bytemuck::cast_slice(src));
        }

        canvas_upload_ops.ops.push(CanvasUploadOp {
            handle,
            start: UVec2::new(padding_min_x, min_ex.y),
            size: UVec2::new(padded_width, height),
            bytes_per_row,
            bytes,
        });
    }
}

#[inline]
fn blit_span_row_major(
    cpu: &mut CanvasCpuChunks,
    dirty: &mut CanvasDirtyRects,
    canvas_size: UVec2,
    chunk_size: UVec2,
    start: UVec2,
    src_u32: &[u32],
) {
    debug_assert!(canvas_size.x > 0 && canvas_size.y > 0);
    debug_assert!(chunk_size.x > 0 && chunk_size.y > 0);

    if src_u32.is_empty() {
        return;
    }

    // Cursor in wrapped canvas coords
    let mut cursor = UVec2::new(start.x % canvas_size.x, start.y % canvas_size.y);

    let mut src_index = 0usize;
    let mut remaining = src_u32.len();

    while remaining > 0 {
        // Chunk coords and chunk-local coords
        let chunk_xy_u32 = UVec2::new(cursor.x / chunk_size.x, cursor.y / chunk_size.y);
        debug_assert!(chunk_xy_u32.x < 256 && chunk_xy_u32.y < 256);

        let chunk_key = U8Vec2::new(chunk_xy_u32.x as u8, chunk_xy_u32.y as u8);

        let chunk_min = UVec2::new(chunk_xy_u32.x * chunk_size.x, chunk_xy_u32.y * chunk_size.y);
        let local_xy = cursor - chunk_min;

        // Remaining pixels to end of canvas row and chunk row
        let until_canvas_row_end = (canvas_size.x - cursor.x) as usize;
        let until_chunk_row_end = (chunk_size.x - local_xy.x) as usize;

        let run = remaining.min(until_canvas_row_end).min(until_chunk_row_end);
        debug_assert!(run > 0);

        // Write into chunk row-major storage (stride = chunk_size.x)
        let dst = cpu.chunk_mut(&chunk_key);
        let dst_stride = chunk_size.x as usize;
        let dst_index = (local_xy.y as usize) * dst_stride + (local_xy.x as usize);

        debug_assert!(dst_index + run <= dst.len());
        debug_assert!(src_index + run <= src_u32.len());

        dst[dst_index..dst_index + run].copy_from_slice(&src_u32[src_index..src_index + run]);

        // Mark dirty: a 1-row-high span in chunk-local coords
        dirty.mark_rect(&chunk_key, UVec2::new(local_xy.x, local_xy.y), UVec2::new(run as u32, 1));

        // Advance
        src_index += run;
        remaining -= run;

        // Move cursor along the canvas in row-major order
        cursor.x += run as u32;
        if cursor.x == canvas_size.x {
            cursor.x = 0;
            cursor.y += 1;
            if cursor.y == canvas_size.y {
                cursor.y = 0;
            }
        }
    }
}
