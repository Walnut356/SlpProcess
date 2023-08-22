#![allow(non_upper_case_globals)]

use anyhow::{anyhow, ensure, Result};
use byteorder::{BigEndian, ReadBytesExt};
use bytes::Bytes;
use nohash_hasher::IntMap;
use num_enum::{FromPrimitive, IntoPrimitive};
use polars::prelude::*;

use std::fs::File;
use std::io::{prelude::*, Cursor};
use std::path::Path;
use std::sync::RwLock;
use std::time::Duration;

use crate::enums::character::Character;
use crate::events::game_end::parse_gameend;
use crate::events::item::parse_itemframes;
use crate::player::Frames;
use crate::{
    events::{
        game_start::GameStart,
        post_frame::parse_postframes,
        pre_frame::parse_preframes,
    },
    utils::ParseError,
};
use crate::{ubjson, Game};

trait AsFrames {
    fn as_frames(&self) -> u64;
}

impl AsFrames for Duration {
    fn as_frames(&self) -> u64 {
        (*self / 60).as_secs()
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, IntoPrimitive)]
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

fn expect_bytes<R: Read>(stream: &mut R, expected: &[u8]) -> std::io::Result<()> {
    let mut actual = vec![0; expected.len()];
    stream.read_exact(&mut actual)?;
    if expected == actual.as_slice() {
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
        let mut file_data = vec![0; file_length];
        f.read_exact(&mut file_data).unwrap();

        Ok(Bytes::from(file_data))
    }

        fn get_event_sizes<R>(file: &mut R) -> Result<IntMap<u8, u16>>
    where
        R: Read,
    {
        let code = EventType::from_primitive(file.read_u8().unwrap());
        ensure!(
            code == EventType::EventPayloads,
            ParseError::Value(
                format!("{:?}", EventType::EventPayloads),
                format!("{:?}", code)
            )
        );

        let payloads_size = file.read_u8().unwrap();

        ensure!(
            (payloads_size - 1) % 3 == 0,
            anyhow!("EventPayloads length invalid")
        );

        let mut event_map: IntMap<u8, u16> = IntMap::default();

        for _ in (0..(payloads_size - 1)).step_by(3) {
            let event = EventType::from(file.read_u8().unwrap());
            let size = file.read_u16::<BigEndian>().unwrap();
            event_map.insert(event.into(), size);
        }

        Ok(event_map)
    }

    /// Accepts a tokio Bytes object, returns a Game object. Useful if you already have the file in
    /// memory for some other reason
    pub fn parse(file_data: Bytes) -> Result<Self> {
        // ---------------------------------------- setup --------------------------------------- //
        // todo replace this with another bytes object? Bytes comes with an internal cursor
        // that supports .advance()
        let mut stream = Cursor::new(file_data);

        expect_bytes(
            &mut stream,
            &[
                0x7b, 0x55, 0x03, 0x72, 0x61, 0x77, 0x5b, 0x24, 0x55, 0x23, 0x6c,
            ],
        )?;

        let raw_length = stream.read_u32::<BigEndian>().unwrap() as u64 + 15;

        let event_sizes: IntMap<u8, u16> = Self::get_event_sizes(&mut stream)?;

        // ------------------------------------- game start ------------------------------------- //

        assert_eq!(stream.read_u8().unwrap(), EventType::GameStart as u8);

        let raw_start = stream.get_ref().slice(
            // wow this is exceptionally ugly! thanks rust =)
            stream.position() as usize
                ..(stream.position() + event_sizes[&EventType::GameStart.into()] as u64) as usize,
        );

        let (game_start, version, mut players) = GameStart::parse(raw_start);

        stream.set_position(stream.position() + event_sizes[&EventType::GameStart.into()] as u64);

        let mut game_end_bytes: Option<Bytes> = None;

        // ----------------------------------- event dispatch ----------------------------------- //

        let mut event = EventType::None;
        let mut pre_bytes = Vec::new();
        let mut post_bytes = Vec::new();
        let mut item_bytes = Vec::new();

        while stream.position() < raw_length && event != EventType::GameEnd {
            let code = stream.read_u8().unwrap();
            event = EventType::from(code);
            // TODO remove this once everything works
            /* EventType::None allows the parser to continue working on newer replays (with possible
            new events). During testing all events are accounted for, so any EventType::Nones are
            likely a misalignment of my slices */
            assert!(event != EventType::None);

            let size = event_sizes[&code] as u64;
            let pos = stream.position();
            let raw_data = stream.get_ref();

            match event {
                EventType::PreFrame => {
                    pre_bytes.push(raw_data.slice(pos as usize..(pos + size) as usize))
                }
                EventType::PostFrame => {
                    post_bytes.push(raw_data.slice(pos as usize..(pos + size) as usize))
                }
                EventType::Item => {
                    item_bytes.push(raw_data.slice(pos as usize..(pos + size) as usize))
                }
                EventType::GameEnd => {
                    game_end_bytes = Some(raw_data.slice(pos as usize..(pos + size) as usize))
                }
                _ => (),
            }

            stream.set_position(pos + size);
        }

        expect_bytes(
            &mut stream,
            // `metadata` key & type ("U\x08metadata{")
            &[
                0x55, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x7b,
            ],
        )?;

        let mut frame_count = 0;

        let mut duration: Duration = Duration::default();
        let metadata = ubjson::to_map(&mut stream)?;
        if let serde_json::Value::Number(lastframe) = &metadata["lastFrame"] {
            // duration, in frames, is translated to seconds. 123 is subtracted from the frame count
            // to match the duration to the in-game timer. The total frame count is easily
            // found from player.frames.len()
            let last = lastframe.as_u64().unwrap();
            frame_count = last + 124;
            let millis = ((last.max(0) as f32 / 60.0) * 1000.0) as u64;

            // i shouldn't have to do any checks on this conversion
            duration = Duration::from_millis(millis);
        }

        let mut game_end = None;

        if let Some(bytes) = game_end_bytes {
            game_end = Some(parse_gameend(bytes));
        }

        // i could map but this gives me arrays instead of slices without into
        let ports = [players[0].port, players[1].port];
        let ics = [
            players[0].character == Character::IceClimbers,
            players[1].character == Character::IceClimbers,
        ];

        let item_frames = parse_itemframes(&mut item_bytes);

        let (mut pre_frames, mut post_frames) = rayon::join(
            || parse_preframes(&mut pre_bytes, frame_count as u64, ports, ics),
            || parse_postframes(&mut post_bytes, frame_count as u64, ports, ics),
        );

        for player in players.iter_mut() {
            let temp_pre = pre_frames.remove(&player.port.into()).unwrap();
            player.frames.pre = temp_pre.0;

            let temp_post = post_frames.remove(&player.port.into()).unwrap();
            player.frames.post = temp_post.0;
            if temp_pre.1.is_some() {
                player.nana_frames = Some(Frames {
                    pre: temp_pre.1.unwrap(),
                    post: temp_post.1.unwrap(),
                })
            }
        }

        Ok(Game {
            start: game_start,
            end: game_end,
            duration,
            total_frames: frame_count,
            players: players.map(|x| Arc::new(RwLock::new(x))),
            version,
            item_frames,
        })
    }
}
