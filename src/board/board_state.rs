use crate::actions::MouseCamera;
use crate::board::game_board::{GameBoard, GameBoardMove, GameBoardEffectType, try_move};
use crate::game_state::GameState;
use crate::loading::PreLoadingState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::HashMap;

const SPACING: f32 = 0.015;

pub struct BoardStatePlugin;

#[derive(Default, Resource)]
pub struct BoardState {
    player_turn: usize,
    pause_actions: f32,
    light_stone: Handle<StandardMaterial>,
    dark_stone: Handle<StandardMaterial>,
    stone_mesh: Handle<Mesh>,
    piece_ents: HashMap<(usize,usize),Entity>,
}


#[derive(Default, Component)]
pub struct BoardActivePiece {
    player: usize,
}

impl Plugin for BoardStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BoardState>()
            .init_resource::<GameBoard>()
            .add_system_set(SystemSet::on_enter(GameState::Running).with_system(setup_world_loading))
            .add_system_set(SystemSet::on_update(GameState::Running).with_system(update_board_state))
            ;
    }
}


fn setup_world_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board_state: ResMut<BoardState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    board_state.pause_actions = 0.2;

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        point_light: PointLight {
            intensity: 6000000.,
            range: 10000.,
            ..default()
        },
        ..default()
    });

    board_state.dark_stone = materials.add(StandardMaterial {
        base_color: Color::hex("20312f").unwrap(),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });
    board_state.light_stone = materials.add(StandardMaterial {
        base_color: Color::hex("ffffff").unwrap(),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    board_state.stone_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.0072,
        ..default()
    }));

    // active piece
    commands.spawn(PbrBundle {
        mesh: board_state.stone_mesh.clone(),
        material: board_state.dark_stone.clone(),
        transform: Transform::from_xyz(0.,  -9999., 0.).with_scale(Vec3::new(1.0,0.5,1.0)),
        ..default()
    }).insert(BoardActivePiece { player: 0 });
    commands.spawn(PbrBundle {
        mesh: board_state.stone_mesh.clone(),
        material: board_state.light_stone.clone(),
        transform: Transform::from_xyz(0.,  -9999., 0.).with_scale(Vec3::new(1.0,0.5,1.0)),
        ..default()
    }).insert(BoardActivePiece { player: 1 });

    // board
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/goban_2k.glb#Scene0"),
        ..default()
    });
    commands.spawn(SpatialBundle {
        transform: Transform::from_scale(Vec3::new(0.29, 0.01, 0.29)),
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(0.5,0.5,0.5));
}



fn update_board_state(
    mut commands: Commands,
    mut board_state: ResMut<BoardState>,
    mut game_board: ResMut<GameBoard>,
    preloading: ResMut<PreLoadingState>,
    audio: Res<Audio>,
    time: Res<Time>,
    windows: Res<Windows>,
    rapier_context: Res<RapierContext>,
    cameras_query: Query<(&Camera, &GlobalTransform, With<MouseCamera>)>,
    mut active_piece_query: Query<(&mut Transform, &BoardActivePiece)>,
    mouse_btn_input: Res<Input<MouseButton>>,
) {
    // pause actions
    if board_state.pause_actions > 0.0 {
        board_state.pause_actions -= time.delta_seconds();
        return;
    }

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform, _mc) in cameras_query.iter() {

        let (ray_pos, ray_dir) =
            ray_from_mouse_position(windows.get_primary().unwrap(), camera, camera_transform);

        // Then cast the ray.
        let hit = rapier_context.cast_ray(
            ray_pos,
            ray_dir,
            20.,
            true,
            QueryFilter::only_fixed(),
        );


        let player_turn = board_state.player_turn;
        let active_transform = if let Some((_entity, toi)) = hit {
            let space_pos = ray_pos + ray_dir * toi;
            let space_x = (space_pos.x * (1. / SPACING) + 0.5).floor().min(9.).max(-9.) + 9.;
            let space_z = (space_pos.z * (1. / SPACING) + 0.5).floor().min(9.).max(-9.) + 9.;

            let space_key = (space_x as usize, space_z as usize);
            if game_board.spaces.contains_key(&space_key) {
                Vec3::Y * -9999.
            } else {
                if mouse_btn_input.just_released(MouseButton::Left) {
                    let effects = try_move(&mut game_board, GameBoardMove { player: player_turn, space: space_key});
                    for effect in effects.iter() {
                        match effect.effect {
                            GameBoardEffectType::AddPiece(tried_move) => {
                                let space_coords = Vec3::new(
                                    tried_move.space.0 as f32 - 9.,
                                    0.5,
                                    tried_move.space.1 as f32 - 9.,
                                );
                                let ent_id = commands.spawn(PbrBundle {
                                    mesh: board_state.stone_mesh.clone(),
                                    material: if tried_move.player == 1 { board_state.light_stone.clone() } else { board_state.dark_stone.clone() },
                                    transform: Transform::from_translation(SPACING * space_coords).with_scale(Vec3::new(1.0,0.5,1.0)),
                                    ..default()
                                }).id();
                                board_state.piece_ents.insert(space_key, ent_id);
                            }
                            GameBoardEffectType::RemovePiece(removed_move) => {
                                if let Some(ent_id) = board_state.piece_ents.remove(&removed_move.space) {
                                    commands.entity(ent_id).despawn_recursive();
                                }
                            }
                            GameBoardEffectType::TogglePlayer => {
                                audio.play(if player_turn == 0 {
                                    preloading.sound_01.clone()
                                } else {
                                    preloading.sound_02.clone()
                                });
                                board_state.player_turn = (player_turn + 1) % 2;
                            }
                        }
                    }

                    board_state.pause_actions = 0.05;
                }
                SPACING * Vec3::new(space_x - 9.,0.5,space_z - 9.)
            }
        } else {
            Vec3::Y * -9999.
        };

        for (mut ap_transform, bap) in active_piece_query.iter_mut() {
            if bap.player == player_turn {
                ap_transform.translation = active_transform;
            } else {
                ap_transform.translation = Vec3::Y * -9999.;
            }
        }


    }
}


// Credit to @doomy on discord.
fn ray_from_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));

    let x = 2.0 * (mouse_position.x / window.width() as f32) - 1.0;
    let y = 2.0 * (mouse_position.y / window.height() as f32) - 1.0;

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}
