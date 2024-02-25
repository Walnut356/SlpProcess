use duckdb::{params, Connection, DefaultOrder, Result};
use slp_parse::prelude::*;
use time::UtcOffset;

pub fn create_stubs(conn: &Connection, stubs: &[GameStub]) -> Result<()> {
    conn.execute_batch(
        r"
            CREATE TYPE MatchType AS ENUM ('Unranked', 'Ranked', 'Direct', 'Unknown');
            CREATE TYPE StageID AS ENUM ('FOUNTAIN_OF_DREAMS', 'POKEMON_STADIUM', 'YOSHIS_STORY', 'DREAM_LAND_N64', 'BATTLEFIELD', 'FINAL_DESTINATION');
            CREATE TYPE Port AS ENUM ('P1', 'P2', 'P3', 'P4');
            CREATE TYPE Character AS ENUM ('CaptainFalcon', 'DonkeyKong', 'Fox', 'GameAndWatch', 'Kirby', 'Bowser', 'Link', 'Luigi', 'Mario', 'Marth', 'Mewtwo', 'Ness', 'Peach', 'Pikachu', 'IceClimbers', 'Jigglypuff', 'Samus', 'Yoshi', 'Zelda', 'Sheik', 'Falco', 'YoungLink', 'DrMario', 'Roy', 'Pichu', 'Ganondorf');
            CREATE TYPE Costume AS ENUM ('DEFAULT', 'INDIGO', 'BLACK', 'RED', 'WHITE', 'GREEN', 'BLUE', 'BROWN', 'PINK', 'YELLOW', 'PURPLE', 'CYAN', 'NAVY', 'TAN', 'ORANGE', 'DAISY', 'CAP', 'PARTY_HAT', 'COWBOY', 'ROSE', 'BOW', 'HEADBAND', 'CROWN', 'BANDANA', 'GOGGLES', 'BACKPACK');
            CREATE TABLE stub (
                datetime TIMESTAMP_MS,
                fname STRING UNIQUE,
                version INT,
                match_id STRING,
                netplay BOOL,
                match_type MatchType,
                game UINTEGER,
                tiebreak UINTEGER,
                duration BIGINT,
                stage StageID,
                p1_port Port,
                p1_code STRING,
                p1_name STRING,
                p1_character Character,
                p1_costume Costume,
                p2_port Port,
                p2_code STRING,
                p2_name STRING,
                p2_character Character,
                p2_costume Costume,
            );
        ")?;

    let mut app = conn.appender("stub").unwrap();
    for (_i, stub) in stubs.iter().enumerate() {
        app.append_row(
            params![
                duckdb::types::Value::Timestamp(
                    duckdb::types::TimeUnit::Millisecond,
                    (stub
                        .date()
                        .to_offset(UtcOffset::current_local_offset().unwrap())
                        .unix_timestamp_nanos()
                        / 1000000) as i64
                ),
                stub.path().to_str(),
                stub.version().as_u32(),
                stub.match_id(),
                stub.netplay(),
                stub.match_type().to_string(),
                stub.game_number(),
                stub.tiebreak_number(),
                stub.duration().as_millis() as u64,
                stub.stage().to_string(),
                stub.players[0].port.to_string(),
                stub.players[0].connect_code,
                stub.players[0].display_name,
                stub.players[0].character.to_string(),
                stub.players[0].costume.to_string(),
                stub.players[1].port.to_string(),
                stub.players[1].connect_code,
                stub.players[1].display_name,
                stub.players[1].character.to_string(),
                stub.players[1].costume.to_string(),
            ],
        )?;
    }

    app.flush();

    Ok(())
}

pub fn create_frame_tables() -> Result<()> {
    let replay = r"E:\Slippi Replays\Netplay\";

    let games = slp_parse::parse(replay, true);

    let conn = Connection::open_in_memory()?;

    conn.execute_batch(
        r"
            CREATE TYPE MatchType AS ENUM ('Unranked', 'Ranked', 'Direct', 'Unknown');
            CREATE TYPE StageID AS ENUM ('FOUNTAIN_OF_DREAMS', 'POKEMON_STADIUM', 'YOSHIS_STORY', 'DREAM_LAND_N64', 'BATTLEFIELD', 'FINAL_DESTINATION');
            CREATE TYPE Port AS ENUM ('P1', 'P2', 'P3', 'P4');
            CREATE TYPE Character AS ENUM ('CaptainFalcon', 'DonkeyKong', 'Fox', 'GameAndWatch', 'Kirby', 'Bowser', 'Link', 'Luigi', 'Mario', 'Marth', 'Mewtwo', 'Ness', 'Peach', 'Pikachu', 'IceClimbers', 'Jigglypuff', 'Samus', 'Yoshi', 'Zelda', 'Sheik', 'Falco', 'YoungLink', 'DrMario', 'Roy', 'Pichu', 'Ganondorf');
            CREATE TYPE Costume AS ENUM ('DEFAULT', 'INDIGO', 'BLACK', 'RED', 'WHITE', 'GREEN', 'BLUE', 'BROWN', 'PINK', 'YELLOW', 'PURPLE', 'CYAN', 'NAVY', 'TAN', 'ORANGE', 'DAISY', 'CAP', 'PARTY_HAT', 'COWBOY', 'ROSE', 'BOW', 'HEADBAND', 'CROWN', 'BANDANA', 'GOGGLES', 'BACKPACK');
            CREATE TABLE pre_frame (
                path STRING,
                port Port,
                character Character,
                frame_index INT,
                random_seed UINTEGER,
                action_state USMALLINT,
                position_x FLOAT,
                position_y FLOAT,
                orientation FLOAT,
                stick_pos_x FLOAT,
                stick_pos_y FLOAT,
                cstick_x FLOAT,
                cstick_y FLOAT,
                engine_trigger FLOAT,
                engine_buttons UINTEGER,
                controller_buttons USMALLINT,
                controller_l FLOAT,
                controller_r FLOAT,
                percent FLOAT,
            );

            CREATE TABLE post_frame (
                path STRING,
                port Port,
                character Character,
                frame_index INT,
                action_state USMALLINT,
                position_x FLOAT,
                position_y FLOAT,
                orientation FLOAT,
                percent FLOAT,
                shield_health FLOAT,
                last_attack_landed UTINYINT,
                combo_count UTINYINT,
                last_hit_by UTINYINT,
                stocks UTINYINT,
                state_frame FLOAT,
                flags UBIGINT,
                misc_as FLOAT,
                is_grounded BOOL,
                last_ground_id USMALLINT,
                jumps_remaining UTINYINT,
                l_cancel UTINYINT,
                hurtbox_state UTINYINT,
                air_vel_x FLOAT,
                air_vel_y FLOAT,
                kb_x FLOAT,
                kb_y FLOAT,
                ground_vel_x FLOAT,
                ground_vel_y FLOAT,
                hitlag_remaining FLOAT,
                animation_index UINTEGER,
            );
        ")?;

    let mut pre_app = conn.appender("pre_frames").unwrap();
    let mut post_app = conn.appender("post_frames").unwrap();
    for (j, game) in games.iter().enumerate() {
        for player in &game.players {
            for i in 0..player.frames.len() {
                pre_app.append_row(params![
                    game.path().file_name().unwrap().to_str(),
                    player.port.to_string(),
                    player.character.to_string(),
                    player.frames.pre.frame_index[i],
                    player.frames.pre.random_seed[i],
                    player.frames.pre.action_state[i],
                    player.frames.pre.position[i].x,
                    player.frames.pre.position[i].y,
                    player.frames.pre.orientation[i],
                    player.frames.pre.joystick[i].x,
                    player.frames.pre.joystick[i].y,
                    player.frames.pre.cstick[i].x,
                    player.frames.pre.cstick[i].y,
                    player.frames.pre.engine_trigger[i],
                    player.frames.pre.engine_buttons[i],
                    player.frames.pre.controller_buttons[i],
                    player.frames.pre.controller_l[i],
                    player.frames.pre.controller_r[i],
                    player.frames.pre.percent.as_ref().unwrap()[i],
                ])?;
                post_app.append_row(params![
                    game.path().file_name().unwrap().to_str(),
                    player.port.to_string(),
                    player.frames.post.character[i].to_string(),
                    player.frames.post.frame_index[i],
                    player.frames.post.action_state[i],
                    player.frames.post.position[i].x,
                    player.frames.post.position[i].y,
                    player.frames.post.orientation[i],
                    player.frames.post.percent[i],
                    player.frames.post.shield_health[i],
                    player.frames.post.last_attack_landed[i],
                    player.frames.post.combo_count[i],
                    player.frames.post.last_hit_by[i],
                    player.frames.post.stocks[i],
                    player.frames.post.state_frame.as_ref().unwrap()[i],
                    player.frames.post.flags.as_ref().unwrap()[i],
                    player.frames.post.misc_as.as_ref().unwrap()[i],
                    player.frames.post.is_grounded.as_ref().unwrap()[i],
                    player.frames.post.last_ground_id.as_ref().unwrap()[i],
                    player.frames.post.jumps_remaining.as_ref().unwrap()[i],
                    player.frames.post.l_cancel.as_ref().unwrap()[i],
                    player.frames.post.hurtbox_state.as_ref().unwrap()[i],
                    player.frames.post.air_velocity.as_ref().unwrap()[i].x,
                    player.frames.post.air_velocity.as_ref().unwrap()[i].y,
                    player.frames.post.knockback.as_ref().unwrap()[i].x,
                    player.frames.post.knockback.as_ref().unwrap()[i].y,
                    player.frames.post.ground_velocity.as_ref().unwrap()[i].x,
                    player.frames.post.ground_velocity.as_ref().unwrap()[i].y,
                    player.frames.post.hitlag_remaining.as_ref().unwrap()[i],
                    player.frames.post.animation_index.as_ref().unwrap()[i],
                ])?;
            }
        }
    }

    pre_app.flush();

    Ok(())
}

pub fn export(conn: &Connection, name: &str) -> Result<()> {
    conn.execute(
        &format!("EXPORT DATABASE '{name}' (FORMAT PARQUET, COMPRESSION ZSTD);"),
        [],
    )?;

    Ok(())
}
