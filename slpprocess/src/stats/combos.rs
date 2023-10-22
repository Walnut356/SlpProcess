use derive_new::new;
use polars::prelude::*;
use ssbm_utils::{
    checks::{
        get_damage_taken, is_cmd_grabbed, is_damaged, is_dodging, is_downed, is_dying, is_grabbed,
        is_in_hitlag, is_in_hitstun, is_ledge_action, is_shield_broken, is_shielding,
        is_special_fall, is_teching, is_upb_lag, lost_stock,
    },
    enums::{stage::Stage, Attack, Character, StageID},
    types::Position,
};

use crate::player::Frames;

pub const COMBO_LENIENCY: u32 = 45;
pub const PRE_COMBO_BUFFER_FRAMES: usize = 60;
pub const POST_COMBO_BUFFER_FRAMES: usize = 90;

#[derive(Debug, Clone, new)]
pub struct Move {
    pub frame_index: i32,
    pub move_id: Attack,
    #[new(value = "1")]
    pub hit_count: u32,
    pub damage: f32,
    pub opponent_position: Position,
    pub player_position: Position,
}

#[derive(Debug, Clone, new)]
pub struct Combo {
    #[new(default)]
    pub movelist: Vec<Move>,
    #[new(value = "false")]
    pub did_kill: bool,
    pub start_position: Position,
    #[new(value = "Position::new(0.0, 0.0)")]
    pub end_position: Position,
    pub player_stocks: u8,
    pub opponent_stocks: u8,

    pub start_percent: f32,
    #[new(value = "-1.0")]
    pub end_percent: f32,
    pub start_frame: i32,
    #[new(value = "-1")]
    pub end_frame: i32,
}

impl Combo {
    pub fn is_game_ender(&self) -> bool {
        self.did_kill && self.opponent_stocks == 1
    }
}

#[derive(Debug, Clone, Default)]
pub struct Combos(pub Vec<Combo>);

#[derive(Debug, Clone)]
pub struct ComboState {
    reset_counter: u32,
    last_hit_animation: Option<u16>,
}

impl Default for ComboState {
    fn default() -> Self {
        Self {
            reset_counter: COMBO_LENIENCY,
            last_hit_animation: None,
        }
    }
}

