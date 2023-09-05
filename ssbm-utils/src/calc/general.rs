#[cfg(test)]
mod test;

/// Returns a sequence containing the character's height on each frame, relative to the starting location.
///
/// Note that trying to determine the number of frames airborne from the vector length will frequently result in
/// slightly incorrect amounts. Landing is decided by the bottom of the ECB, which is only tangentially related to the
/// current coordinate position of the character. The number will be close, but can off by 1 or 2 frames.

/// If grav_frame_1 is true, gravity will be applied on the first frame of the jump, as normally it is not. This occurs
/// when a player uses an aerial on the first frame airborne

pub fn jump_arc(
    jump_force: f32,
    gravity: f32,
    terminal_velocity: f32,
    grav_frame_1: bool,
) -> Vec<f32> {
    let mut arc = vec![0.0]; // treat starting location as 0, making measurements relative

    let mut vel: f32 = jump_force;
    if grav_frame_1 {
        vel -= gravity;
    }
    let mut height: f32 = 0.0;

    while height >= 0.0 {
        height += vel;
        arc.push(height);

        vel = (vel - gravity).clamp(-terminal_velocity, f32::MAX)
    }

    arc
}

/// Drain rate of the shield **per frame** based on trigger value.
///
/// See also: `SHIELD_REGEN_RATE` constant for regen/frame.
pub fn shield_drain_rate(analog: f32) -> f32 {
    let analog_scalar = 1.9 * ((analog - 0.3) / 0.7);

    (analog_scalar + 0.1) * 0.14
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
