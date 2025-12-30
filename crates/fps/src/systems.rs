use bevy::{dev_tools::fps_overlay::FpsOverlayConfig, prelude::*};

use super::messages::ToggleFpsDisplay;

pub fn handle_toggle_fps_display(
    mut toggle_fps_display_msg: MessageReader<ToggleFpsDisplay>,
    mut fps_overlay_config: ResMut<FpsOverlayConfig>,
) {
    for ToggleFpsDisplay in toggle_fps_display_msg.read() {
        fps_overlay_config.enabled = !fps_overlay_config.enabled;
        fps_overlay_config.frame_time_graph_config.enabled = !fps_overlay_config.frame_time_graph_config.enabled;
    }
}
