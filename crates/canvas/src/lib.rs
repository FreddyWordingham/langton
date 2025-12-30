use bevy::{prelude::*, render::extract_resource::ExtractResourcePlugin};

mod components;
pub mod config;
mod resources;
mod systems;
mod types;

use config::*;
use resources::*;
use systems::*;

pub struct CanvasPlugin {
    pub config: CanvasConfig,
}

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins(ExtractResourcePlugin::<CanvasUploadOps>::default());

        // Resources
        app.insert_resource(self.config.clone());

        // Systems
        app.add_systems(Startup, spawn_canvas_images);
    }
}