pub fn find_combos(
    plyr_frames: &Frames,
    opnt_frames: &Frames,
    stage_id: StageID,
    player_char: Character,
) -> Combos {
    let mut result = Vec::new();

    let mut event = None;
    let mut combo_state = ComboState::default();
    let stage = Stage::from_id(stage_id);

    for i in 1..plyr_frames.len() {
        let plyr_state = plyr_frames.post.action_state[i];
        let prev_plyr_state = plyr_frames.post.action_state[i - 1];
        let plyr_position = plyr_frames.post.position[i];

        let opnt_state = opnt_frames.post.action_state[i];
        let prev_opnt_state = opnt_frames.post.action_state[i - 1];

        let opnt_position = opnt_frames.post.position[i];
        let opnt_is_damaged = is_damaged(opnt_state);
        let opnt_flags = opnt_frames.post.flags.as_ref().unwrap()[i];
        let opnt_is_in_hitstun = is_in_hitstun(opnt_flags);
        let opnt_is_grabbed = is_grabbed(opnt_state) || is_cmd_grabbed(opnt_state);
        let opnt_percent = opnt_frames.post.percent[i];
        let opnt_prev_percent = opnt_frames.post.percent[i - 1];
        let opnt_damage_taken = get_damage_taken(opnt_percent, opnt_prev_percent);

        /* "Keep track of whether actionState changes after a hit. Used to compute move count
        When purely using action state there was a bug where if you did two of the same
        move really fast (such as ganon's jab), it would count as one move. Added
        the actionStateCounter at this point which counts the number of frames since
        an animation started. Should be more robust, for old files it should always be
        null and null < null = false" - official parser */
        let action_changed_since_hit = combo_state
            .last_hit_animation
            .is_some_and(|x| x == plyr_state);

        let action_state_reset = {
            let state_age = plyr_frames.post.state_frame.as_ref().unwrap();
            let action_frame_counter = state_age[i];
            let prev_action_counter = state_age[i - 1];
            action_frame_counter < prev_action_counter
        };

        if action_changed_since_hit || action_state_reset {
            combo_state.last_hit_animation = None
        }

        if opnt_is_in_hitstun || opnt_is_grabbed || opnt_is_damaged {
            // if the opponent has been hit and there's no active combo, start a combo
            if event.is_none() {
                event = Some(Combo::new(
                    opnt_position,
                    plyr_frames.post.stocks[i],
                    opnt_frames.post.stocks[i],
                    opnt_prev_percent,
                    i as i32 - 123,
                ));
            }

            /* if the opponent has taken damage and we're sure it's not the same move, record the
            move's data */

            // TODO BUG slippi-js has issues with this too, but magnifying glass damage while in
            // a knockback animation will count as a move
            if opnt_damage_taken > 0.0 {
                if combo_state.last_hit_animation.is_none() {
                    event.as_mut().unwrap().movelist.push(Move::new(
                        i as i32 - 123,
                        Attack::from(plyr_frames.post.last_attack_landed[i]),
                        opnt_damage_taken,
                        opnt_position,
                        plyr_position,
                    ));
                } else {
                    let temp = event
                        .as_mut()
                        .unwrap()
                        .movelist
                        .last_mut()
                        .expect("No move despite last hit animation and damage taken");

                    temp.hit_count += 1;
                    temp.damage += opnt_damage_taken;
                }
            }
        }

        // Avoid processing the combo extend criteria if there is no active combo
        if event.is_none() {
            continue;
        }

        // Now we check all relevant conditions and see if we should keep the combo going or end it
        let opnt_is_in_hitlag = is_in_hitlag(opnt_flags);
        let opnt_is_teching = is_teching(opnt_state);
        let opnt_is_downed = is_downed(opnt_state);
        let opnt_is_dying = is_dying(opnt_state);
        let opnt_is_offstage = stage.is_offstage(opnt_position);
        let opnt_is_dodging = is_dodging(opnt_state);
        let opnt_is_shielding = is_shielding(opnt_state);
        let opnt_shield_broken = is_shield_broken(opnt_state);
        let opnt_is_ledge_action = is_ledge_action(opnt_state);
        let opnt_is_special_fall = is_special_fall(opnt_state);
        let opnt_is_upb_lag = is_upb_lag(opnt_state, prev_opnt_state);

        if opnt_is_damaged
            || opnt_is_grabbed
            || opnt_is_in_hitlag
            || opnt_is_in_hitstun
            || opnt_is_offstage
            || opnt_is_dodging
            || opnt_is_shielding
            || opnt_shield_broken
            || opnt_is_ledge_action
            || opnt_is_special_fall
            || opnt_is_upb_lag
            || opnt_is_teching
            || opnt_is_downed
            || opnt_is_dying
        {
            combo_state.reset_counter = COMBO_LENIENCY;
        } else {
            combo_state.reset_counter -= 1;
        }

        let plyr_is_grabbed = is_grabbed(plyr_state);
        let plyr_lost_stock =
            lost_stock(plyr_frames.post.stocks[i], plyr_frames.post.stocks[i - 1]);

        let mut should_terminate =
            combo_state.reset_counter == 0 || plyr_is_grabbed || plyr_lost_stock;

        if lost_stock(opnt_frames.post.stocks[i], opnt_frames.post.stocks[i - 1]) {
            should_terminate = true;
            event.as_mut().unwrap().did_kill = true;
        }

        if should_terminate {
            let temp = event.as_mut().unwrap();
            temp.end_frame = i as i32 - 123;
            temp.end_percent = opnt_prev_percent;
            temp.end_position = opnt_frames.post.position[i - 1];

            result.push(event.unwrap());

            event = None;
        }
    }

    Combos(result)
}
