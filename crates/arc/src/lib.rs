use arc_camera::CameraPlugin;
use arc_canvas::CanvasPlugin;
use arc_fps::FpsPlugin;
use arc_random::RandomPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct ArcPlugin;

impl Plugin for ArcPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin::default())
            .add_plugins((
                CameraPlugin,
                CanvasPlugin {
                    config: Default::default(),
                },
                FpsPlugin,
                RandomPlugin,
            ));

        // Systems
        app.add_systems(Update, mutate_canvas);
    }
}

use arc_canvas::messages::DrawPixel;
use arc_random::resources::SeededRng;
use rand::Rng;

fn mutate_canvas(mut seeded_rng: ResMut<SeededRng>, mut draw_pixel_msg: MessageWriter<DrawPixel>) {
    let rng = seeded_rng.rng();

    for _ in 0..10 {
        let x = rng.random_range(0..1024);
        let y = rng.random_range(0..512);

        let r = rng.random_range(0..=255);
        let g = rng.random_range(0..=255);
        let b = rng.random_range(0..=255);
        let a = 255;
        let rgba_u32 = u32::from_ne_bytes([r, g, b, a]);

        draw_pixel_msg.write(DrawPixel {
            pos: UVec2::new(x, y),
            rgba_u32,
        });
    }
}

// fn mutate_canvas(mut seeded_rng: ResMut<SeededRng>, mut draw_span_msg: MessageWriter<DrawSpan>) {
//     let rng = seeded_rng.rng();

//     for _ in 0..10 {
//         let x = rng.random_range(0..1024);
//         let y = rng.random_range(0..512);
//         let l = rng.random_range(10..=50);

//         let r = rng.random_range(0..=255);
//         let g = rng.random_range(0..=255);
//         let b = rng.random_range(0..=255);
//         let a = 255;
//         let pixel = u32::from_ne_bytes([r, g, b, a]);
//         let rgba_u32 = vec![pixel; l as usize];

//         draw_span_msg.write(DrawSpan {
//             start: UVec2::new(x, y),
//             rgba_u32,
//         });
//     }
// }

// fn mutate_canvas(mut seeded_rng: ResMut<SeededRng>, mut draw_rect_msg: MessageWriter<DrawRect>) {
//     let rng = seeded_rng.rng();

//     for _ in 0..10 {
//         let x = rng.random_range(0..1024);
//         let y = rng.random_range(0..512);
//         let w = rng.random_range(10..=10);
//         let h = rng.random_range(10..=10);

//         let r = rng.random_range(0..=255);
//         let g = rng.random_range(0..=255);
//         let b = rng.random_range(0..=255);
//         let a = 255;
//         let pixel = u32::from_ne_bytes([r, g, b, a]);
//         let rgba_u32 = vec![pixel; (w * h) as usize];

//         draw_rect_msg.write(DrawRect {
//             start: UVec2::new(x, y),
//             size: UVec2::new(w, h),
//             rgba_u32,
//         });
//     }
// }

// fn mutate_canvas(mut draw_rect_msg: MessageWriter<DrawRect>, mut seeded_rng: ResMut<SeededRng>, mut pos: Local<UVec2>) {
//     let rng = seeded_rng.rng();

//     pos.x = (pos.x + 7) % 1024;
//     pos.y = (pos.y + 3) % 512;

//     let w = 5;
//     let h = 5;

//     let r = rng.random_range(0..=255);
//     let g = rng.random_range(0..=255);
//     let b = rng.random_range(0..=255);
//     let a = 255;
//     let pixel = u32::from_ne_bytes([r, g, b, a]);
//     let rgba_u32 = vec![pixel; (w * h) as usize];

//     draw_rect_msg.write(DrawRect {
//         start: UVec2::new(pos.x, pos.y),
//         size: UVec2::new(w, h),
//         rgba_u32,
//     });
// }
