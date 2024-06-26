#![allow(clippy::too_many_arguments)]

use ssbm_utils::{
    calc::{
        apply_di, get_di_efficacy, initial_x_velocity, initial_y_velocity, kb_from_initial,
        should_kill,
    },
    checks::{
        get_damage_taken, is_electric_attack, is_in_hitlag, is_thrown, is_vcancel_state,
        just_pressed_any, just_took_damage,
    },
    constants::ASDI_DIST,
    enums::{Attack, Character, EngineInput, State, StickRegion},
    types::{Degrees, Position, StickPos, Velocity},
};

use crate::{frames::Frames, stats::Stat};

pub(crate) fn find_defense(
    plyr_frames: &Frames,
    opnt_frames: &Frames,
    stage_id: u16,
    player_char: Character,
    opnt_char: Character,
) -> DefenseStats {
    let pre = &plyr_frames.pre;
    let post = &plyr_frames.post;
    let attacks = &opnt_frames.post.last_attack_landed;

    let flags: &[u64] = post.flags.as_ref().unwrap();
    let states: &[u16] = post.action_state.as_ref();
    let grounded: &[bool] = post.is_grounded.as_ref().unwrap();

    let mut event = None;
    let mut stat_table = DefenseStats::default();

    // value tracking for v cancel
    let mut l_lockout: i32 = 0;
    let mut most_recent_l = 0;

    // start 1 frame "late" to prevent index errors
    for i in 1..pre.frame_index.len() {
        // check for grab states

        if just_pressed_any(
            EngineInput::R | EngineInput::L,
            pre.engine_buttons[i],
            pre.engine_buttons[i - 1],
        ) {
            l_lockout = 40;
            most_recent_l = i;
        }
        l_lockout -= 1;

        // just_in_hitlag, filtering out shield hits
        let in_hitlag = is_in_hitlag(flags[i]);
        let was_in_hitlag = is_in_hitlag(flags[i - 1]);

        // let shielding = is_shielding_flag(flags[i]);
        // let grabbed_check = false;

        let took_damage = just_took_damage(post.percent[i], post.percent[i - 1]);
        let damage_taken = get_damage_taken(post.percent[i], post.percent[i - 1]);

        // ----------------------------------- event detection ---------------------------------- //
        // TODO check for being hit while already in hitlag
        if event.is_none()
            && ((!was_in_hitlag && took_damage)
                || (!in_hitlag && took_damage && is_thrown(states[i])))
        // && !is_magnifying_damage(damage_taken, flags, i)
        {
            let prev_state = states[i - 1];

            event = Some(DefenseRow::new(
                i as i32 - 123,
                post.stocks[i],
                post.percent[i],
                damage_taken,
                Attack::from_repr(attacks[i]).unwrap(),
                State::from_state_and_char(prev_state, Some(player_char)),
                grounded[i - 1],
                post.position[i],
            ));

            let row = event.as_mut().unwrap();
            row.kb = post.knockback.as_ref().unwrap()[i];

            if row.grounded || !is_vcancel_state(prev_state) {
                row.v_cancel = None;
            } else if (1..3).contains(&(i - most_recent_l)) // must have hit l a max of 2 frames before the hit
                && l_lockout.is_negative()
            // must not be in L lockout
            {
                println!("Woah a vcancel!");
                row.v_cancel = Some(true);
            } else {
                row.v_cancel = Some(false);
            }
        }

        // ----------------------------------- mid-event data ----------------------------------- //
        if event.is_some() && in_hitlag {
            let row = event.as_mut().unwrap();
            row.hitlag_frames += 1;
            let curr_stick = pre.joystick[i].as_stickregion();
            row.stick_during_hitlag.push(curr_stick);

            if row.hitlag_frames > 1 && curr_stick.valid_sdi(pre.joystick[i - 1].as_stickregion()) {
                row.sdi_inputs.push(curr_stick)
            }

            continue;
        }

        // ----------------------------------- finalize event ----------------------------------- //

        if !in_hitlag && was_in_hitlag && event.is_some() {
            let row = event.as_mut().unwrap();

            // check for crouch cancel. I could use action states, but there's complications with
            // if you just entered crouch, or if you crouched during a subframe event, so we're just
            // gonna check against the expected hitlag frames.

            let expected_hitlag = ssbm_utils::calc::on_hit::hitlag(
                row.damage_taken,
                is_electric_attack(row.last_hit_by, &opnt_char),
                true,
            );
            if row.grounded {
                if row.hitlag_frames as u32 == expected_hitlag {
                    row.crouch_cancel = Some(true);
                } else {
                    row.crouch_cancel = Some(false);
                }
            } else {
                row.crouch_cancel = None;
            }

            let effective_stick = pre.joystick[i];

            row.di_stick = effective_stick;

            let mut asdi_dist = Velocity::default();

            let cstick = pre.cstick[i];
            row.asdi = if !cstick.as_stickregion().is_deadzone() {
                asdi_dist.x = cstick.x * ASDI_DIST;
                asdi_dist.y = cstick.y * ASDI_DIST;
                cstick.as_stickregion()
            } else {
                asdi_dist.x = effective_stick.x * ASDI_DIST;
                asdi_dist.y = effective_stick.y * ASDI_DIST;
                effective_stick.as_stickregion()
            };

            // let kb = post.knockback.as_ref().unwrap()[i];
            // let with_decay = kb - Velocity::new(KB_DECAY * kb.x, KB_DECAY * kb.y);

            // You can't SDI on the last frame of hitlag anyway, so the position on that last frame
            // + the ASDI distance will equal your starting position before knockback takes effect
            row.hitlag_end = post.position[i - 1] + asdi_dist;

            let kb_angle_rads = row.kb.as_angle();
            row.kb_angle = kb_angle_rads.to_degrees();

            if !row.kb.is_zero() {
                let with_di = apply_di(kb_angle_rads, effective_stick);

                row.di_efficacy = Some(get_di_efficacy(kb_angle_rads, with_di));
                row.di_kb_angle = with_di.to_degrees();

                let kb_scalar = kb_from_initial(row.kb);

                row.di_kb = Velocity::new(
                    initial_x_velocity(kb_scalar, with_di),
                    initial_y_velocity(kb_scalar, with_di, row.grounded),
                );

                let char_stats = player_char.get_stats();

                row.kills_no_di = should_kill(
                    stage_id,
                    row.kb,
                    row.hitlag_end,
                    char_stats.gravity,
                    char_stats.max_fall_speed,
                );

                if effective_stick.x != 0.0 || effective_stick.y != 0.0 {
                    row.kills_with_di = should_kill(
                        stage_id,
                        row.di_kb,
                        row.hitlag_end,
                        char_stats.gravity,
                        char_stats.max_fall_speed,
                    );
                } else {
                    row.kills_with_di = row.kills_no_di;
                }

                row.kills_any_di = {
                    let mut result = true;
                    for j in -90..=90 {
                        let new_traj = kb_angle_rads - (j as f32 / 5.0).to_radians();
                        if !should_kill(
                            stage_id,
                            Velocity::new(
                                initial_x_velocity(kb_scalar, new_traj),
                                initial_y_velocity(kb_scalar, new_traj, row.grounded),
                            ),
                            row.hitlag_end,
                            char_stats.gravity,
                            char_stats.max_fall_speed,
                        ) {
                            result = false;
                        } else {
                            row.kills_some_di = true;
                        }
                    }

                    result
                }
            } else {
                // No reason to calculate when there's no knockback. Handles things like fox laser
                row.di_efficacy = None;
                row.di_kb = row.kb;
                row.di_kb_angle = row.kb_angle;
                row.kills_no_di = false;
                row.kills_with_di = false;
                row.kills_any_di = false;
            }

            stat_table.append(event.as_ref().unwrap());
            event = None;
        }
    }

    stat_table
}

