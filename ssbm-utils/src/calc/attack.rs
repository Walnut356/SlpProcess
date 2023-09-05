#![allow(clippy::too_many_arguments)]

use std::{
    cmp,
    f32::consts::TAU,
};

use crate::{
    character::*, BATTLEFIELD_BLASTZONES, DREAMLAND_BLASTZONES, FD_BLASTZONES, FOUNTAIN_BLASTZONES,
    STADIUM_BLASTZONES, YOSHIS_BLASTZONES,
};

#[cfg(test)]
mod test;

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
    // if (trajectory > 180.0 && trajectory != 361.0) && grounded {
    //     if kb >= 80.0 {
    //         groundDownHitType = "Fly";
    //     } else {
    //         groundDownHitType = "Stay";
    //     }
    //     groundDownHit = true;
    // }

    kb
}

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

/// Checks if a given trajectory is a sakurai angle and returns a modified trajectory with the proper sakurai-angle
/// logic applied.
///
/// If the victim is airborne, the angle is 45 degrees.
///
/// If the victim is grounded and the knockback is less than or equal to 32.0, the angle is horizontal. If the victim is
/// grounded and the knockback is greater than or equal to 32.1, the knockback is 44 degrees. If the knockback is
/// **between** the thresholds, it is scaled linearly between 0 and 44 degrees. The gap between the thresholds is small
/// enough that it's rare that this situation comes up.
///
/// e.g. Marth sourspot jab vs falco at 9%, with a knockback value of 32.03333, thus a final knockback angle of
/// ~14.667 degrees
pub fn resolve_sakurai_angle(trajectory: f32, knockback: f32, grounded: bool) -> f32 {
    if trajectory != 361.0 {
        return trajectory;
    }

    if !grounded {
        return 45.0;
    }

    let kb_extreme = knockback <= 32.0 || knockback >= 32.1;

    if kb_extreme {
        if knockback <= 32.0 {
            return 0.0;
        } else {
            return 44.0;
        }
    }

    // at this point we can guarantee that 32.0 < knockback < 32.1
    // the following just treats 32 as 0 and 32.1 as 1, which gives us a percentage to scale by
    let scalar = (knockback - 32.0) * 10.0;

    44.0 * scalar
}

/// Modifies a knockback trajectory to account for DI based on a joystick X and Y value.
/// This should be done after dealing with trajectory modifiers such as sakurai angle
pub fn apply_di(trajectory: f32, joystick_x: f32, joystick_y: f32) -> f32 {
    let joystick_angle = point_to_angle(joystick_x, joystick_y);

    let mut angle_diff = trajectory - joystick_angle;
    if angle_diff > 180.0 {
        angle_diff -= 360.0;
    }

    let perp_dist = angle_diff.sin() * f32::hypot(joystick_x, joystick_y);
    let mut angle_offset = (perp_dist.powi(2)) * 18.0;

    if angle_offset > 18.0 {
        angle_offset = 18.0
    }
    if -180.0 < angle_diff && angle_diff < 0.0 {
        angle_offset *= -1.0;
    }

    trajectory - angle_offset
}

/// Returns a percentage representing how much the DI affected the final trajectory, relative to the
/// maximum possible effect that DI can have.
pub fn get_di_efficacy(old_trajectory: f32, new_trajectory: f32) -> f32 {
    (new_trajectory - old_trajectory).abs() / 18.0
}

/// Converts a knockback value and angle into the initial X knockback velocity imparted on the
/// character.
pub fn initial_x_velocity(knockback: f32, angle: f32) -> f32 {
    let magnitude = knockback * 0.03;
    let angle = angle.to_radians().cos();
    angle * magnitude
}

/// Converts a knockback value and angle into the initial Y knockback velocity imparted on the
/// character.
pub fn initial_y_velocity(knockback: f32, angle: f32, grounded: bool) -> f32 {
    let high_kb = knockback >= 80.0;

    if high_kb && grounded && (angle == 0.0 || angle == 180.0) {
        return 0.0;
    }

    let magnitude = knockback * 0.03;
    let angle = angle.to_radians().sin();
    let mut velocity = angle * magnitude;

    let down = angle > 180.0 && angle < 361.0;

    if down && high_kb {
        velocity *= 0.8;
    }

    velocity
}

/// Rate at which horizontal knockback velocity decreases per frame
pub fn get_horizontal_decay(angle: f32) -> f32 {
    0.051 * angle.to_radians().cos()
}

/// Rate at which vertical knockback velocity decreases per frame
/// Gravity also plays a role, but that is done in knockback_travel
pub fn get_vertical_decay(angle: f32) -> f32 {
    0.051 * angle.to_radians().sin()
}

pub fn will_tumble(kb: f32) -> bool {
    kb > 80.0
}

/// Accepts a tournament legal stage's in-game id and a pair of X, Y coordinates.
/// Returns true if the player is outside of the stage's blast zones.
pub fn is_past_blastzone(stage: u16, position_x: f32, position_y: f32) -> bool {
    let blast_zones = match stage {
        2 => FOUNTAIN_BLASTZONES,
        3 => STADIUM_BLASTZONES,
        8 => YOSHIS_BLASTZONES,
        28 => DREAMLAND_BLASTZONES,
        31 => BATTLEFIELD_BLASTZONES,
        32 => FD_BLASTZONES,
        _ => panic!("invalid stage ID"),
    };
    use crate::BlastZone::*;

    !(position_x < blast_zones[Right as usize]
        && position_x > blast_zones[Left as usize]
        && position_y < blast_zones[Top as usize]
        && position_y > blast_zones[Bottom as usize])
}

pub fn point_to_angle(x: f32, y: f32) -> f32 {
    (f32::atan2(y, x) + TAU) % TAU
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn knockback_travel(
    knockback: f32,
    trajectory: f32,
    hitstun: u32,
    gravity: f32,
    max_fall_speed: f32,
    mut position_x: f32,
    mut position_y: f32,
    grounded: bool,
) -> Vec<(f32, f32)> {
    let mut result = Vec::with_capacity(hitstun as usize + 1);
    result.push((position_x, position_y));

    let x_decay = get_horizontal_decay(trajectory);
    let y_decay = get_vertical_decay(trajectory);

    let mut self_y = 0.0;
    let mut kb_x = initial_x_velocity(knockback, trajectory);
    let mut kb_y = initial_y_velocity(knockback, trajectory, grounded);

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
            Direction::Left => kb_x = (kb_x + x_decay).min(0.0),
            Direction::Right => kb_x = (kb_x - x_decay).max(0.0),
            _ => panic!("how did you get here"),
        }
        match y_direction {
            Direction::Up => kb_y = (kb_y - y_decay).max(0.0),
            Direction::Down => kb_y = (kb_y + y_decay).min(0.0),
            _ => panic!("how did you get here"),
        }

        position_x += kb_x;
        position_y += kb_y + self_y;

        result.push((position_x, position_y));
    }

    result
}
