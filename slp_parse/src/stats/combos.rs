use std::{
    collections::HashSet,
    ops::{Deref, Range, RangeInclusive},
    path::PathBuf,
    sync::Arc,
};

use derive_new::new;
use serde_json::json;
use ssbm_utils::{
    checks::{is_grabbed, just_lost_stock},
    enums::{stage::Stage, Attack, Character, StageID},
    mf,
    prelude::*,
    types::Position,
};

use crate::{frames::Frames, Game, GameMetadata};

pub const COMBO_LENIENCY: u32 = 45;
pub const PRE_COMBO_BUFFER_FRAMES: i32 = 75;
pub const POST_COMBO_BUFFER_FRAMES: i32 = 120;

#[derive(Debug, Clone, new)]
pub struct Move {
    pub frame_index: i32,
    pub move_id: Attack,
    #[new(value = "1")]
    pub hit_count: u32,
    pub damage: f32,
    pub opponent_position: Position,
    pub player_position: Position,
    pub player_orientation: Orientation,
}

#[derive(Debug, Clone, new)]
pub struct Combo {
    pub path: Arc<PathBuf>,
    #[new(default)]
    pub move_list: Vec<Move>,
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
    pub fn to_queue_obj(&self) -> serde_json::Value {
        json!({
            "path": self.path.to_str(),
            "startFrame": self.start_frame - PRE_COMBO_BUFFER_FRAMES,
            "endFrame": self.end_frame + POST_COMBO_BUFFER_FRAMES,
        })
    }

    pub fn is_game_ender(&self) -> bool {
        self.did_kill && self.opponent_stocks == 1
    }

    pub fn filter_out_attack(&self, attacks: Vec<Attack>) -> impl Iterator<Item = &Move> {
        self.move_list
            .iter()
            .filter(move |m| attacks.contains(&m.move_id))
    }

    pub fn damage(&self) -> f32 {
        self.end_percent - self.start_percent
    }

    pub fn duration(&self) -> i32 {
        self.end_frame - self.start_frame
    }

    /// Returns a 0-indexed range object useful for iterating over the frames during the combo
    pub fn frame_range(&self) -> RangeInclusive<usize> {
        (self.start_frame + 123) as usize..=(self.end_frame + 123) as usize
    }

    /// Returns a -123 indexed range object useful for checking the timing of other events
    pub fn melee_frame_range(&self) -> RangeInclusive<i32> {
        self.start_frame..=self.end_frame
    }
}

#[derive(Debug, Clone, Default)]
pub struct Combos {
    pub data: Vec<Combo>,
    pub path: Arc<PathBuf>,
}

impl Combos {
    /// Creates a new combo object containing only combos whose starting percent are below a given
    /// value
    pub fn filter_max_start_percent(&self, value: f32) -> impl Iterator<Item = &Combo> {
        self.iter().filter(move |c| c.start_percent <= value)
    }

    pub fn filter_hit_count(&self, value: Range<usize>) -> Combos {
        Combos {
            data: self
                .iter()
                .filter_map(|c| (value.contains(&c.move_list.len())).then_some(c.clone()))
                .collect(),
            path: self.path.clone(),
        }
    }

    pub fn filter_min_duration(&self, value: Range<isize>) -> impl Iterator<Item = &Combo> {
        self.iter()
            .filter(move |c| value.contains(&((c.end_frame - c.start_frame).abs() as isize)))
    }
}

