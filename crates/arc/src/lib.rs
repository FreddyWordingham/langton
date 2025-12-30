use arc_camera::CameraPlugin;
use arc_canvas::CanvasPlugin;
use arc_fps::FpsPlugin;
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
            ));
    }
}
