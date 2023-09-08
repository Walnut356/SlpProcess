use ssbm_utils::{enums::character::*, calc::attack::*};

fn main() {
    let fox = Character::Fox.get_stats();
    let kb = knockback(
        20.0, 20.0, 50, 100, 0, false, &fox, 100.0, false, false, false, false, false, false,
    );
    let trajectory = resolve_sakurai_angle(70.0, kb, true);
    let hitstun = hitstun(kb);
    // let travel = knockback_travel(
    //     kb,
    //     trajectory,
    //     hitstun,
    //     fox.gravity,
    //     fox.terminal_velocity,
    //     -68.4,
    //     0.0,
    //     true,
    // );

    let comp_kb = kb_from_initial(initial_x_velocity(kb, trajectory), initial_y_velocity(kb, trajectory, true));

    // let (base, growth) = base_kb_from_kb(comp_kb, fox.weight, 20.0, 20.0, 100.0);

    println!("{:?} {:?}", kb, comp_kb)
}

// pub fn base_kb_from_kb(knockback: f32, weight: u32, damage_unstaled: f32, damage_staled: f32, percent: f32) -> (f32, f32) {
//     let n = (1.4
//                 * (((0.05 * (damage_unstaled * (damage_staled + percent.floor())))
//                     + (damage_staled + percent.floor()) * 0.1)
//                     * (2.0 - (2.0 * (weight as f32 * 0.01)) / (1.0 + (weight as f32 * 0.01)))))
//                 + 18.0;


// }