use bevy::prelude::*;
use bevy_egui::input::EguiWantsInput;

pub fn egui_not_wanting_keyboard(egui_wants_input: Res<EguiWantsInput>) -> bool {
    !egui_wants_input.wants_keyboard_input()
}
