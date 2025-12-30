use bevy::{
    prelude::*,
    render::{Render, RenderApp, RenderSystems, extract_resource::ExtractResourcePlugin},
};

mod components;
pub mod config;
pub mod messages;
mod resources;
mod systems;
mod types;

use config::*;
use messages::*;
use resources::*;
use systems::*;

pub struct CanvasPlugin {
    pub config: CanvasConfig,
}

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins(ExtractResourcePlugin::<CanvasUploadOps>::default());

        // Messages
        app.add_message::<DrawPixel>()
            .add_message::<DrawRect>()
            .add_message::<DrawSpan>();

        // Resources
        app.insert_resource(self.config.clone());

        // Systems
        app.add_systems(Startup, spawn_canvas_images).add_systems(Update, collect_ops);

        // Render-world systems
        app.sub_app_mut(RenderApp)
            .add_systems(Render, apply_canvas_uploads.in_set(RenderSystems::Queue));
    }
}
