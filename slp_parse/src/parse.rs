#![allow(non_upper_case_globals)]

use anyhow::{anyhow, ensure, Result};
use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, Bytes};
use strum_macros::FromRepr;
use time::{format_description::well_known::Iso8601, OffsetDateTime};

use std::path::Path;
use std::time::Duration;
use std::{collections::HashMap, fs::File};
use std::{
    io::{prelude::*, BufReader, SeekFrom},
    sync::Arc,
};

use crate::game::{GameStub, Metadata};
use crate::{
    events::{
        game_end::parse_gameend, game_start::GameStart, item_frames::parse_itemframes,
        post_frame::parse_postframes, pre_frame::parse_preframes,
    },
    frames::Frames,
    ubjson,
    utils::ParseError,
    Game,
};

use ssbm_utils::enums::character::Character;

trait AsFrames {
    fn as_frames(&self) -> u64;
}

impl AsFrames for Duration {
    fn as_frames(&self) -> u64 {
        (*self / 60).as_secs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr, Default, Hash)]
#[repr(u8)]
enum EventType {
    EventPayloads = 0x35,
    GameStart = 0x36,
    PreFrame = 0x37,
    PostFrame = 0x38,
    GameEnd = 0x39,
    FrameStart = 0x3A,
    Item = 0x3B,
    FrameEnd = 0x3C,
    GeckoList = 0x3D,
    MessageSplitter = 0x10,
    #[default]
    None = 0x00,
}

fn expect_bytes(stream: &mut Bytes, expected: &[u8]) -> std::io::Result<()> {
    let actual = stream.get(0..expected.len()).unwrap();
    if expected == actual {
        stream.advance(expected.len());
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("got {actual:?}, expected {expected:?}"),
        ))
    }
}

impl Game {
    pub(crate) fn get_file_contents(path: &Path) -> Result<Bytes> {
        let mut f = File::open(path)?;
        let file_length = f.metadata()?.len() as usize;
        // #[cfg(debug_assertions)]
        // dbg!(file_length);
        let mut file_data = vec![0; file_length];
        f.read_exact(&mut file_data).unwrap();

        Ok(Bytes::from(file_data))
    }

    fn get_event_sizes(file: &mut Bytes) -> Result<HashMap<EventType, u16>> {
        let code = EventType::from_repr(file.get_u8()).unwrap();
        ensure!(
            code == EventType::EventPayloads,
            ParseError::Value(
                format!("{:?}", EventType::EventPayloads),
                format!("{:?}", code)
            )
        );

        let payloads_size = file.get_u8();

        ensure!(
            (payloads_size - 1) % 3 == 0,
            anyhow!("EventPayloads length invalid")
        );

        let mut event_map = HashMap::default();

        for _ in (0..(payloads_size - 1)).step_by(3) {
            let event = EventType::from_repr(file.get_u8()).unwrap();
            let size = file.get_u16();
            event_map.insert(event, size);
        }

        Ok(event_map)
    }

