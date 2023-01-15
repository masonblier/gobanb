use crate::game_state::GameState;
use crate::actions::{key_input::KeyInputState,mouse_input::MouseCamera,mouse_input::MouseLookState};
use bevy::prelude::*;

const CAMERA_FLY_MOVE_SPEED: f32 = 1.0;

// system state
#[derive(Default, Resource)]
pub struct CamerasState {
}

pub struct CamerasStatePlugin;

impl Plugin for CamerasStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamerasState>();
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_camera_movement)
        );
    }
}

// update camera position from movement
fn update_camera_movement(
    time: Res<Time>,
    key_state: Res<KeyInputState>,
    mouse_look: Res<MouseLookState>,
    mut query: Query<&mut Transform, With<MouseCamera>>,
) {
    let mut camera = query.single_mut();
    let run_mult = if key_state.run { 5.0 } else { 1.0 };

    let camera_move = CAMERA_FLY_MOVE_SPEED * run_mult * time.delta_seconds() * (
        if key_state.forward { mouse_look.forward.clone() } else { Vec3::ZERO } +
        if key_state.backward { -mouse_look.forward.clone() } else { Vec3::ZERO } +
        if key_state.right { mouse_look.right.clone() } else { Vec3::ZERO } +
        if key_state.left { -mouse_look.right.clone() } else { Vec3::ZERO } +
        if key_state.up { mouse_look.up.clone() } else { Vec3::ZERO } +
        if key_state.down { -mouse_look.up.clone() } else { Vec3::ZERO }
    );

    let next_position = camera.translation + camera_move;
    camera.translation = next_position.clone();
    camera.look_at(next_position + mouse_look.forward, Vec3::Y);
}