#[derive(Debug, Default, Clone)]
pub struct DefenseStats {
    pub frame_index: Vec<i32>,
    pub stocks_remaining: Vec<u8>,
    pub percent: Vec<f32>,
    pub damage_taken: Vec<f32>,
    pub last_hit_by: Vec<Attack>,
    pub state_before_hit: Vec<State>,
    pub grounded: Vec<bool>,
    pub crouch_cancel: Vec<Option<bool>>,
    pub hitlag_frames: Vec<u8>,
    pub stick_during_hitlag: Vec<Vec<StickRegion>>,
    pub sdi_inputs: Vec<Vec<StickRegion>>,
    pub asdi: Vec<StickRegion>,
    pub kb: Vec<Velocity>,
    pub kb_angle: Vec<Degrees>,
    pub di_stick: Vec<StickPos>,
    pub di_kb: Vec<Velocity>,
    pub di_efficacy: Vec<Option<f32>>,
    pub di_kb_angle: Vec<Degrees>,
    pub hitlag_start: Vec<Position>,
    pub hitlag_end: Vec<Position>,
    pub kills_with_di: Vec<bool>,
    pub kills_no_di: Vec<bool>,
    pub kills_any_di: Vec<bool>,
    pub kills_some_di: Vec<bool>,
    pub v_cancel: Vec<Option<bool>>,
    // TODO shieldpoke: Vec<Option<bool>>,
    // TODO ground_id: Vec<Option<GroundID>>,
}

