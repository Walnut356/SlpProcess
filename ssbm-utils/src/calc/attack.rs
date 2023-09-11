#![allow(clippy::too_many_arguments)]

use std::{cmp, f32::consts::TAU};

use approx::assert_relative_eq;

use crate::{
    calc::knockback::knockback,
    constants::{KB_DECAY, TUMBLE_THRESHOLD, Z_ANALOG},
    enums::{character::*, stage::*}, types::Radians,
};

pub fn shield_stun(damage: f32, analog: f32, is_yoshi: bool) -> u32 {
    if is_yoshi {
        return 0;
    }
    let analog_scalar = 0.65 * (1.0 - (analog - 0.3) / 0.7);
    let shield_stun = (damage.floor() * (analog_scalar + 0.3) * 1.5 + 2.0) * (200.0 / 201.0);

    shield_stun.floor() as u32
}

// pub fn shield_size(shield_health: f32, analog: f32) -> f32 {
//     let analog_scalar = 1.0 - (0.5 * (analog - 0.3) / 0.7);
//     (shield_health * analog_scalar / 60.0) * 0.85 + 0.15
// }

pub fn shield_damage(damage: f32, analog: f32, powershield: bool) -> f32 {
    let mut ps_scalar: f32 = 1.0;
    if powershield {
        ps_scalar = 0.0;
    }

    let analog_scalar = 0.2 * (1.0 - (analog - 0.3) / 0.7);
    damage.floor() * (analog_scalar + 0.7) * ps_scalar
}

/// Calculates the hitlag for a given move.
///
/// electric modifier does not affect shield hits
pub fn hitlag(damage: f32, electric: bool, crouch_cancel: bool) -> u32 {
    let e: f32 = match electric {
        true => 1.5,
        false => 1.0,
    };
    let cc: f32 = match crouch_cancel {
        true => 2.0 / 3.0,
        false => 1.0,
    };
    cmp::min(
        ((((damage / 3.0).floor() + 3.0).floor() * e).floor() * cc).floor() as u32,
        20,
    )
}

/// Converts a knockback value into the number of frames the victim spends in hitstun
pub fn hitstun(knockback: f32) -> u32 {
    (knockback * 0.4).floor() as u32
}

/// Velocity imparted on the defender when their shield is hit. Decays by the defender's traction
/// per frame.
pub fn shield_pushback_defender(
    damage: f32,
    analog: f32,
    powershield: bool,
    is_yoshi: bool,
) -> f32 {
    let ps_scalar: f32 = {
        if powershield {
            1.0
        } else {
            0.6
        }
    };

    let d_push: f32 = {
        if is_yoshi {
            let a = 0.3 * (1.0 - (analog - 0.3) / 0.7);
            (damage.floor() * a) + 0.14
        } else {
            // only non-yoshi defender is capped
            let a = 0.195 * (1.0 - (analog - 0.3) / 0.7);
            ((damage.floor() * (a + 0.09) + 0.4) * ps_scalar).clamp(0.0, 2.0)
        }
    };

    d_push
}

/// Velocity imparted on the attacker when hitting shield
pub fn shield_pushback_attacker(damage: f32, analog: f32) -> f32 {
    let a = (analog - 0.3) * 0.1;
    (damage.floor() * a) + 0.02
}



/// Calculates staled damage based on a damage value and stale move queue.
///
/// Stale move queue is assumed to be in order from **least** recent to **most** recent. Each `true` represents an
/// instance of the move in the queue at that position. **Move order in the stale queue is very important**
pub fn staled_damage(damage: f32, stale_queue: &[bool]) -> f32 {
    assert_eq!(stale_queue.len(), 9);

    let mut result: f32 = 0.0;
    let mut scalar: f32 = 0.09;

    for &i in stale_queue {
        if i {
            result += scalar
        }
        scalar -= 0.01;
    }

    damage * (1.0 - result)
}

/// Calculates unstaled damage based on damage dealt and stale move queue.
///
/// Stale move queue is assumed to be in order from **least** recent to **most** recent. Each `true` represents an
/// instance of the move in the queue at that position. **Move order in the stale queue is very important**
pub fn unstaled_damage(damage: f32, stale_queue: &[bool]) -> f32 {
    assert_eq!(stale_queue.len(), 9);
    let mut result = 0.0;
    let mut scalar: f32 = 0.09;

    for &i in stale_queue {
        if i {
            result += scalar
        }
        scalar -= 0.01;
    }

    damage / (1.0 - result)
}

#[test]
fn test_shield_stun() {
    // the values for these are equal parts manually tested and not.
    // i'm not sure how uncle punch displays shield stun, but i don't think it quite lines up with how
    // the community defines it or how it works intuitively? I'm not sure. But UP's values are universally 1 higher than
    // these, maybe because the last frame of hitlag and the first frame of hitstun decrement at the same time?
    let ss = shield_stun(17.0, 1.0, false);
    assert_eq!(ss, 9);

    let ss = shield_stun(12.0, Z_ANALOG, false);
    assert_eq!(ss, 18);

    // tests weird animation shortening
    let ss = shield_stun(20.0, 1.0, false);
    assert_eq!(ss, 10);
}

#[test]
fn test_hitstun() {
    let fox = Character::Fox.get_stats();
    // marth's tipper fsmash
    // trajectory: 361.0
    let kb = knockback(
        20.0, 20.0, 70, 80, 0, false, &fox, 80.0, false, false, false, false, false, false,
    );
    let hs = hitstun(kb);

    assert_eq!(hs, 86);

    // falco shine
    // trajectory: 84.0
    let kb = knockback(
        8.0, 8.0, 50, 110, 0, false, &fox, 80.0, false, false, false, false, false, false,
    );
    let hs = hitstun(kb);

    assert_eq!(hs, 61);
}

#[test]
fn test_stale_damage() {
    let sd = staled_damage(
        15.0,
        &[true, true, false, false, false, true, false, false, false],
    );

    assert_relative_eq!(sd, 11.85);

    let usd = unstaled_damage(
        sd,
        &[true, true, false, false, false, true, false, false, false],
    );

    assert_eq!(usd, 15.0);
}
