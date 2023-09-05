use crate::character::*;
use crate::general::*;
use crate::*;

use approx::*;

#[test]
fn test_fh_jump_arc() {
    let character = Character::Falco.get_stats();
    let ja = jump_arc(
        character.fh_jump_force,
        character.gravity,
        character.terminal_velocity,
        false,
    );

    assert_eq!(
        ja,
        vec![
            0.0, 4.1, 8.03, 11.789999, 15.379999, 18.8, 22.05, 25.13, 28.039999, 30.779999, 33.35,
            35.749996, 37.979996, 40.039993, 41.929993, 43.64999, 45.19999, 46.579987, 47.789986,
            48.829987, 49.699986, 50.399986, 50.929985, 51.289986, 51.479984, 51.499985, 51.349983,
            51.029984, 50.53998, 49.879982, 49.04998, 48.04998, 46.87998, 45.539978, 44.029976,
            42.349976, 40.499973, 38.479973, 36.28997, 33.92997, 31.39997, 28.699968, 25.829967,
            22.789967, 19.689966, 16.589966, 13.489965, 10.389965, 7.289965, 4.1899652, 1.0899653,
            -2.0100346
        ]
    );
}

#[test]
fn test_grav_jump_arc() {
    let character = Character::Falco.get_stats();
    let ja = jump_arc(
        character.fh_jump_force,
        character.gravity,
        character.terminal_velocity,
        true,
    );

    assert_eq!(
        ja,
        vec![
            0.0, 3.9299998, 7.6899996, 11.279999, 14.699999, 17.949999, 21.029999, 23.939999,
            26.679998, 29.249998, 31.649998, 33.879997, 35.939995, 37.829994, 39.54999, 41.09999,
            42.47999, 43.689987, 44.72999, 45.599987, 46.299988, 46.829987, 47.189987, 47.379986,
            47.399986, 47.249985, 46.929985, 46.439983, 45.779984, 44.94998, 43.94998, 42.77998,
            41.43998, 39.929977, 38.249977, 36.399975, 34.379974, 32.189972, 29.829971, 27.29997,
            24.59997, 21.729969, 18.689968, 15.589968, 12.489967, 9.389967, 6.289967, 3.1899672,
            0.08996725, -3.0100327
        ]
    );
}

#[test]
fn test_sh_jump_arc() {
    let character = Character::Falco.get_stats();
    let ja = jump_arc(
        character.sh_jump_force,
        character.gravity,
        character.terminal_velocity,
        false,
    );

    assert_eq!(
        ja,
        vec![
            0.0, 1.9, 3.63, 5.19, 6.58, 7.8, 8.85, 9.7300005, 10.440001, 10.9800005, 11.35, 11.55,
            11.58, 11.44, 11.13, 10.650001, 10.000001, 9.180001, 8.1900015, 7.0300016, 5.7000017,
            4.2000017, 2.5300019, 0.6900021, -1.3199977
        ]
    );
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

#[test]
fn test_drain_rate() {
    let hard_shield = shield_drain_rate(1.0);
    assert_eq!(hard_shield, 0.28);

    // the following values were manually tested in-game via unclepunch trianing mode
    // smash wiki lists different values for lightest shield degen, but smashwiki has been wrong before. I'm not sure if
    // they're using the wrong float size or what, but i've also had to correct their jump arc calculations.
    let lightest_shield = shield_drain_rate(TRIGGER_MIN);
    assert_relative_eq!(lightest_shield, 0.01671428);

    let z_shield = shield_drain_rate(Z_TRIGGER);

    // since uncle punch only has 4 digits of precision, it's easier to test over a long sample size,
    // in this case the number of frames necessary to break the shield, 1819 for z shield
    assert!(60.0 - (z_shield * 1818.0) >= 0.0);
    assert!(60.0 - (z_shield * 1819.0) <= 0.0);
}
