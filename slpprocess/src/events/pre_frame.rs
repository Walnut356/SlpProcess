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
    facing: Box<[f32]>,
    joystick_x: Box<[f32]>,
    joystick_y: Box<[f32]>,
    cstick_x: Box<[f32]>,
    cstick_y: Box<[f32]>,
    trigger: Box<[f32]>,
    logical_buttons: Box<[u32]>,
    physical_buttons: Box<[u16]>,
    physical_l: Box<[f32]>,
    physical_r: Box<[f32]>,
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
            facing: unsafe {
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
            trigger: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            logical_buttons: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            physical_buttons: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            physical_l: unsafe {
                let mut temp = Vec::with_capacity(len);
                temp.set_len(len);
                temp.into_boxed_slice()
            },
            physical_r: unsafe {
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
}
// preframes objects are purely a temporary container to make the code clearer, so i impl Into
// rather than From (and implicitly Into) because I intentionally want to disallow translation back.
#[allow(clippy::from_over_into)]
impl Into<DataFrame> for PreFrames {
    fn into(self) -> DataFrame {
        let vec_series = vec![
            Series::new("frame number", self.frame_number),
            Series::new("random seed", self.random_seed),
            Series::new("action state", self.action_state),
            Series::new("position x", self.position_x),
            Series::new("position y", self.position_y),
            Series::new("facing", self.facing),
            Series::new("joystick x", self.joystick_x),
            Series::new("joystick y", self.joystick_y),
            Series::new("cstick x", self.cstick_x),
            Series::new("cstick y", self.cstick_y),
            Series::new("trigger", self.trigger),
            Series::new("logical buttons", self.logical_buttons),
            Series::new("physical buttons", self.physical_buttons),
            Series::new("physical l", self.physical_l),
            Series::new("physical r", self.physical_r),
            Series::new("percent", self.percent),
        ];

        DataFrame::new(vec_series).unwrap()
    }
}

pub fn parse_preframes(
    frames: &mut [Bytes],
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (DataFrame, Option<DataFrame>)> {
    let p_frames = {
        /* splitting these out saves us a small amount of time in conditional logic, and allows for
        exact iterator chunk sizes. */
        if !ics[0] && !ics[1] {
            no_ics(frames, ports)
        } else if ics[0] ^ ics[1] {
            one_ics(frames, ports, ics)
        } else {
            two_ics(frames, ports)
        }
    };

    let mut result = IntMap::default();

    for (port, (player_frames, nana_frames)) in p_frames {
        result.insert(port, (player_frames.into(), nana_frames.map(|x| x.into())));
    }

    result
}

pub fn no_ics(
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
                *working.facing.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_y.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_y.get_unchecked_mut(i) = frame.get_f32();
                *working.trigger.get_unchecked_mut(i) = frame.get_f32();
                *working.logical_buttons.get_unchecked_mut(i) = frame.get_u32();
                *working.physical_buttons.get_unchecked_mut(i) = frame.get_u16();
                *working.physical_l.get_unchecked_mut(i) = frame.get_f32();
                *working.physical_r.get_unchecked_mut(i) = frame.get_f32();
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

pub fn one_ics(
    frames: &mut [Bytes],
    ports: [Port; 2],
    ics: [bool; 2],
) -> IntMap<u8, (PreFrames, Option<PreFrames>)> {
    /* TODO defining it like this *should* eliminate bounds checks, but i need to inspect the
    assembly to be sure. It's gonna start looking real gross if it's having trouble seeing through
    the constructor though */

    let frames_iter = frames.chunks_exact_mut(3).enumerate();
    let len = frames_iter.len();

    let mut p_frames: IntMap<u8, (PreFrames, Option<PreFrames>)> = IntMap::default();
    p_frames.insert(
        ports[0].into(),
        (PreFrames::new(len), ics[0].then(|| PreFrames::new(len))),
    );
    p_frames.insert(
        ports[1].into(),
        (PreFrames::new(len), ics[1].then(|| PreFrames::new(len))),
    );

    for (i, frames_raw) in frames_iter {
        for frame in frames_raw {
            let frame_number = frame.get_i32();
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

            // if the compiler doesn't catch that these are in-bounds, it's still fairly obvious.
            // i has to be 0..frames_iter.len(), and that length was used to construct all of the
            // vecs that make up the PreFrames objects.
            unsafe {
                *working.frame_number.get_unchecked_mut(i) = frame_number;
                *working.random_seed.get_unchecked_mut(i) = frame.get_u32();
                *working.action_state.get_unchecked_mut(i) = frame.get_u16();
                *working.position_x.get_unchecked_mut(i) = frame.get_f32();
                *working.position_y.get_unchecked_mut(i) = frame.get_f32();
                *working.facing.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_y.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_y.get_unchecked_mut(i) = frame.get_f32();
                *working.trigger.get_unchecked_mut(i) = frame.get_f32();
                *working.logical_buttons.get_unchecked_mut(i) = frame.get_u32();
                *working.physical_buttons.get_unchecked_mut(i) = frame.get_u16();
                *working.physical_l.get_unchecked_mut(i) = frame.get_f32();
                *working.physical_r.get_unchecked_mut(i) = frame.get_f32();
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
    }

    p_frames
}

/// Frame ordering on a completed replay is guaranteed to be in port order, with that port's nana
/// frames directly following the leader frames
///
/// e.g.
/// * port 1: leader
/// * port 1: nana
/// * port 2: leader
/// * port 2: nana
pub fn two_ics(
    frames: &mut [Bytes],
    ports: [Port; 2],
) -> IntMap<u8, (PreFrames, Option<PreFrames>)> {
    /* TODO defining it like this *should* eliminate bounds checks, but i need to inspect the
    assembly to be sure. It's gonna start looking real gross if it's having trouble seeing through
    the constructor though */

    let frames_iter = frames.chunks_exact_mut(4).enumerate();
    let len = frames_iter.len();

    let mut p_frames: IntMap<u8, (PreFrames, Option<PreFrames>)> = IntMap::default();
    p_frames.insert(
        ports[0].into(),
        (PreFrames::new(len), Some(PreFrames::new(len))),
    );
    p_frames.insert(
        ports[1].into(),
        (PreFrames::new(len), Some(PreFrames::new(len))),
    );

    for (i, frames_raw) in frames_iter {
        for frame in frames_raw {
            let frame_number = frame.get_i32();
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

            // if the compiler doesn't catch that these are in-bounds, it's still fairly obvious.
            // i has to be 0..frames_iter.len(), and that length was used to construct all of the
            // vecs that make up the PreFrames objects.
            unsafe {
                // ----------------------------------- leader ----------------------------------- //
                *working.frame_number.get_unchecked_mut(i) = frame_number;
                *working.random_seed.get_unchecked_mut(i) = frame.get_u32();
                *working.action_state.get_unchecked_mut(i) = frame.get_u16();
                *working.position_x.get_unchecked_mut(i) = frame.get_f32();
                *working.position_y.get_unchecked_mut(i) = frame.get_f32();
                *working.facing.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.joystick_y.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_x.get_unchecked_mut(i) = frame.get_f32();
                *working.cstick_y.get_unchecked_mut(i) = frame.get_f32();
                *working.trigger.get_unchecked_mut(i) = frame.get_f32();
                *working.logical_buttons.get_unchecked_mut(i) = frame.get_u32();
                *working.physical_buttons.get_unchecked_mut(i) = frame.get_u16();
                *working.physical_l.get_unchecked_mut(i) = frame.get_f32();
                *working.physical_r.get_unchecked_mut(i) = frame.get_f32();
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
