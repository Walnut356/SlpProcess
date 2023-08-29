use itertools::Itertools;

use crate::enums::general::*;
use crate::enums::stage::Stage;
use crate::stats::actions::find_actions;
use crate::utils::BitFlags;
use crate::{
    enums::{buttons::*, general::*, state::*},
    Game,
};

use super::lcancel::find_lcancels;

pub fn get_stats(game: &mut Game) {
    for players in game.players.iter().permutations(2) {
        let mut player = players[0].as_ref().write().unwrap();
        let opponent = players[1].as_ref().read().unwrap();

        player.stats.l_cancel = find_lcancels(&player.frames, Stage::from_id(game.start.stage));
        player.stats.actions = find_actions(&player.frames, game.total_frames);
    }
}

pub fn get_actionstate() {

}

pub fn just_input_lcancel(frames: &[Option<u32>], i: usize) -> bool {
    let current = EngineInput::from(frames[i].unwrap());
    let previous = EngineInput::from(frames[i.saturating_sub(1)].unwrap());

    let mask:u32 = (EngineInput::Z | EngineInput::ANY_TRIGGER).into();

    current.intersects(mask) && !previous.intersects(mask)
}

/// Requires the Flags2 bitfield
pub fn is_in_hitlag(flags2: u8) -> bool {
    Flags2::from(flags2).contains(Flags2::HITLAG.into())
}

/// Requires the Flags2 bitfield
pub fn is_fastfalling(flags2: u8) -> bool {
    Flags2::from(flags2).contains(Flags2::FASTFALL.into())
}
