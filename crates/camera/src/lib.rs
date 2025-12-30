use bevy::prelude::*;

mod conditions;
mod settings;
mod systems;

use conditions::*;
use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Systems
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, ((zoom_camera, pan_camera).run_if(egui_not_wanting_keyboard),));
    }
}
