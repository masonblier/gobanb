use crate::game_state::GameState;
use crate::actions::MouseCamera;
use bevy::prelude::*;

pub struct PreLoadingPlugin;

#[derive(Default, Resource)]
pub struct PreLoadingState {
    pub pre_loaded: bool,
    pub font_handle: Handle<Font>,
    pub ui_entity: Option<Entity>,
}

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for PreLoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PreLoadingState>()
            .add_system_set(SystemSet::on_enter(GameState::PreLoading)
                .with_system(setup_pre_loading))
            .add_system_set(SystemSet::on_update(GameState::PreLoading).with_system(update_pre_loading));
    }
}

fn setup_pre_loading(
    mut commands: Commands,
    mut pre_loading_state: ResMut<PreLoadingState>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.3, 0.2),
        ..Default::default()
    })
    .insert(UiCameraConfig {
        show_ui: true,
        ..default()
    })
    .insert(MouseCamera::default());

    // Load font
    pre_loading_state.font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

}

fn update_pre_loading(
    font_assets: Res<Assets<Font>>,
    mut pre_loading_state: ResMut<PreLoadingState>,
    mut state: ResMut<State<GameState>>,
) {
    let font_asset = font_assets.get(&pre_loading_state.font_handle);
    if pre_loading_state.pre_loaded || font_asset.is_none() {
        return;
    }

    info!("Pre loaded: {:?}", font_asset.unwrap());
    pre_loading_state.pre_loaded = true;
    state.set(GameState::Menu).unwrap();
}
