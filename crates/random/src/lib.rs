use bevy::prelude::*;

pub mod resources;

use resources::*;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.init_resource::<SeededRng>();
    }
}
