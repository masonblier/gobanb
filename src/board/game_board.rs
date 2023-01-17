use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Resource)]
pub struct GameBoard {
    pub spaces: HashMap<(usize,usize),usize>,
}

pub enum GameBoardEffectType {
    TogglePlayer,
    AddPiece(GameBoardMove),
    RemovePiece(GameBoardMove),
}
pub struct GameBoardEffect {
    pub effect: GameBoardEffectType,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct GameBoardMove {
    pub player: usize,
    pub space: (usize, usize),
}

pub fn try_move(game_board: &mut GameBoard, tried_move: GameBoardMove) -> Vec<GameBoardEffect> {
    if game_board.spaces.contains_key(&tried_move.space) {
        vec![]
    } else {
        // add piece to board
        game_board.spaces.insert(tried_move.space, tried_move.player);

        let mut effects = vec![
            GameBoardEffect { effect: GameBoardEffectType::AddPiece(tried_move) },
            GameBoardEffect { effect: GameBoardEffectType::TogglePlayer },
        ];

        // check for surrounded groups to remove
        let surrounded = get_surrounded_positions(game_board, tried_move);
        for pos in surrounded {
            let piece_player = game_board.spaces.remove(&pos).unwrap();
            effects.push(GameBoardEffect { effect: GameBoardEffectType::RemovePiece(
                GameBoardMove { player: piece_player, space: pos }) })
        }

        effects
    }
}

pub fn get_surrounded_positions(game_board: &GameBoard, last_move: GameBoardMove) -> Vec<(usize,usize)> {
    let mut group_stats: Vec<(usize,bool)> = vec![];
    let mut space_groups = HashMap::<(usize,usize),usize>::new();
    let mut last_move_group: Option<usize> = None;

    for i in 0..19 {
        for j in 0..19 {
            if game_board.spaces.contains_key(&(i, j)) {
                let space_player = *game_board.spaces.get(&(i, j)).unwrap();
                let group_id = if i > 0 && space_groups.contains_key(&(i - 1, j)) && (*game_board.spaces.get(&(i - 1, j)).unwrap() == space_player) {
                    *space_groups.get(&(i - 1, j)).unwrap()
                } else if j > 0 && game_board.spaces.contains_key(&(i, j - 1)) && (*game_board.spaces.get(&(i, j - 1)).unwrap() == space_player) {
                    *space_groups.get(&(i, j - 1)).unwrap()
                } else {
                    group_stats.push((space_player, false));
                    group_stats.len() - 1
                };
                group_stats[group_id].1 = group_stats[group_id].1 ||
                    (i > 0 && !game_board.spaces.contains_key(&(i - 1, j))) ||
                    (j > 0 && !game_board.spaces.contains_key(&(i, j - 1))) ||
                    (i < 18 && !game_board.spaces.contains_key(&(i + 1, j))) ||
                    (j < 18 && !game_board.spaces.contains_key(&(i, j + 1)));
                space_groups.insert((i, j), group_id);
                if i == last_move.space.0 && j == last_move.space.1 {
                    last_move_group = Some(group_id);
                }
            }
        }
    }

    let mut surrounded_pos: Vec<(usize,usize)> = vec![];

    // todo handle self-kill case
    for (space_key, group_id) in space_groups {
        if (!group_stats[group_id].1) && (last_move_group != Some(group_id)) {
            surrounded_pos.push(space_key);
        }
    }

    surrounded_pos
}