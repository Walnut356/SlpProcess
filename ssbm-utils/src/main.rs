use ssbm_utils::{attack::*, character::Character, *};

fn main() {
    let fox = Character::Fox.get_stats();
    let kb = knockback(
        20.0, 20.0, 70, 80, 0, false, &fox, 80.0, false, false, false, false, false, false,
    );
    let trajectory = resolve_sakurai_angle(361.0, kb, true);
    let hitstun = hitstun(kb);
    let travel = knockback_travel(
        kb,
        trajectory,
        hitstun,
        fox.gravity,
        fox.terminal_velocity,
        -68.4,
        0.0,
        true,
    );

    println!("{:?}", travel)
}
