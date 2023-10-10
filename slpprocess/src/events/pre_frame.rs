#![allow(clippy::uninit_vec)]

use crate::{events::game_start::Version, Port};
use bytes::{Buf, Bytes};
use crossbeam::channel::{Receiver, Sender};
use nohash_hasher::IntMap;
use polars::prelude::*;
use ssbm_utils::types::{Position, StickPos};

#[derive(Debug, Default, Clone)]
pub struct PreFrames {
    len: usize,
    pub version: Version,
    pub frame_index: Box<[i32]>,
    pub random_seed: Box<[u32]>,
    pub action_state: Box<[u16]>,
    pub position: Box<[Position]>,
    pub orientation: Box<[f32]>,
    pub joystick: Box<[StickPos]>,
    pub cstick: Box<[StickPos]>,
    pub engine_trigger: Box<[f32]>,
    pub engine_buttons: Box<[u32]>,
    pub controller_buttons: Box<[u16]>,
    pub controller_l: Box<[f32]>,
    pub controller_r: Box<[f32]>,
    pub percent: Option<Box<[f32]>>,
}

impl PreFrames {
    fn new(duration: usize, version: Version) -> Self {
        /* Because this is only used internally and only exists in this function, there's no real
        reason to 0-initialize the memory when we're immediately overwriting it anyway. Saves
        a fair few cycles */
        PreFrames {
            len: duration,
            version,
            frame_index: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            random_seed: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            action_state: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            position: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            orientation: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            joystick: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            cstick: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            engine_trigger: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            engine_buttons: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            controller_buttons: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            controller_l: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            controller_r: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                temp.into_boxed_slice()
            },
            percent: unsafe {
                let mut temp = Vec::with_capacity(duration);
                temp.set_len(duration);
                Some(temp.into_boxed_slice())
            },
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// When nana is dead, she is considered `inactive`, which is the variable checked by slippi to
    /// determine what characters to record frames for. As a result, we cannot rely on the same
    /// invariants as `new()` (that provide extra optimization room). Because nana can have less
    /// frames than pop, we can't get away with skipping 0-initialization, since we don't know if
    /// the whole array will be overwritten. There's also a few awkward points such as needing to
    /// initialize the frame counter to the correct values since we won't have an event with which
    /// to populate them for any frames where nana is dead.
    ///
    /// a (possibly) nice result of this is that, unlike other parsers, we can guarantee that nana
    /// frames (if they exist) will always be the same length as leader frames, even if some of the
    /// data is filled with dummy "null" values.
    fn ics(duration: usize, version: Version) -> Self {
        let len = (duration - 123) as i32;
        PreFrames {
            len: duration,
            version,
            frame_index: ((-123)..len).collect::<Vec<i32>>().into_boxed_slice(),
            random_seed: vec![0; duration].into_boxed_slice(),
            // Initialize to ActionState::Sleep, since that's what nana will be in when frames are
            // skipped
            action_state: vec![11; duration].into_boxed_slice(),
            // can't go wrong 0ing out most of these values
            position: vec![Position::default(); duration].into_boxed_slice(),
            orientation: vec![0.0; duration].into_boxed_slice(),
            joystick: vec![StickPos::default(); duration].into_boxed_slice(),
            cstick: vec![StickPos::default(); duration].into_boxed_slice(),
            engine_trigger: vec![0.0; duration].into_boxed_slice(),
            engine_buttons: vec![0; duration].into_boxed_slice(),
            controller_buttons: vec![0; duration].into_boxed_slice(),
            controller_l: vec![0.0; duration].into_boxed_slice(),
            controller_r: vec![0.0; duration].into_boxed_slice(),
            percent: Some(vec![0.0; duration].into_boxed_slice()),
        }
    }
}

#[allow(clippy::from_over_into)]
impl From<PreFrames> for DataFrame {
    fn from(val: PreFrames) -> DataFrame {
        let len = val.len();

        use crate::columns::Pre as col;
        let mut vec_series = vec![
            Series::new(col::FrameIndex.into(), val.frame_index),
            Series::new(col::RandomSeed.into(), val.random_seed),
            Series::new(col::ActionState.into(), val.action_state),
            // wow polars is ugly in rust
            StructChunked::new(
                col::Position.into(),
                &[
                    Series::new("x", val.position.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.position.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::Orientation.into(), val.orientation),
            StructChunked::new(
                col::JoystickPos.into(),
                &[
                    Series::new("x", val.joystick.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.joystick.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            StructChunked::new(
                col::CstickPos.into(),
                &[
                    Series::new("x", val.cstick.iter().map(|p| p.x).collect::<Vec<_>>()),
                    Series::new("y", val.cstick.iter().map(|p| p.y).collect::<Vec<_>>()),
                ],
            )
            .unwrap()
            .into_series(),
            Series::new(col::EngineTrigger.into(), val.engine_trigger),
            Series::new(col::EngineButtons.into(), val.engine_buttons),
            Series::new(col::ControllerButtons.into(), val.controller_buttons),
            Series::new(col::ControllerL.into(), val.controller_l),
            Series::new(col::ControllerR.into(), val.controller_r),
        ];
        if val.version.at_least(1, 4, 0) {
            vec_series.push(Series::new(col::Percent.into(), val.percent.unwrap()));
        } else {
            vec_series.push(Series::new_null(col::Percent.into(), len));
        }

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_preframes(
    version: Version,
    frames: Receiver<Bytes>,
    // send_to: Sender<IntMap<u8, (PreFrames, Option<PreFrames>)>>,
    duration: u64,
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (PreFrames, Option<PreFrames>)> {
    let p_frames = {
        /* splitting these out saves us a small amount of time in conditional logic, and allows for
        exact iterator chunk sizes. */
        // if !ics[0] && !ics[1] {
        unpack_frames(frames, duration, ports, version)
        // } else {
        //     unpack_frames_ics(frames, duration, ports, ics, version)
        // }
    };

    let mut result = IntMap::default();

    for (port, (player_frames, nana_frames)) in p_frames {
        result.insert(port, (player_frames, nana_frames));
    }

    result
}

pub fn unpack_frames(
    frames: Receiver<Bytes>,
    duration: u64,
    ports: [Port; 2],
    version: Version,
) -> IntMap<u8, (PreFrames, Option<PreFrames>)> {
    let mut p_frames: IntMap<u8, (PreFrames, Option<PreFrames>)> = IntMap::default();

    p_frames.insert(
        ports[0] as u8,
        (PreFrames::new(duration as usize, version), None),
    );
    p_frames.insert(
        ports[1] as u8,
        (PreFrames::new(duration as usize, version), None),
    );

    while let Ok(mut frame) = frames.recv() {
        let frame_number = frame.get_i32();
        let i = (frame_number + 123) as usize;
        assert!(i < duration as usize);
        let port = frame.get_u8();
        frame.advance(1); // skip nana byte

        let (working, _) = p_frames.get_mut(&port).unwrap();
        // if the compiler doesn't catch that these are in-bounds, it's still fairly obvious.
        // i has to be 0..frames_iter.len(), and that length was used to construct all of the
        // vecs that make up the PreFrames objects.
        unsafe {
            *working.frame_index.get_unchecked_mut(i) = frame_number;
            *working.random_seed.get_unchecked_mut(i) = frame.get_u32();
            *working.action_state.get_unchecked_mut(i) = frame.get_u16();
            *working.position.get_unchecked_mut(i) =
                Position::new(frame.get_f32(), frame.get_f32());
            *working.orientation.get_unchecked_mut(i) = frame.get_f32();
            *working.joystick.get_unchecked_mut(i) =
                StickPos::new(frame.get_f32(), frame.get_f32());
            *working.cstick.get_unchecked_mut(i) = StickPos::new(frame.get_f32(), frame.get_f32());
            *working.engine_trigger.get_unchecked_mut(i) = frame.get_f32();
            *working.engine_buttons.get_unchecked_mut(i) = frame.get_u32();
            *working.controller_buttons.get_unchecked_mut(i) = frame.get_u16();
            *working.controller_l.get_unchecked_mut(i) = frame.get_f32();
            *working.controller_r.get_unchecked_mut(i) = frame.get_f32();
            if !frame.has_remaining() {
                // version < 1.2.0
                continue;
            }
            frame.advance(1);
            if !frame.has_remaining() {
                // version < 1.4.0
                continue;
            }
            *working.percent.as_mut().unwrap().get_unchecked_mut(i) = frame.get_f32();
        }
    }

    p_frames
}

// pub fn unpack_frames_ics(
//     frames: &mut [Bytes],
//     duration: u64,
//     ports: [Port; 2],
//     ics: [bool; 2],
//     version: Version,
// ) -> IntMap<u8, (PreFrames, Option<PreFrames>)> {
//     let len = duration as usize;

//     let mut p_frames: IntMap<u8, (PreFrames, Option<PreFrames>)> = IntMap::default();
//     p_frames.insert(
//         ports[0] as u8,
//         (
//             PreFrames::new(len, version),
//             ics[0].then(|| PreFrames::ics(len, version)),
//         ),
//     );
//     p_frames.insert(
//         ports[1] as u8,
//         (
//             PreFrames::new(len, version),
//             ics[1].then(|| PreFrames::ics(len, version)),
//         ),
//     );

//     for frame in frames.iter_mut() {
//         let frame_number = frame.get_i32();
//         let i = (frame_number + 123) as usize;
//         assert!(
//             i < len,
//             "Frame index incorrect, index ({i}) is greater than or equal to the max length of the container ({len})."
//         );
//         let port = frame.get_u8();
//         let nana = frame.get_u8() != 0;

//         let working = {
//             let temp = p_frames.get_mut(&port).unwrap();
//             if nana {
//                 temp.1.as_mut().unwrap()
//             } else {
//                 &mut temp.0
//             }
//         };

//         unsafe {
//             *working.frame_index.get_unchecked_mut(i) = frame_number;
//             *working.random_seed.get_unchecked_mut(i) = frame.get_u32();
//             *working.action_state.get_unchecked_mut(i) = frame.get_u16();
//             *working.position.get_unchecked_mut(i) =
//                 Position::new(frame.get_f32(), frame.get_f32());
//             *working.orientation.get_unchecked_mut(i) = frame.get_f32();
//             *working.joystick.get_unchecked_mut(i) =
//                     StickPos::new(frame.get_f32(), frame.get_f32());
//                 *working.cstick.get_unchecked_mut(i) =
//                     StickPos::new(frame.get_f32(), frame.get_f32());
//             *working.engine_trigger.get_unchecked_mut(i) = frame.get_f32();
//             *working.engine_buttons.get_unchecked_mut(i) = frame.get_u32();
//             *working.controller_buttons.get_unchecked_mut(i) = frame.get_u16();
//             *working.controller_l.get_unchecked_mut(i) = frame.get_f32();
//             *working.controller_r.get_unchecked_mut(i) = frame.get_f32();
//             if !frame.has_remaining() {
//                 // version < 1.2.0
//                 continue;
//             }
//             frame.advance(1);
//             if !frame.has_remaining() {
//                 // version < 1.4.0
//                 continue;
//             } else {
//                 *working.percent.as_mut().unwrap().get_unchecked_mut(i) = frame.get_f32();
//             }
//         }
//     }

//     p_frames
// }
