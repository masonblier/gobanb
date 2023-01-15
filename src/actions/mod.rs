use bevy::prelude::*;

mod key_input;
mod mouse_input;
mod camera_movement;

pub struct ActionsPlugin;
pub use mouse_input::{CursorLockState,MouseCamera,MouseLookState};

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(key_input::KeyInputPlugin)
            .add_plugin(mouse_input::MouseInputPlugin)
            .add_plugin(camera_movement::CamerasStatePlugin)
            ;
    }
}
