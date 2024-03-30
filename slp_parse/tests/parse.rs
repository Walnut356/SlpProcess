use std::{
    collections::HashMap, iter::zip, path::{Path, PathBuf}, sync::Arc, time::Duration
};

use slp_parse::{events::game_end::Placement, prelude::*};
use slp_parse::{
    events::{
        game_end::{EndMethod, GameEnd},
        game_start::{ControllerFix, GameStart, MatchType, Version},
        post_frame::PostRow,
        pre_frame::PreRow,
    },
    frames::Frame,
    player::UCFToggles,
};
use ssbm_utils::{
    enums::{character::Costume, Character, ControllerInput, EngineInput, Flags, StageID},
    pos, stick_pos,
    types::{Position, StickPos, Velocity},
    vel,
};

// for some reason "Run Tests" and "Debug" use different working folders. This is a dumb workaround
// for a dumb problem. You might have to replace the backslash with a forward slash on linux.
pub fn test_data_path(file_path: &str) -> PathBuf {
    match std::env::var("ENV_ROOT_DIR") {
        Ok(path) => Path::new(&path).join(file_path),
        Err(_) => Path::new(&std::env::current_dir().unwrap())
            .join(r"..\")
            .join(file_path),
    }
}

#[test]
pub fn test_metadata() {
    let replay = test_data_path(r"test_replays\netplay_sample.slp");
    let game = Game::new(&replay, false).unwrap();

    let metadata = GameStart {
        random_seed: 32794,
        teams: false,
        stage: StageID::YOSHIS_STORY,
        timer: Duration::from_secs(8 * 60),
        damage_ratio: 1.0,
        pal: Some(false),
        frozen_stadium: Some(false),
        netplay: Some(true),
        match_id: Arc::new("mode.direct-2023-11-29T00:26:03.31-0".to_owned()),
        match_type: MatchType::Direct,
        game_number: Some(2),
        tiebreak_number: Some(0),
        // date: OffsetDateTime::parse("2023-11-29T00:26:22+00:00", &Iso8601::DEFAULT).unwrap(),
    };

    assert_eq!(game.metadata().start, metadata);
    assert_eq!(game.total_frames(), 9809);
    assert_eq!(
        game.duration(),
        Duration::from_millis((((9809.0 - 124.0) / 60.0) * 1000.0) as u64)
    );
    assert_eq!(game.version(), Version::new(3, 16, 0));
    assert_eq!(
        game.end().unwrap(),
        &GameEnd {
            end_method: EndMethod::Stocks,
            lras_initiator: None,
            placements: {
                let mut temp = HashMap::new();
                temp.insert(Port::P1, Placement::Win);
                temp.insert(Port::P2, Placement::Loss);
                Some(temp)
            }
        }
    );
    assert_eq!(game.item_frames.as_ref().unwrap().len(), 14232);
}

#[test]
pub fn test_players() {
    let replay = test_data_path(r"test_replays\netplay_sample.slp");
    let game = Game::new(&replay, false).unwrap();

    let players = &game.players;

    assert!(players[0].character == Character::Falco && players[1].character == Character::Falco);
    assert!(players[0].costume == Costume::GREEN && players[1].costume == Costume::RED);
    assert!(players[0].port == Port::P1 && players[1].port == Port::P2);
    assert!(
        players[0].connect_code == Some("DERE#660".into())
            && players[1].connect_code == Some("NUT#356".into())
    );
    assert!(
        players[0].display_name == Some("Dereo".into())
            && players[1].display_name == Some("Walnut".into())
    );
    assert!(players[0].is_winner == Some(true) && players[1].is_winner == Some(false));
    assert!(
        players[0].ucf
            == Some(UCFToggles {
                dashback: ControllerFix::UCF,
                shield_drop: ControllerFix::UCF
            })
            && players[1].ucf
                == Some(UCFToggles {
                    dashback: ControllerFix::UCF,
                    shield_drop: ControllerFix::UCF
                })
    );
    assert!(
        players[0].frames.len() == game.total_frames() as usize
            && players[1].frames.len() == game.total_frames() as usize
    );
    assert!(players[0].nana_frames.is_none() && players[0].nana_frames.is_none());
}

#[test]
pub fn test_frames() {
    let replay = test_data_path(r"test_replays\netplay_sample.slp");
    let game = Game::new(&replay, false).unwrap();

    let p1_frames = &game.players[0].frames;
    let p2_frames = &game.players[1].frames;

    let mut index = -123;
    for (&i_1, &i_2) in zip(
        p1_frames.pre.frame_index.iter(),
        p1_frames.post.frame_index.iter(),
    ) {
        assert_eq!(index, i_1);
        assert_eq!(index, i_2);
        index += 1;
    }

    // Check first and last frame, as well as 4 frames that I RNG'd to select.
    // Values are verified against py-slippi-stats, which I'm reasonably sure is accurate.

    assert_eq!(
        p1_frames.get_frame(0),
        Frame(
            PreRow {
                character: Character::Falco,
                frame_index: -123,
                random_seed: 32794,
                action_state: 322,
                position: pos!(-42.0, 26.60),
                orientation: 1.0,
                joystick: stick_pos!(0.0, 0.0),
                cstick: stick_pos!(0.0, 0.0),
                engine_trigger: 0.0,
                engine_buttons: 0,
                controller_buttons: 0,
                controller_l: 0.0,
                controller_r: 0.0,
                raw_stick_x: Some(0),
                percent: Some(0.0),
                raw_stick_y: Some(0),
            },
            PostRow {
                frame_index: -123,
                character: 22,
                action_state: 322,
                position: pos!(-42.0, 26.60),
                orientation: 1.0,
                percent: 0.0,
                shield_health: 60.0,
                last_attack_landed: 0,
                combo_count: 0,
                last_hit_by: 6,
                stocks: 4,
                state_frame: Some(-1.0),
                flags: Some(Flags::DEAD.into()),
                misc_as: Some(f32::from_be_bytes([0, 0, 0, 4])),
                is_grounded: Some(false),
                last_ground_id: Some(u16::MAX),
                jumps_remaining: Some(1),
                l_cancel: Some(0),
                hurtbox_state: Some(0),
                air_velocity: Some(vel!(0.00, 0.00)),
                knockback: Some(vel!(0.00, 0.00)),
                ground_velocity: Some(vel!(0.00, 0.00)),
                hitlag_remaining: Some(0.0),
                animation_index: Some(u32::MAX),
                instance_hit_by: Some(0),
                instance_id: Some(1),
            }
        )
    );

    assert_eq!(
        p1_frames.get_frame(p1_frames.len() - 1),
        Frame(
            PreRow {
                character: Character::Falco,
                frame_index: 9685,
                random_seed: 642809882,
                action_state: 14,
                position: pos!(
                    f32::from_be_bytes([66, 43, 234, 128]),
                    f32::from_be_bytes([191, 73, 133, 112])
                ),
                orientation: 1.0,
                joystick: stick_pos!(0.0, 0.0),
                cstick: stick_pos!(0.0, 0.0),
                engine_trigger: 0.0,
                engine_buttons: 0,
                controller_buttons: 0,
                controller_l: 0.007142857,
                controller_r: 0.0,
                raw_stick_x: Some(0),
                percent: Some(21.22),
                raw_stick_y: Some(0),
            },
            PostRow {
                frame_index: 9685,
                character: 22,
                action_state: 14,
                position: pos!(
                    f32::from_be_bytes([66, 43, 234, 128]),
                    f32::from_be_bytes([191, 73, 133, 112])
                ),
                orientation: 1.0,
                percent: 21.22,
                shield_health: 60.0,
                last_attack_landed: 12,
                combo_count: 1,
                last_hit_by: 6,
                stocks: 1,
                state_frame: Some(11.0),
                flags: Some((Flags::BIT_1_7 | Flags::ALLOW_INTERRUPT).into()),
                misc_as: Some(f32::from_be_bytes([0, 0, 0, 1])),
                is_grounded: Some(true),
                last_ground_id: Some(6),
                jumps_remaining: Some(2),
                l_cancel: Some(0),
                hurtbox_state: Some(0),
                air_velocity: Some(vel!(0.0, 0.0)),
                knockback: Some(vel!(0.0, 0.0)),
                ground_velocity: Some(vel!(0.0, 0.0)),
                hitlag_remaining: Some(0.0),
                animation_index: Some(2),
                instance_hit_by: Some(2028),
                instance_id: Some(2100),
            }
        )
    );

    assert_eq!(
        p1_frames.get_frame(3887),
        Frame(
            PreRow {
                character: Character::Falco,
                frame_index: 3764,
                random_seed: 254771226,
                action_state: 88,
                position: pos!(43.118572, -2.8006186),
                orientation: -1.0,
                joystick: stick_pos!(0.0, 0.0),
                cstick: stick_pos!(0.0, 0.0),
                engine_trigger: 0.0,
                engine_buttons: 0,
                controller_buttons: 0,
                controller_l: 0.021428572,
                controller_r: 0.0,
                raw_stick_x: Some(0),
                percent: Some(128.81566),
                raw_stick_y: Some(0),
            },
            PostRow {
                frame_index: 3764,
                character: 22,
                action_state: 88,
                position: pos!(44.68274, -4.8087397),
                orientation: -1.0,
                percent: 128.81566,
                shield_health: 52.299965,
                last_attack_landed: 21,
                combo_count: 1,
                last_hit_by: 1,
                stocks: 3,
                state_frame: Some(29.0),
                flags: Some((Flags::ALLOW_INTERRUPT | Flags::HITSTUN | Flags::OFFSCREEN).into()),
                misc_as: Some(13.0),
                is_grounded: Some(false),
                last_ground_id: Some(1),
                jumps_remaining: Some(1),
                l_cancel: Some(0),
                hurtbox_state: Some(0),
                air_velocity: Some(vel!(0.0, -3.10)),
                knockback: Some(vel!(1.564166, 1.0918791)),
                ground_velocity: Some(vel!(0.00, -3.10)),
                hitlag_remaining: Some(0.0),
                animation_index: Some(178),
                instance_hit_by: Some(831),
                instance_id: Some(832),
            }
        )
    );

    assert_eq!(
        p1_frames.get_frame(2231),
        Frame(
            PreRow {
                character: Character::Falco,
                frame_index: 2108,
                random_seed: 146243610,
                action_state: 76,
                position: pos!(-21.21875, 0.0001),
                orientation: 1.0,
                joystick: stick_pos!(0.0, 0.0),
                cstick: stick_pos!(0.0, 0.0),
                engine_trigger: 1.0,
                engine_buttons: (EngineInput::ANY_TRIGGER | EngineInput::R).into(),
                controller_buttons: ControllerInput::R.into(),
                controller_l: 0.0,
                controller_r: 0.0,
                raw_stick_x: Some(0),
                percent: Some(33.395657),
                raw_stick_y: Some(0),
            },
            PostRow {
                frame_index: 2108,
                character: 22,
                action_state: 76,
                position: pos!(-21.21875, 0.0001),
                orientation: 1.0,
                percent: 33.395657,
                shield_health: 51.179993,
                last_attack_landed: 17,
                combo_count: 1,
                last_hit_by: 1,
                stocks: 3,
                state_frame: Some(1.0),
                flags: Some(
                    (Flags::BIT_1_3 | Flags::DEFENDER_HITLAG | Flags::HITLAG | Flags::HITSTUN)
                        .into()
                ),
                misc_as: Some(18.0),
                is_grounded: Some(false),
                last_ground_id: Some(3),
                jumps_remaining: Some(1),
                l_cancel: Some(0),
                hurtbox_state: Some(0),
                air_velocity: Some(vel!(0.0, 0.0)),
                knockback: Some(vel!(-0.99772424, 0.9634912)),
                ground_velocity: Some(vel!(0.0, 0.0)),
                hitlag_remaining: Some(5.0),
                animation_index: Some(166),
                instance_hit_by: Some(502),
                instance_id: Some(503),
            }
        )
    );

    assert_eq!(
        p2_frames.get_frame(7974),
        Frame(
            PreRow {
                character: Character::Falco,
                frame_index: 7851,
                random_seed: 522616858,
                action_state: 345,
                position: pos!(5.677475, 10.980101),
                orientation: 1.0,
                joystick: stick_pos!(0.0, 0.0),
                cstick: stick_pos!(0.0, 0.0),
                engine_trigger: 0.0,
                engine_buttons: EngineInput::B.into(),
                controller_buttons: ControllerInput::B.into(),
                controller_l: 0.0,
                controller_r: 0.0,
                raw_stick_x: Some(0),
                percent: Some(70.29),
                raw_stick_y: Some(0),
            },
            PostRow {
                frame_index: 7851,
                character: 22,
                action_state: 345,
                position: pos!(5.677475, 11.3501005),
                orientation: 1.0,
                percent: 70.29,
                shield_health: 56.500015,
                last_attack_landed: 15,
                combo_count: 1,
                last_hit_by: 6,
                stocks: 2,
                state_frame: Some(0.0),
                flags: Some(Flags::BIT_1_7.into()),
                misc_as: Some(0.0),
                is_grounded: Some(false),
                last_ground_id: Some(3),
                jumps_remaining: Some(1),
                l_cancel: Some(0),
                hurtbox_state: Some(0),
                air_velocity: Some(vel!(0.0, 0.37000012)),
                knockback: Some(vel!(0.0, 0.0)),
                ground_velocity: Some(vel!(0.0, 0.37000012)),
                hitlag_remaining: Some(0.0),
                animation_index: Some(299),
                instance_hit_by: Some(1602),
                instance_id: Some(1707),
            }
        )
    );

    assert_eq!(
        p2_frames.get_frame(9626),
        Frame(
            PreRow {
                character: Character::Falco,
                frame_index: 9503,
                random_seed: 630882330,
                action_state: 90,
                position: pos!(55.381676, 31.87702),
                orientation: 1.0,
                joystick: stick_pos!(-1.0, 0.0),
                cstick: stick_pos!(0.0, 0.0),
                engine_trigger: 0.7285714,
                engine_buttons: (EngineInput::ANY_TRIGGER | EngineInput::JOYSTICK_LEFT).into(),
                controller_buttons: ControllerInput::None.into(),
                controller_l: 0.0,
                controller_r: 0.7285714,
                raw_stick_x: Some(-98),
                percent: Some(67.91),
                raw_stick_y: Some(0),
            },
            PostRow {
                frame_index: 9503,
                character: 22,
                action_state: 90,
                position: pos!(55.818798, 30.95254),
                orientation: 1.0,
                percent: 67.91,
                shield_health: 60.00,
                last_attack_landed: 2,
                combo_count: 0,
                last_hit_by: 0,
                stocks: 1,
                state_frame: Some(43.0),
                flags: Some((Flags::BIT_1_7 | Flags::HITSTUN).into()),
                misc_as: Some(16.0),
                is_grounded: Some(false),
                last_ground_id: Some(3),
                jumps_remaining: Some(1),
                l_cancel: Some(0),
                hurtbox_state: Some(0),
                air_velocity: Some(vel!(0.0, -3.10)),
                knockback: Some(vel!(0.43712196, 2.17552)),
                ground_velocity: Some(vel!(0.0, -3.10)),
                hitlag_remaining: Some(0.0),
                animation_index: Some(180),
                instance_hit_by: Some(2069),
                instance_id: Some(2070),
            }
        )
    );
}
