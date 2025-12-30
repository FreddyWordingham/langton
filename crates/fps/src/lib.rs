use std::time::Duration;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    prelude::*,
};

pub mod messages;
mod settings;
mod systems;

use messages::*;
use settings::*;
use systems::*;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                text_color: TEXT_COLOUR,
                refresh_interval: Duration::from_millis(REFRESH_INTERVAL),
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: true,
                    min_fps: MIN_FPS,
                    target_fps: TARGET_FPS,
                },
            },
        });

        // Messages
        app.add_message::<ToggleFpsDisplay>();

        // Systems
        app.add_systems(Update, handle_toggle_fps_display);
    }
}
