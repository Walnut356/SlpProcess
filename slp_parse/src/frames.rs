use std::sync::Arc;

use ssbm_utils::enums::{ActionRange as AR, ActionState, BitFlags, EngineInput, Flags};

use crate::events::{
    post_frame::{PostFrames, PostRow},
    pre_frame::{PreFrames, PreRow},
};

/// Container for Pre-frame and Post-frame containers.
///
/// Note that frames are stored in columnar format, meaning data access is as follows:
/// `player.frames.post.acion_state[index]`
///
/// `.get_frame(index)` functions exist for `Frames`, `PreFrames`, and `PostFrames` objects, but
/// these will generally be much slower than iterating through only the columns you need.
#[derive(Debug, Default, Clone)]
pub struct Frames {
    pub pre: Arc<PreFrames>,
    pub post: Arc<PostFrames>,
}

impl Frames {
    #[inline]
    pub fn len(&self) -> usize {
        self.pre.frame_index.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    /// Gets both the full pre-frame and post-frame for a given frame index (0-indexed). This is very
    /// slow compared to iterating through only the columns you need.
    pub fn get_frame(&self, index: usize) -> Frame {
        Frame(self.pre.get_frame(index), self.post.get_frame(index))
    }

    pub fn get_last_frame(&self) -> Frame {
        Frame(
            self.pre.get_frame(self.len() - 1),
            self.post.get_frame(self.len() - 1),
        )
    }
}

#[derive(Default, PartialEq)]
pub struct Frame(pub PreRow, pub PostRow);

impl core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Frame(\n\t{:#?}\n\t{:#?})", self.0, self.1)
        // f.debug_tuple("Frame").field(&self.0).field(&self.1).finish()
    }
}
impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl Frames {
    #[inline]
    pub fn just_changed_state(&self, i: usize) -> bool {
        self.post.action_state[i] != self.post.action_state[i - 1]
    }

    #[inline]
    pub fn just_entered_state(&self, state: ActionState, i: usize) -> bool {
        self.post.action_state[i] == state && self.post.action_state[i - 1] != state
    }

    #[inline]
    pub fn just_exited_state(&self, state: ActionState, i: usize) -> bool {
        self.post.action_state[i] != state && self.post.action_state[i - 1] == state
    }

    #[inline]
    pub fn just_took_damage(&self, i: usize) -> bool {
        self.post.percent[i] > self.post.percent[i - 1]
    }

    #[inline]
    pub fn damage_taken(&self, i: usize) -> f32 {
        (self.post.percent[i] - self.post.percent[i - 1]).max(0.0)
    }

    #[inline]
    pub fn in_hitlag(&self, i: usize) -> bool {
        match &self.post.flags {
            Some(f) => Flags::HITLAG.intersects(f[i]),
            None => false,
        }
    }

    #[inline]
    pub fn in_defender_hitlag(&self, i: usize) -> bool {
        match &self.post.flags {
            Some(f) => Flags::DEFENDER_HITLAG.intersects(f[i]),
            None => false,
        }
    }

    #[inline]
    pub fn in_hitstun(&self, i: usize) -> bool {
        match &self.post.flags {
            Some(f) => Flags::HITSTUN.intersects(f[i]),
            None => false,
        }
    }

    #[inline]
    pub fn in_magnifying_glass(&self, i: usize) -> bool {
        match &self.post.flags {
            Some(f) => Flags::OFFSCREEN.intersects(f[i]),
            None => false,
        }
    }

    #[inline]
    pub fn shielding(&self, i: usize) -> bool {
        match &self.post.flags {
            Some(f) => Flags::SHIELDING.intersects(f[i]),
            None => matches!(
                // deliberately ignoring shield release
                ActionState::from_repr(self.post.action_state[i]).unwrap(),
                ActionState::GUARD_ON
                    | ActionState::GUARD
                    | ActionState::GUARD_SET_OFF
                    | ActionState::GUARD_REFLECT
            ),
        }
    }

    #[inline]
    pub fn fastfalling(&self, i: usize) -> bool {
        match &self.post.flags {
            Some(f) => Flags::FASTFALL.intersects(f[i]),
            None => false,
        }
    }

    #[inline]
    pub fn damaged_state(&self, i: usize) -> bool {
        let state = self.post.action_state[i];
        (AR::DAMAGE_START..=AR::DAMAGE_END).contains(&state)
            || ActionState::DAMAGE_FALL == state
            || ActionState::DOWN_DAMAGE_D == state
            || ActionState::DOWN_DAMAGE_U == state
    }

    #[inline]
    pub fn grabbed(&self, i: usize) -> bool {
        (AR::CAPTURE_START..=AR::CAPTURE_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn cmd_grabbed(&self, i: usize) -> bool {
        let state = self.post.action_state[i];
        ActionState::BARREL_WAIT != state
            && ((AR::COMMAND_GRAB_RANGE1_START..=AR::COMMAND_GRAB_RANGE1_END).contains(&state)
                || (AR::COMMAND_GRAB_RANGE2_START..=AR::COMMAND_GRAB_RANGE2_END).contains(&state))
    }

    #[inline]
    pub fn teching(&self, i: usize) -> bool {
        let state = self.post.action_state[i];
        (AR::TECH_START..=AR::TECH_END).contains(&state)
        // || (AR::DOWN_START..=AR::DOWN_END).contains(&state)
        || ActionState::FLY_REFLECT_CEIL == state
        || ActionState::FLY_REFLECT_WALL == state
    }

    #[inline]
    pub fn downed(&self, i: usize) -> bool {
        (AR::DOWN_START..=AR::DOWN_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn thrown(&self, i: usize) -> bool {
        (AR::THROWN_START..=AR::THROWN_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn dying(&self, i: usize) -> bool {
        (AR::DYING_START..=AR::DYING_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn dodging(&self, i: usize) -> bool {
        (AR::DODGE_START..AR::DODGE_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn shield_broken(&self, i: usize) -> bool {
        (AR::GUARD_BREAK_START..=AR::GUARD_BREAK_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn ledge_action(&self, i: usize) -> bool {
        (AR::LEDGE_ACTION_START..=AR::LEDGE_ACTION_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn special_fall(&self, i: usize) -> bool {
        (AR::FALL_SPECIAL_START..=AR::FALL_SPECIAL_END).contains(&self.post.action_state[i])
    }

    #[inline]
    pub fn just_lost_stock(&self, i: usize) -> bool {
        self.post.stocks[i] < self.post.stocks[i - 1]
    }

    #[inline]
    pub fn just_pressed_any(&self, target: EngineInput, i: usize) -> bool {
        target.intersects(self.pre.engine_buttons[i])
            && !target.intersects(self.pre.engine_buttons[i - 1])
    }

    #[inline]
    pub fn just_pressed_all(&self, target: EngineInput, i: usize) -> bool {
        target.contained_by(self.pre.engine_buttons[i])
            && !target.contained_by(self.pre.engine_buttons[i - 1])
    }
}
