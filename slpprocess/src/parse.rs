use byteorder::{BigEndian, ReadBytesExt};
use nohash_hasher::IntMap;
use num_enum::FromPrimitive;
use rayon::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};
use std::path::Path;
use std::slice;
use std::thread;

use crate::event::GameStart;

// #[derive(Debug, Copy, Clone)]
// struct PreFrame {
//     frame_index: i32,
//     player: u8,
//     follower: u8,
//     random_seed: u32,
//     pos_x: f32,
//     pos_y: f32,
//     facing_direction: f32,
//     joystick_x: f32,
//     joystick_y: f32,
//     cstick_x: f32,
//     cstick_y: f32,
//     trigger: f32,
//     processed_buttons: u32,
//     physical_buttons: u32,
//     physical_l: f32,
//     physical_r: f32,
//     x_analog: i8,
//     percent: f32,
// }

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq)]
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

fn expect_bytes<R: Read>(r: &mut R, expected: &[u8]) -> std::io::Result<()> {
    let mut actual = vec![0; expected.len()];
    r.read_exact(&mut actual)?;
    if expected == actual.as_slice() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "got {actual}, expected {expected}",
        ))
    }
}

fn get_event_sizes<R>(file: &mut R, bytes_read: &mut u32) -> IntMap<u8, u16>
where
    R: Read,
{
    let code = EventType::from_primitive(file.read_u8().unwrap());
    if code != EventType::EventPayloads {
        panic!("expected EventPayloads, got {code:?}")
    };
    let payloads_size = file.read_u8().unwrap();

    if (payloads_size - 1) % 3 != 0 {
        panic!("invalid length for EventPayloads Event")
    };

    let mut event_map: IntMap<u8, u16> = IntMap::default();

    for _ in (0..(payloads_size - 1)).step_by(3) {
        event_map.insert(
            file.read_u8().unwrap(),
            file.read_u16::<BigEndian>().unwrap(),
        );
    }

    *bytes_read += payloads_size as u32 + 1;

    return event_map;
}

pub fn parse(path: &String) -> Vec<Vec<u8>> {
    let p = Path::new(path);
    let mut f = File::open(p).unwrap();
    let mut stream = BufReader::with_capacity(f.metadata().unwrap().len() as usize, f);

    let mut bytes_read: u32 = 0;

    expect_bytes(
        &mut stream,
        &[
            0x7b, 0x55, 0x03, 0x72, 0x61, 0x77, 0x5b, 0x24, 0x55, 0x23, 0x6c,
        ],
    )
    .unwrap();
    bytes_read += 11;

    let raw_length = stream.read_u32::<BigEndian>().unwrap();
    bytes_read += 4;

    let event_sizes = get_event_sizes(&mut stream, &mut bytes_read);
    let mut event = EventType::None;
    let mut event_dispatch = Vec::new();

    let mut game_start: GameStart;

    if stream.read_u8().unwrap() == EventType::GameStart as u8 {
        game_start = GameStart::new(
            stream.buffer().as_ptr(),
            event_sizes[&(EventType::GameStart as u8)] as usize,
        );
    }
    bytes_read += event_sizes[&(EventType::GameStart as u8)] as u32;

    // update stream position, need to subtract 1 from event size length due to command byte
    stream
        .seek_relative((event_sizes[&(EventType::GameStart as u8)]) as i64)
        .unwrap();

    while bytes_read < raw_length && event != EventType::GameEnd {
        let code = stream.read_u8().unwrap();
        bytes_read += 1;

        event = EventType::from(code);
        let size = event_sizes[&code];

        event_dispatch.push((code, bytes_read as isize, size as usize));

        //BufReader.buffer().as_ptr() returns a pointer to the *current location* within the buffer for some reason,
        // not to the start of the buffer. It's more convenient for my purposes but still weird.

        // event_dispatch.push((code, stream.buffer().as_ptr(), size as usize));

        bytes_read += size as u32;
        stream.seek_relative(size as i64).unwrap();
    }

    // -------------------------------------------------- Rayon Map ------------------------------------------------- //

    // rayon par_iter doesn't like raw pointers, so it's necessary to use the bytes_read and set the stream position
    // back to the start.

    stream.seek(SeekFrom::Start(0)).unwrap();

    let thing: Vec<Vec<u8>> = event_dispatch
        .par_iter()
        .map(|f| {
            let code = f.0;
            let pos = f.1;
            let size = f.2;

            let start = unsafe { stream.buffer().as_ptr().offset(pos) };

            let slice = unsafe { slice::from_raw_parts(start, size).clone() };
            let mut vec = Vec::with_capacity(size);
            vec.resize(size, 0);
            vec[..].clone_from_slice(slice);
            vec
        })
        .collect();

    // ------------------------------------------------ Regular Loop ------------------------------------------------ //

    // let mut thing = Vec::new();

    // for (code, size, pos) in event_dispatch {
    //     let start = unsafe { stream.buffer().as_ptr().offset(pos) };

    //     let slice = unsafe { slice::from_raw_parts(start, size).clone() };

    //     thing.push(Vec::from_iter(slice));
    // }

    // ------------------------------------------------- Regular Map ------------------------------------------------ //

    // let thing: Vec<Vec<u8>> = event_dispatch
    //     .iter()
    //     .map(|f| {
    //         let code = f.0;
    //         let ptr = f.1;
    //         let size = f.2;

    //         let slice = unsafe { slice::from_raw_parts(ptr, size) };
    //         let mut vec = Vec::with_capacity(size);
    //         vec.resize(size, 0);
    //         vec[..].clone_from_slice(slice);
    //         vec
    //     })
    //     .collect();
    thing
}