    /// Accepts a tokio Bytes object, returns a Game object. Useful if you already have the file in
    /// memory for some other reason
    pub fn parse(file_data: Bytes, path: &Path) -> Result<Self> {
        // ---------------------------------------- setup --------------------------------------- //

        /*
        I used a Cursor over the bytes object internally for a while, but due to the way Bytes
        and Buf are implemented, Cursor<Bytes> is much harder to optimize and requires extra derefs.
        Using a raw bytes object, the entire `get_x` function can be reduced to a pointer cast, a
        pointer add, and a bounds check. While cloning the Bytes object is more expensive than
        setting the Cursor's position, it's still cheap and should be worth it considering how many
        `get_x` functions are in the hot loops.
        */

        // We treat `file_data` as the "canonical" Bytes object, and any time we need to set the
        // position, it needs to be cloned so that we retain at least 1 instance with the original's
        // pointer offset.
        let mut stream = file_data.slice(..);

        expect_bytes(
            &mut stream,
            &[
                0x7b, 0x55, 0x03, 0x72, 0x61, 0x77, 0x5b, 0x24, 0x55, 0x23, 0x6c,
            ],
        )?;

        let raw_length = stream.get_u32() as u64 + 15;

        // ----------------------------------- metadata block ----------------------------------- //
        let mut temp_meta = file_data.slice(raw_length as usize..);

        expect_bytes(
            &mut temp_meta,
            // `metadata` key & type ("U\x08metadata{")
            &[
                0x55, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x7b,
            ],
        )?;

        let mut frame_count: usize = 0;

        let mut duration: Duration = Duration::default();
        let metadata = ubjson::to_map(&mut temp_meta.reader())?;
        if let serde_json::Value::Number(lastframe) = &metadata["lastFrame"] {
            // duration, in frames, is translated to seconds. 123 is subtracted from the frame count
            // to match the duration to the in-game timer. The total frame count is easily
            // found from player.frames.len()
            let last = lastframe.as_i64().unwrap();
            frame_count = (last + 124) as usize;
            let millis = ((last.max(0) as f32 / 60.0) * 1000.0) as u64;

            // i shouldn't have to do any checks on this conversion
            duration = Duration::from_millis(millis);
        };

        let mut metadata_identifiers = [("", ""), ("", "")];

        if let serde_json::Value::Object(ps) = &metadata["players"] {
            let mut i = 0;
            for (k, v) in ps.iter() {
                if let serde_json::Value::Object(player_vals) = v {
                    if let serde_json::Value::Object(names) = &player_vals["names"] {
                        metadata_identifiers[i].0 = match names.get("code") {
                            Some(x) => x.as_str().unwrap_or_default(),
                            None => "",
                        };
                        metadata_identifiers[i].1 = match names.get("netplay") {
                            Some(x) => x.as_str().unwrap_or_default(),
                            None => "",
                        };
                    }
                }
                i += 1;
            }
        }

        let mut date = OffsetDateTime::UNIX_EPOCH;
        if let serde_json::Value::String(start_at) = &metadata["startAt"] {
            date = OffsetDateTime::parse(start_at.as_str(), &Iso8601::DEFAULT)
                .unwrap_or(OffsetDateTime::UNIX_EPOCH);
        }

        // ------------------------------------- game start ------------------------------------- //
        let event_sizes = Self::get_event_sizes(&mut stream)?;

        assert_eq!(stream.get_u8(), EventType::GameStart as u8);

        let raw_start = stream.slice(0..event_sizes[&EventType::GameStart] as usize);
        stream.advance(event_sizes[&EventType::GameStart] as usize);

        // .slice(
        //     // wow this is exceptionally ugly! thanks rust =)
        //     stream.position() as usize
        //         ..(stream.position() + event_sizes[&EventType::GameStart.into()] as u64) as usize,
        // );

        let (game_start, version, mut players) = GameStart::parse(raw_start)?;

        for (i, player) in players.iter_mut().enumerate() {
            if !version.at_least(3, 9, 0) {
                player.connect_code = Some(metadata_identifiers[i].0.to_owned());
                player.display_name = Some(metadata_identifiers[i].1.to_owned());
            }
        }

        // i could map but this gives me arrays instead of slices without into
        let ports = [players[0].port, players[1].port];
        let ics = [
            players[0].character == Character::IceClimbers,
            players[1].character == Character::IceClimbers,
        ];

        // stream.set_position(stream.position() + event_sizes[&EventType::GameStart.into()] as u64);

        let mut game_end_bytes: Option<Bytes> = None;

        // ----------------------------------- event dispatch ----------------------------------- //

        // there is 100% a better way to do this but it's 3am and i'll think of it later
        let ics_count = match ics {
            [false, false] => 0,
            [true, false] | [false, true] => 1,
            [true, true] => 2,
        };

        let mut event = EventType::None;
        // It's better to overallocate than to have to reallocate these vecs. The pre and post
        // should be oversize by a little bit when factoring in rollback'd frames.
        let mut pre_offsets = Vec::with_capacity(frame_count * (3 + ics_count));
        let mut post_offsets = Vec::with_capacity(frame_count * (3 + ics_count));
        let mut item_offsets = Vec::new();

        let mut pos = file_data.len() - stream.len();

        while pos < raw_length as usize && event != EventType::GameEnd {
            let code = stream.get_u8();
            event = EventType::from_repr(code).unwrap_or_default();
            /* EventType::None allows the parser to continue working on newer replays (with possible
            new events). During testing all events must be accounted for, so any EventType::Nones
            are likely a misalignment of my slices */
            debug_assert!(event != EventType::None);

            match event {
                EventType::PreFrame => pre_offsets.push(pos + 1),
                EventType::PostFrame => post_offsets.push(pos + 1),
                EventType::Item => item_offsets.push(pos + 1),
                EventType::GameEnd => {
                    let size = event_sizes[&event] as usize;
                    game_end_bytes = Some(stream.slice(..size))
                }
                _ => (),
            }
            stream.advance(event_sizes[&event] as usize);
            pos = file_data.len() - stream.len();
        }

        let mut game_end = None;

        if let Some(bytes) = game_end_bytes {
            game_end = Some(parse_gameend(bytes));
        }

        let frames_rollbacked = (pre_offsets.len() / (2 + ics_count)).saturating_sub(frame_count);

        let metadata = Arc::new(Metadata {
            version,
            start: game_start,
            end: game_end,
            duration,
                total_frames: frame_count,
                rolled_back_frames: Some(frames_rollbacked),
                path: Arc::new(path.to_owned()),
                date,
            });

            let mut item_frames = None;
            if version.at_least(3, 0, 0) {
                item_frames = Some(parse_itemframes(file_data.clone(), metadata.clone(), &item_offsets));
        }



        let (pre_frames, post_frames) = rayon::join(
            || {
                parse_preframes(
                    file_data.clone(),
                    metadata.clone(),
                    &pre_offsets,
                    ports,
                    ics,
                    [players[0].character, players[1].character],
                )
            },
            || {
                parse_postframes(
                    file_data.clone(),
                    metadata.clone(),
                    &post_offsets,
                    ports,
                    ics,
                )
            },
        );

        let mut pre_f = pre_frames?;
        let mut post_f = post_frames?;

        for player in players.iter_mut() {
            let temp_pre = pre_f.remove(&(player.port as u8)).unwrap();
            player.frames.pre = Arc::new(temp_pre.0);

            let temp_post = post_f.remove(&(player.port as u8)).unwrap();
            player.frames.post = Arc::new(temp_post.0);
            if temp_pre.1.is_some() {
                player.nana_frames = Some(Frames {
                    pre: Arc::new(temp_pre.1.unwrap()),
                    post: Arc::new(temp_post.1.unwrap()),
                })
            }
        }

        Ok(Game {
            metadata,
            players: players.map(Arc::new),
            item_frames: item_frames.map(Arc::new),
        })
    }