// Deref abuse is sick and nobody can tell me otherwise
impl Deref for Combos {
    type Target = Vec<Combo>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

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
    _player_char: Character,
    path: Arc<PathBuf>,
) -> Combos {
    let mut result = Vec::new();

    let mut event = None;
    let mut combo_state = ComboState::default();
    let stage = Stage::from_id(stage_id);

    for i in 1..plyr_frames.len() {
        let plyr_state = plyr_frames.post.action_state[i];
        let plyr_position = plyr_frames.post.position[i];

        // let opnt_state = opnt_frames.post.action_state[i];
        // let prev_opnt_state = opnt_frames.post.action_state[i - 1];

        let opnt_position = opnt_frames.post.position[i];
        let opnt_is_damaged = opnt_frames.damaged_state(i);
        let opnt_is_in_hitstun = opnt_frames.in_hitstun(i);
        let opnt_is_grabbed = opnt_frames.grabbed(i) || opnt_frames.cmd_grabbed(i);
        let opnt_damage_taken = opnt_frames.damage_taken(i);
        let opnt_prev_percent = opnt_frames.post.percent[i - 1];
        // let opnt_damage_taken = get_damage_taken(opnt_percent, opnt_prev_percent);

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
                    path.clone(),
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
                    event.as_mut().unwrap().move_list.push(Move::new(
                        i as i32 - 123,
                        Attack::from_repr(plyr_frames.post.last_attack_landed[i]).unwrap(),
                        opnt_damage_taken,
                        opnt_position,
                        plyr_position,
                        Orientation::from_repr(plyr_frames.post.orientation[i] as i8).unwrap(),
                    ));
                } else {
                    let temp = event
                        .as_mut()
                        .unwrap()
                        .move_list
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
        let opnt_is_in_hitlag = opnt_frames.in_hitlag(i);
        let opnt_is_teching = opnt_frames.teching(i);
        let opnt_is_downed = opnt_frames.downed(i);
        let opnt_is_dying = opnt_frames.dying(i);
        let opnt_is_offstage = stage.is_offstage(opnt_position);
        let opnt_is_dodging = opnt_frames.dodging(i);
        let opnt_is_shielding = opnt_frames.shielding(i);
        let opnt_shield_broken = opnt_frames.shield_broken(i);
        let opnt_is_ledge_action = opnt_frames.ledge_action(i);
        let opnt_is_special_fall = opnt_frames.special_fall(i);
        let opnt_is_upb_lag = opnt_frames.upb_lag(i);

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
            just_lost_stock(plyr_frames.post.stocks[i], plyr_frames.post.stocks[i - 1]);

        let mut should_terminate =
            combo_state.reset_counter == 0 || plyr_is_grabbed || plyr_lost_stock;

        if just_lost_stock(opnt_frames.post.stocks[i], opnt_frames.post.stocks[i - 1]) {
            should_terminate = true;
            event.as_mut().unwrap().did_kill = true;
        }

        if should_terminate {
            let temp = event.as_mut().unwrap();
            temp.end_frame = i as i32 - 123;
            temp.end_percent = opnt_prev_percent;
            temp.end_position = opnt_frames.post.position[i - 1];

            if !temp.move_list.is_empty() {
                result.push(event.unwrap());
            }

            event = None;
        }
    }

    Combos { data: result, path }
}