impl DefenseStats {
    fn append(&mut self, stat: &DefenseRow) {
        self.frame_index.push(stat.frame_index);
        self.stocks_remaining.push(stat.stocks_remaining);
        self.percent.push(stat.percent);
        self.damage_taken.push(stat.damage_taken);
        self.last_hit_by.push(stat.last_hit_by);
        self.state_before_hit.push(stat.state_before_hit);
        self.grounded.push(stat.grounded);
        self.crouch_cancel.push(stat.crouch_cancel);
        self.hitlag_frames.push(stat.hitlag_frames);
        self.stick_during_hitlag
            .push(stat.stick_during_hitlag.clone());
        self.sdi_inputs.push(stat.sdi_inputs.clone());
        self.asdi.push(stat.asdi);
        self.kb.push(stat.kb);
        self.kb_angle.push(stat.kb_angle);
        self.di_stick.push(stat.di_stick);
        self.di_kb.push(stat.di_kb);
        self.di_efficacy.push(stat.di_efficacy);
        self.di_kb_angle.push(stat.di_kb_angle);
        self.hitlag_start.push(stat.hitlag_start);
        self.hitlag_end.push(stat.hitlag_end);
        self.kills_with_di.push(stat.kills_with_di);
        self.kills_no_di.push(stat.kills_no_di);
        self.kills_any_di.push(stat.kills_any_di);
        self.kills_some_di.push(stat.kills_some_di);
        self.v_cancel.push(stat.v_cancel);
    }
}

impl Stat for DefenseStats {}

#[derive(Debug, Default, Clone)]
pub struct DefenseRow {
    pub frame_index: i32,
    pub stocks_remaining: u8,
    pub percent: f32,
    pub damage_taken: f32,
    pub last_hit_by: Attack,
    pub state_before_hit: State,
    pub grounded: bool,
    pub crouch_cancel: Option<bool>,
    pub hitlag_frames: u8,
    pub stick_during_hitlag: Vec<StickRegion>,
    pub sdi_inputs: Vec<StickRegion>,
    pub asdi: StickRegion,
    pub kb: Velocity,
    pub kb_angle: Degrees,
    pub di_stick: StickPos,
    pub di_kb: Velocity,
    pub di_kb_angle: Degrees,
    pub di_efficacy: Option<f32>,
    pub hitlag_start: Position,
    pub hitlag_end: Position,
    pub kills_with_di: bool,
    pub kills_no_di: bool,
    pub kills_any_di: bool,
    pub kills_some_di: bool,
    pub v_cancel: Option<bool>,
}

impl DefenseRow {
    pub fn new(
        frame_index: i32,
        stocks_remaining: u8,
        percent: f32,
        damage_taken: f32,
        last_hit_by: Attack,
        state_before_hit: State,
        grounded: bool,
        start: Position,
    ) -> Self {
        Self {
            frame_index,
            stocks_remaining,
            percent,
            damage_taken,
            last_hit_by,
            state_before_hit,
            grounded,
            hitlag_start: start,
            ..Default::default()
        }
    }
}