    pub fn stub(path: &Path) -> Result<GameStub> {
        // TODO yeah yeah eventually i should extract this into a function instead of duplicating.
        // I'll get around to it eventually,it's gonna take a bit of fiddling to make sure all the
        // bytes objects are created/updated correctly and I just don't want to deal with it atm.

        // default buffer capacity is more than 8x what we need, and read time is at a premium.
        // we only need the gamestart, game end, and metadata events
        let mut stream = BufReader::with_capacity(1000, File::open(path)?);

        let mut buf = [0; 11];
        stream.read_exact(&mut buf).unwrap();
        assert_eq!(
            buf,
            [0x7b, 0x55, 0x03, 0x72, 0x61, 0x77, 0x5b, 0x24, 0x55, 0x23, 0x6c,],
        );

        let raw_length = stream.read_u32::<BigEndian>().unwrap() as u64 + 15;

        let header_offset = stream.stream_position().unwrap();
        // ------------------------------------- game start ------------------------------------- //
        assert_eq!(stream.read_u8().unwrap(), 0x35);
        let payloads_size = stream.read_u8().unwrap();

        let mut start_len = 0;
        for _ in (0..(payloads_size - 1)).step_by(3) {
            let event = EventType::from_repr(stream.read_u8().unwrap()).unwrap();
            if event == EventType::GameStart {
                start_len = stream.read_u16::<BigEndian>().unwrap();
                break;
            }
        }

        stream
            .seek(SeekFrom::Start(header_offset + payloads_size as u64 + 1))
            .unwrap();

        assert_eq!(stream.read_u8().unwrap(), EventType::GameStart as u8);

        let mut buf = vec![0; start_len.into()];
        stream.read_exact(&mut buf).unwrap();

        let raw_start = Bytes::from(buf);

        let (game_start, version, players) = GameStart::parse(raw_start)?;

        stream.seek(SeekFrom::Start(raw_length)).unwrap();

        let mut metadata_header = [0; 11];
        stream.read_exact(&mut metadata_header).unwrap();
        assert_eq!(
            metadata_header,
            // `metadata` key & type ("U\x08metadata{")
            [0x55, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x7b,],
        );

        let mut metadata_block = Vec::new();
        stream.read_to_end(&mut metadata_block).unwrap();

        let mut duration: Duration = Duration::default();
        let metadata = ubjson::to_map(&mut metadata_block.reader())?;
        let mut total_frames: usize = 0;

        if let serde_json::Value::Number(lastframe) = &metadata["lastFrame"] {
            // duration, in frames, is translated to seconds. 123 is subtracted from the frame count
            // to match the duration to the in-game timer.
            let last = lastframe.as_i64().unwrap();
            total_frames = (last + 123) as usize;
            let millis = ((last.max(0) as f32 / 60.0) * 1000.0) as u64;

            // i shouldn't have to do any checks on this conversion
            duration = Duration::from_millis(millis);
        };

        let mut date = OffsetDateTime::UNIX_EPOCH;
        if let serde_json::Value::String(start_at) = &metadata["startAt"] {
            date = OffsetDateTime::parse(start_at.as_str(), &Iso8601::DEFAULT)
                .unwrap_or(OffsetDateTime::UNIX_EPOCH);
        }

        Ok(GameStub {
            metadata: Arc::new(Metadata {
                version,
                start: game_start,
                end: None,
                duration,
                total_frames,
                rolled_back_frames: None,
                path: Arc::new(path.to_owned()),
                date,
            }),
            players: players
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }
}
