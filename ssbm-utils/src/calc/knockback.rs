#![allow(clippy::too_many_arguments)]

use std::{cmp, f32::consts::{PI, TAU}};

use approx::assert_relative_eq;

use crate::{
    calc::attack::hitstun,
    constants::{KB_DECAY, TUMBLE_THRESHOLD, Z_ANALOG},
    enums::{character::*, stage::*},
    types::{Velocity, Position, Degrees, StickPos, Radians},
};

/// Calculates the raw knockback value given the circumstances of the hit.
///
/// *note: assumes Victim Defense Ratio, Attacker Offense Ratio, and Global Damage Ratio are 1, as
/// they are in all tournament matches.
pub fn knockback(
    damage_staled: f32,
    damage_unstaled: f32,
    kb_growth: u32,
    base_kb: u32,
    set_kb: u32,
    is_throw: bool,
    character: &Attributes,
    percent: f32,
    crouch_cancel: bool,
    charge_interrupt: bool,
    vcancel: bool,
    dj_armor: bool,
    metal: bool,
    ice: bool,
) -> f32 {
    let weight: u32 = match is_throw {
        true => 100,
        false => character.weight,
    };

    let mut kb: f32;

    if set_kb == 0 {
        kb = (0.01 * kb_growth as f32)
            * ((1.4
                * (((0.05 * (damage_unstaled * (damage_staled + percent.floor())))
                    + (damage_staled + percent.floor()) * 0.1)
                    * (2.0 - (2.0 * (weight as f32 * 0.01)) / (1.0 + (weight as f32 * 0.01)))))
                + 18.0)
            + base_kb as f32;
    } else {
        kb = ((((set_kb * 10 / 20) + 1) as f32 * 1.4 * (200 / (weight + 100)) as f32 + 18.0)
            * (kb_growth / 100) as f32)
            + base_kb as f32;
    }

    if crouch_cancel {
        kb *= 0.667;
    }
    if charge_interrupt {
        kb *= 1.2;
    }
    if vcancel {
        kb *= 0.95;
    }
    if ice {
        kb *= 0.25;
    }
    if dj_armor {
        kb = f32::max(0.0, kb - 120.0);
    }
    if metal {
        kb = f32::max(0.0, kb - 30.0);
    }
    if character.name == "Nana" {
        kb = f32::max(0.0, kb - 5.0);
    }
    kb = f32::min(2500.0, kb);

    kb
}

/// Checks if a given trajectory is a sakurai angle and returns a modified trajectory with the
/// proper sakurai-angle logic applied.
///
/// If the victim is airborne, the angle is 45 degrees.
///
/// If the victim is grounded and the knockback is less than or equal to 32.0, the angle is
/// horizontal. If the victim is grounded and the knockback is greater than or equal to 32.1, the
/// knockback is 44 degrees. If the knockback is **between** the thresholds, it is scaled linearly
/// between 0 and 44 degrees. The gap between the thresholds is small enough that it's rare that
/// this situation comes up.
///
/// e.g. Marth sourspot jab vs falco at 9%, with a knockback value of 32.03333, thus a final
/// knockback angle of ~14.667 degrees
pub fn resolve_sakurai_angle(angle: Radians, knockback: f32, grounded: bool) -> Radians {
    if angle != 361.0_f32.to_radians() {
        return angle;
    }

    if !grounded {
        return 45.0_f32.to_radians();
    }

    let kb_extreme = knockback <= 32.0 || knockback >= 32.1;

    if kb_extreme {
        if knockback <= 32.0 {
            return 0.0;
        } else {
            return 44.0_f32.to_radians();
        }
    }

    // at this point we can guarantee that 32.0 < knockback < 32.1
    // the following just treats 32 as 0 and 32.1 as 1, which gives us a percentage to scale by
    let scalar = (knockback - 32.0) * 10.0;

    44.0 * scalar
}

/// Returns a new knockback trajectory by modifying the given trajectory to account for DI based on
/// a joystick X and Y value. This should be done after dealing with trajectory modifiers such as
/// sakurai angle.
pub fn apply_di(original_angle: Radians, joystick: StickPos) -> Radians {
    let mut angle_diff = original_angle - joystick.as_angle();

    if angle_diff > PI {
        angle_diff -= TAU;
    }

    let perp_dist = angle_diff.sin() * f32::hypot(joystick.x, joystick.y);
    let mut angle_offset = (perp_dist.powi(2)) * 18.0;

    if angle_offset > 18.0 {
        angle_offset = 18.0
    }
    if -PI < angle_diff && angle_diff < 0.0 {
        angle_offset *= -1.0;
    }

    original_angle - angle_offset
}

/// Returns a percentage representing how much the DI affected the final trajectory, relative to the
/// maximum possible effect that DI can have.
pub fn get_di_efficacy(old_angle: Radians, new_angle: Radians) -> f32 {
    (new_angle - old_angle).abs() / 18.0_f32.to_radians()
}

/// Converts a knockback value and angle into the initial X knockback velocity imparted on the
/// character.
pub fn initial_x_velocity(knockback: f32, angle: Radians) -> f32 {
    let magnitude = knockback * 0.03;
    let angle = angle.cos();
    angle * magnitude
}