pub fn rate_falco_combos(connect_code: &str, games: &[Game]) -> Vec<(Combo, i32)> {
    let mut result = Vec::new();

    for game in games {
        let player = game.player_by_code(connect_code);

        if player.is_err() {
            continue;
        }

        let player = player.unwrap();
        if player.character != Character::Falco {
            continue;
        }

        let stage = Stage::from_id(game.stage());
        let opnt = game.opponent_by_code(connect_code).unwrap();

        for combo in &player.combos.data {
            let mut rating: i32 = 0;

            if !combo.did_kill || combo.move_list.len() < 3 {
                rating -= 100;
            } else if combo.start_percent == 0.0 {
                // 0-death bonus
                rating += 10;
            }

            // really really long combos are rarely worth watching
            if combo.end_frame - combo.start_frame >= 900 {
                rating -= 100;
            }

            if combo.is_game_ender() {
                rating += 10;
            }

            // incentivize longer combos, but cap at 60. Longer combos tend to be meandering and
            // not very interesting.
            rating += (combo.move_list.len() * 10).min(70) as i32;

            // incentivize combos where the opponent is in hitstun more of the time - balances out
            // the above by allowing shorter combos to get extra points
            let hitstun_frames = opnt.frames.post.flags.as_ref().unwrap()
                [mf!(combo.start_frame)..mf!(combo.end_frame)]
                .iter()
                .fold(0, |acc, x| {
                    if (Flags::HITSTUN).contained_by(*x) {
                        acc + 1
                    } else {
                        acc
                    }
                });

            rating += ((hitstun_frames as f32 / combo.duration() as f32) * 50.0) as i32;

            // deduct points if the player gets hit
            for def in &player.stats.defense.as_ref().unwrap().frame_index {
                if (combo.start_frame..combo.end_frame).contains(def) {
                    rating -= 5;
                }
            }

            // deduct points if the first hit is offstage - this targets certain long edgeguard
            // strings that aren't particularly interesting
            if stage.is_offstage(combo.move_list[0].opponent_position) {
                rating -= 10;
            }

            {
                // deduct points for missing moves. Does not count B moves, as those can be used for
                // utility purposes
                let mut attack = false;
                let mut hit = false;
                let flags = player.frames.post.flags.as_deref().unwrap();

                for i in combo.frame_range() {
                    let prev = player.frames.post.action_state[i - 1];
                    let state = player.frames.post.action_state[i];

                    if ActionState::GROUND_ATTACK_RANGE.contains(&state) {
                        if state != prev {
                            attack = true;
                            hit = false;
                        }
                        if Flags::HITLAG.contained_by(flags[i])
                            && !Flags::DEFENDER_HITLAG.contained_by(flags[i])
                        {
                            hit = true;
                        }
                    } else {
                        if attack && !hit {
                            rating -= 5;
                        }
                        attack = false;
                        hit = false;
                    }
                }
            }

            let mut moveset = HashSet::new();

            for mv in &combo.move_list {
                moveset.insert(mv.move_id);

                // non-standard moves are cool
                if is_unusual(mv) {
                    rating += 2;
                }
                // lasers are boring
                if mv.move_id == Attack::NEUTRAL_SPECIAL {
                    rating -= 2;
                }

                // jab resets
                if matches!(
                    ActionState::from_repr(opnt.frames.post.action_state[mf!(mv.frame_index)])
                        .unwrap_or_default(),
                    ActionState::DOWN_DAMAGE_U | ActionState::DOWN_DAMAGE_D
                ) {
                    rating += 4;
                }

                // reverse hits
                // neat trick: orientation left is negative, right is positive. Normalizing the
                // opponent's position to the player will make it negative if they are farther left
                // and positive if they are farther right. If the player is facing the same direction
                // as their opponent is positioned relative to them, the result will be positive
                // (right/pos * right/pos or left/neg * left/neg).
                let relative_pos = mv.player_orientation as i8 as f32
                    * (mv.opponent_position.x - mv.player_position.x);
                // make sure it's a move that can actually be meaningfully reversed
                // also bair is a special case in that a reverse hit means you're facing your opnt
                if !matches!(mv.move_id, Attack::NEUTRAL_SPECIAL | Attack::DOWN_SPECIAL)
                    && relative_pos.is_sign_negative()
                    && mv.move_id != Attack::BAIR
                    || relative_pos.is_sign_positive() && mv.move_id == Attack::BAIR
                {
                    rating += 2;
                }
            }

            // tech reads are cool
            if let Some(techs) = opnt.stats.tech.as_ref() {
                for i in 0..techs.frame_index.len() {
                    if !(combo.start_frame..combo.end_frame).contains(&techs.frame_index[i]) {
                        continue;
                    }

                    if techs.punished[i] {
                        rating += 5;
                    }
                }
            }

            // offstage edgeguards are cool
            if stage.is_offstage(combo.move_list.last().unwrap().player_position)
                && stage.is_offstage(combo.move_list.last().unwrap().opponent_position)
            {
                rating += 5;
            }

            rating += 2 * moveset.len() as i32;

            result.push((combo.clone(), rating));
        }
    }

    result
}

fn is_unusual(mv: &Move) -> bool {
    matches!(
        mv.move_id,
        Attack::F_TILT
            | Attack::D_TILT
            | Attack::FAIR
            | Attack::UAIR
            | Attack::SIDE_SPECIAL
            | Attack::FORWARD_THROW
            | Attack::BACK_THROW
            | Attack::UP_THROW
            | Attack::DOWN_THROW
    )
}
