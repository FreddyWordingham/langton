use arc_camera::CameraPlugin;
use arc_fps::FpsPlugin;
use arc_langton::LangtonPlugin;
use arc_random::RandomPlugin;
use bevy::{math::U8Vec2, prelude::*};
use bevy_canvas_2d::prelude::*;
use bevy_egui::EguiPlugin;

const BOARD_SIZE: UVec2 = UVec2::new(1024 * 4, 1024 * 4);

pub struct ArcPlugin;

impl Plugin for ArcPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin::default())
            .add_plugins(CanvasPlugin {
                config: CanvasConfig {
                    canvas_size: BOARD_SIZE,
                    num_chunks: U8Vec2::new(1, 1),
                    ..default()
                },
            })
            .add_plugins((CameraPlugin, FpsPlugin, LangtonPlugin, RandomPlugin));
    }
}
