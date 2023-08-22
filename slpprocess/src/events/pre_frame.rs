#![allow(clippy::uninit_vec)]

use crate::Port;
use bytes::{Buf, Bytes};
use nohash_hasher::IntMap;
use polars::prelude::*;

#[derive(Debug)]
pub struct PreFrames {
    frame_number: Box<[i32]>,
    random_seed: Box<[u32]>,
    action_state: Box<[u16]>,
    position_x: Box<[f32]>,
    position_y: Box<[f32]>,
    orientation: Box<[f32]>,
    joystick_x: Box<[f32]>,
    joystick_y: Box<[f32]>,
    cstick_x: Box<[f32]>,
    cstick_y: Box<[f32]>,
    engine_trigger: Box<[f32]>,
    engine_buttons: Box<[u32]>,
    controller_buttons: Box<[u16]>,
    controller_l: Box<[f32]>,
    controller_r: Box<[f32]>,
    percent: Box<[Option<f32>]>,
}

impl PreFrames {
    fn new(len: usize) -> Self {
        /* Because this is only used internally and only exists in this function, there's no real
        reason to 0-initialize the memory when we're immediately overwriting it anyway. Saves
        a fair few cycles */
        PreFrames {
            frame_number: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            random_seed: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            action_state: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            position_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            position_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            orientation: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            joystick_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            joystick_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            cstick_x: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            cstick_y: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            engine_trigger: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            engine_buttons: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            controller_buttons: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            controller_l: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            controller_r: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            percent: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
        }
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
    fn ics(duration: usize) -> Self {
        let len = (duration - 123) as i32;
        PreFrames {
            frame_number: ((-123)..len).collect::<Vec<i32>>().into_boxed_slice(),
            random_seed: vec![0; duration].into_boxed_slice(),
            // Initialize to ActionState::Sleep, since that's what nana will be in when frames are
            // skipped
            action_state: vec![11; duration].into_boxed_slice(),
            // can't go wrong 0ing out most of these values
            position_x: vec![0.0; duration].into_boxed_slice(),
            position_y: vec![0.0; duration].into_boxed_slice(),
            orientation: vec![0.0; duration].into_boxed_slice(),
            joystick_x: vec![0.0; duration].into_boxed_slice(),
            joystick_y: vec![0.0; duration].into_boxed_slice(),
            cstick_x: vec![0.0; duration].into_boxed_slice(),
            cstick_y: vec![0.0; duration].into_boxed_slice(),
            engine_trigger: vec![0.0; duration].into_boxed_slice(),
            engine_buttons: vec![0; duration].into_boxed_slice(),
            controller_buttons: vec![0; duration].into_boxed_slice(),
            controller_l: vec![0.0; duration].into_boxed_slice(),
            controller_r: vec![0.0; duration].into_boxed_slice(),
            percent: vec![None; duration].into_boxed_slice(),
        }
    }
}

// preframes objects are purely a temporary container to make the code clearer, so i impl Into
// rather than From (and implicitly Into) because I intentionally want to disallow translation back.
#[allow(clippy::from_over_into)]
impl Into<DataFrame> for PreFrames {
    fn into(self) -> DataFrame {
        use crate::columns::Pre::*;
        let vec_series = vec![
            Series::new(&FrameNumber.to_string(), self.frame_number),
            Series::new(&RandomSeed.to_string(), self.random_seed),
            Series::new(&ActionState.to_string(), self.action_state),
            Series::new(&PositionX.to_string(), self.position_x),
            Series::new(&PositionY.to_string(), self.position_y),
            Series::new(&Orientation.to_string(), self.orientation),
            Series::new(&JoystickX.to_string(), self.joystick_x),
            Series::new(&JoystickY.to_string(), self.joystick_y),
            Series::new(&CstickX.to_string(), self.cstick_x),
            Series::new(&CstickY.to_string(), self.cstick_y),
            Series::new(&EngineTrigger.to_string(), self.engine_trigger),
            Series::new(&EngineButtons.to_string(), self.engine_buttons),
            Series::new(&ControllerButtons.to_string(), self.controller_buttons),
            Series::new(&ControllerL.to_string(), self.controller_l),
            Series::new(&ControllerR.to_string(), self.controller_r),
            Series::new(&Percent.to_string(), self.percent),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_preframes(
    frames: &mut [Bytes],
    duration: u64,
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (DataFrame, Option<DataFrame>)> {
    let p_frames = {
        /* splitting these out saves us a small amount of time in conditional logic, and allows for
        exact iterator chunk sizes. */
        if !ics[0] && !ics[1] {
            unpack_frames(frames, ports)
        } else {
            unpack_frames_ics(frames, duration, ports, ics)
        }
    };

    let mut result = IntMap::default();

    for (port, (player_frames, nana_frames)) in p_frames {
        result.insert(port, (player_frames.into(), nana_frames.map(|x| x.into())));
    }

    result
}

pub fn unpack_frames(
    frames: &mut [Bytes],
    ports: [Port; 2],
) -> IntMap<u8, (PreFrames, Option<PreFrames>)> {
    /* TODO defining it like this *should* eliminate bounds checks, but i need to inspect the
    assembly to be sure. It's gonna start looking real gross if it's having trouble seeing through
    the constructor though */

    let frames_iter = frames.chunks_exact_mut(2).enumerate();
    let len = frames_iter.len();

    let mut p_frames: IntMap<u8, (PreFrames, Option<PreFrames>)> = IntMap::default();
    p_frames.insert(ports[0].into(), (PreFrames::new(len), None));
    p_frames.insert(ports[1].into(), (PreFrames::new(len), None));

    for (i, frames_raw) in frames_iter {
        for frame in frames_raw {
            let frame_number = frame.get_i32();
            let port = frame.get_u8();
            frame.advance(1); // skip nana byte

            let (working, _) = p_frames.get_mut(&port).unwrap();
            // if the compiler doesn't catch that these are in-bounds, it's still fairly obvious.
            // i has to be 0..frames_iter.len(), and that length was used to construct all of the
            // vecs that make up the PreFrames objects.
            unsafe {
                *working.frame_number.get_unchecked_mut(i) = frame_number;
                *working.random_seed.get_unchecked_mut(i) = frame.get_u32();
                *working.action_state.get_unchecked_mut(i) = frame.get_u16();
                *working.position_x.get_unchecked_mut(i) = frame.get_f32();
                *working.position_y.get_unchecked_mut(i) = frame.get_f32();
                *working.orientation.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_y.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_y.get_unchecked_mut(i) = frame.get_f32();
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
                *working.percent.get_unchecked_mut(i) = Some(frame.get_f32());
            }
        }
    }

    p_frames
}

pub fn unpack_frames_ics(
    frames: &mut [Bytes],
    duration: u64,
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (PreFrames, Option<PreFrames>)> {
    let len = duration as usize;

    let mut p_frames: IntMap<u8, (PreFrames, Option<PreFrames>)> = IntMap::default();
    p_frames.insert(
        ports[0].into(),
        (PreFrames::new(len), ics[0].then(|| PreFrames::ics(len))),
    );
    p_frames.insert(
        ports[1].into(),
        (PreFrames::new(len), ics[1].then(|| PreFrames::ics(len))),
    );

    for frame in frames.iter_mut() {
        let frame_number = frame.get_i32();
        let i = (frame_number + 123) as usize;
        assert!(
            i < len,
            "Frame index incorrect, index ({i}) is greater than or equal to the max length of the container ({len})."
        );
        let port = frame.get_u8();
        let nana = frame.get_u8() != 0;

        let working = {
            let temp = p_frames.get_mut(&port).unwrap();
            if nana {
                temp.1.as_mut().unwrap()
            } else {
                &mut temp.0
            }
        };

        unsafe {
            *working.frame_number.get_unchecked_mut(i) = frame_number;
            *working.random_seed.get_unchecked_mut(i) = frame.get_u32();
            *working.action_state.get_unchecked_mut(i) = frame.get_u16();
            *working.position_x.get_unchecked_mut(i) = frame.get_f32();
            *working.position_y.get_unchecked_mut(i) = frame.get_f32();
            *working.orientation.get_unchecked_mut(i) = frame.get_f32();
            *working.joystick_x.get_unchecked_mut(i) = frame.get_f32();
            *working.joystick_y.get_unchecked_mut(i) = frame.get_f32();
            *working.cstick_x.get_unchecked_mut(i) = frame.get_f32();
            *working.cstick_y.get_unchecked_mut(i) = frame.get_f32();
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
                *working.percent.get_unchecked_mut(i) = None;
            } else {
                *working.percent.get_unchecked_mut(i) = Some(frame.get_f32());
            }
        }
    }

    p_frames
}
