use itertools::Itertools;

use crate::enums::general::*;
use crate::enums::stage::Stage;
use crate::stats::inputs::find_inputs;
use crate::utils::BitFlags;
use crate::{
    enums::{buttons::*, general::*, state::*},
    Game,
};

use super::defense::find_defense;
use super::items::find_items;
use super::lcancel::find_lcancels;

pub fn get_stats(game: &mut Game) {
    for players in game.players.iter().permutations(2) {
        let mut player = players[0].as_ref().write().unwrap();
        let opponent = players[1].as_ref().read().unwrap();
        let items = &game.item_frames;

        player.stats.l_cancel = find_lcancels(&player.frames, Stage::from_id(game.start.stage));
        player.stats.actions = find_inputs(&player.frames, game.total_frames);
        player.stats.items = find_items(&player.frames, player.port, items);
        player.stats.defense = find_defense(&player.frames, &opponent.frames);
    }
}

pub fn just_input_lcancel(frames: &[u32], i: usize) -> bool {
    let current = EngineInput::from(frames[i]);
    let previous = EngineInput::from(frames[i.saturating_sub(1)]);

    let mask: u32 = (EngineInput::Z | EngineInput::ANY_TRIGGER).into();

    current.intersects(mask) && !previous.intersects(mask)
}

pub fn is_in_hitlag(flags: u64) -> bool {
    Flags::from(flags).contains(Flags::HITLAG.into())
}



pub fn is_fastfalling(flags: u64) -> bool {
    Flags::from(flags).contains(Flags::FASTFALL.into())
}

/// Returns true if the player is in any tumble or reeling animation, or if they are in the jab reset animation
pub fn is_damaged(state: u16) -> bool {
    (ActionRange::DAMAGE_START as u16 <= state && state <= ActionRange::DAMAGE_END as u16)
        || state == ActionState::DOWN_DAMAGE_D as u16
        || state == ActionState::DOWN_DAMAGE_U as u16
}