/// Converts a knockback value and angle into the initial Y knockback velocity imparted on the
/// character.
pub fn initial_y_velocity(knockback: f32, angle: Radians, grounded: bool) -> f32 {
    let high_kb = knockback >= 80.0;

    if high_kb && grounded && (angle == 0.0 || angle == 180.0_f32.to_radians()) {
        return 0.0;
    }

    let magnitude = knockback * 0.03;
    let angle = angle.sin();
    let mut velocity = angle * magnitude;

    let down = angle > 180.0_f32.to_radians() && angle < 361.0_f32.to_radians();

    if down && high_kb {
        velocity *= 0.8;
    }

    velocity
}

/// Rate at which horizontal knockback velocity decreases per frame
pub fn get_horizontal_decay(angle: Radians) -> f32 {
    KB_DECAY * angle.cos()
}

/// Rate at which vertical knockback velocity decreases per frame
/// Gravity also plays a role, but that is done in knockback_travel
pub fn get_vertical_decay(angle: Radians) -> f32 {
    KB_DECAY * angle.sin()
}

pub fn will_tumble(kb: f32) -> bool {
    kb > TUMBLE_THRESHOLD
}

/// Accepts the initial knockback velocity, returns the flat knockback value
pub fn kb_from_initial(val: Velocity) -> f32 {
    let angle = val.as_angle();

    val.x / angle.cos() / 0.03
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// TODO account for stage so downward knockback doesn't count as killing
pub fn knockback_travel(
    mut kb: Velocity,
    mut position: Position,
    gravity: f32,
    max_fall_speed: f32,
) -> Vec<Position> {
    let kb_scalar = kb_from_initial(kb);
    let hitstun = hitstun(kb_scalar);

    let mut result = Vec::with_capacity(hitstun as usize + 1);
    result.push(position);

    let trajectory = kb.as_angle();

    let x_decay = get_horizontal_decay(trajectory);
    let y_decay = get_vertical_decay(trajectory);

    let mut self_y = 0.0;

    let x_direction = match trajectory > 180.0 && trajectory < 270.0 {
        true => Direction::Left,
        false => Direction::Right,
    };

    let y_direction = match trajectory > 0.0 && trajectory < 180.0 {
        true => Direction::Up,
        false => Direction::Down,
    };

    for _ in 0..hitstun {
        self_y = (self_y - gravity).max(max_fall_speed.abs() * -1.0); // coerce to negative regardless of sign
        match x_direction {
            Direction::Left => kb.x = (kb.x + x_decay).min(0.0),
            Direction::Right => kb.x = (kb.x - x_decay).max(0.0),
            _ => panic!("how did you get here"),
        }
        match y_direction {
            Direction::Up => kb.y = (kb.y - y_decay).max(0.0),
            Direction::Down => kb.y = (kb.y + y_decay).min(0.0),
            _ => panic!("how did you get here"),
        }

        position += kb;
        position.y += self_y;

        result.push(position);
    }

    result
}

pub fn should_kill(
    stage_id: u16,
    kb: Velocity,
    position: Position,
    gravity: f32,
    max_fall_speed: f32,
) -> bool {
    let stage = Stage::try_from(stage_id).unwrap();

    let travel = knockback_travel(kb, position, gravity, max_fall_speed);

    for pos in travel {
        if stage.is_past_blastzone(pos) {
            return true;
        }
    }

    false
}

// -------------------------------------------- Tests ------------------------------------------- //

// TODO broke tests due to changing knockback travel to take x/y directly instead of KB + trajectory
// #[test]
// fn test_knockback() {
//     let fox = Character::Fox.get_stats();
//     // marth's tipper fsmash
//     // trajectory: 361.0
//     let kb = knockback(
//         20.0, 20.0, 70, 80, 0, false, &fox, 80.0, false, false, false, false, false, false,
//     );
//     assert_relative_eq!(kb, 215.8);

//     // falco shine
//     // trajectory: 84.0
//     let kb = knockback(
//         8.0, 8.0, 50, 110, 0, false, &fox, 80.0, false, false, false, false, false, false,
//     );

//     assert_relative_eq!(kb, 154.2);
// }

// #[test]
// fn test_sakurai_angle() {
//     let falco = Character::Falco.get_stats();
//     // Marth sourspot jab
//     let kb = knockback(
//         4.0, 4.0, 50, 20, 0, false, &falco, 9.0, false, false, false, false, false, false,
//     );
//     let trajectory = 361.0;

//     assert_eq!(kb, 32.033333);
//     let modified = resolve_sakurai_angle(trajectory, kb, true);
//     assert_eq!(modified, 14.666443);

//     let dk = Character::DonkeyKong.get_stats();
//     // Marth sourspot jab, @ position 8 in the stale move queue
//     let kb = knockback(
//         3.92, 3.92, 50, 20, 0, false, &dk, 12.0, false, false, false, false, false, false,
//     );
//     assert_eq!(kb, 32.082825);
//     let modified = resolve_sakurai_angle(trajectory, kb, true);
//     assert_eq!(modified, 36.44287);
// }
