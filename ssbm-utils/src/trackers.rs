//! Trackers are small, self contained structs that keep track of in-game parameters through an
//! `update` method. These are useful in stats code for tracking things like tech lockout windows
//! and state transitions

use crate::{
    checks::{just_pressed_any, is_in_hitlag},
    enums::{ActionState,  EngineInput},
};

#[derive(Debug)]
pub struct StateTracker {
    target: ActionState,
    prev_state: u16,
    just_entered: bool,
    just_exited: bool,
}

impl StateTracker {
    pub fn new(target: ActionState) -> Self {
        Self {
            target,
            // you'll probably not ever be in this state, so it's an okay initial value =)
            prev_state: u16::MAX,
            just_entered: false,
            just_exited: false,
        }
    }

    pub fn update(&mut self, state: u16) {
        self.just_entered = state == self.target && self.prev_state != self.target;
        self.just_exited = state != self.target && self.prev_state == self.target;
    }

    pub fn just_entered(&self) -> bool {
        self.just_entered
    }

    pub fn just_exited(&self) -> bool {
        self.just_exited
    }
}

/// Tracks tech/vcancel lockout behavior.
///
/// ```ignore
/// let mut tracker = LockoutTracker::default();
///
/// for i in 1..frames.len() {
///    tracker.update(frames.pre.engine_buttons[i], frames.post.flags.as_ref().unwrap()[i]);
///
///    if tracker.is_locked_out() {
///         ...
///     }
///
///    ...
/// }
/// ```
#[derive(Debug, Default)]
pub struct LockoutTracker {
    tech_window: i32,
    lockout_window: i32,
    prev_inputs: u32,
    prev_flags: u64,
    just_pressed: bool,
    during_hitlag: bool,
}

impl LockoutTracker {
    /// Updates the state of the tracker with new frame information. Requires frame data to be
    /// passed in order
    pub fn update(&mut self, engine_inputs: u32, flags: u64) {
        let just_input = just_pressed_any(EngineInput::R | EngineInput::L, engine_inputs, self.prev_inputs);
        let in_hitlag = is_in_hitlag(flags);
        let just_out_hl = !in_hitlag && is_in_hitlag(self.prev_flags);

        /*
        We purposfully use self.just_pressed before updating it, as it represents the previous
        frame. If we just pressed on the last frame of hitlag, the window is still valid, thus we
        need to retroactively fix it.
        */
        if just_out_hl && self.just_pressed {
            self.tech_window = 19;
        }

        self.just_pressed = just_input;
        self.during_hitlag = just_input && in_hitlag;

        /*
        If we're not in hitlag, don't have a lockout window, and just pressed, open 20f window. When
        pressing on any hitlag frame other than the last, the input is considered "repeated" every
        frame, instantly locking out the tech window.
        */
        if just_input && self.lockout_window < 0 && !in_hitlag {
            self.tech_window = 20;
        }

        if just_input {
            if self.lockout_window >= 0 {
                self.tech_window = 0;
            }
            self.lockout_window = 40;
        }


        self.lockout_window -= 1;
        self.tech_window -= 1;

        self.prev_inputs = engine_inputs;
        self.prev_flags = flags;
    }

    /// Returns true if the player is currently locked out of teching
    pub fn is_locked_out(&self) -> bool {
        self.lockout_window >= 0 && self.tech_window < 0
    }

    /// Returns true if the 2 frame vcancel window is active
    pub fn can_vcancel(&self) -> bool {
        (18..20).contains(&self.tech_window)
    }

    /// Returns true if the 20 frame tech window is active
    pub fn can_tech(&self) -> bool {
        self.tech_window >= 0
    }

    /// Returns true if the input occurred during hitlag. For any hitlag frame except the last,
    /// pressing L or R will be treated as pressing it repeatedly every frame of hitlag, thus
    /// instantly cancelling the tech window and incurring lockout
    pub fn input_during_hitlag(&self) -> bool {
        self.during_hitlag
    }

    /// Returns true if the player was not pressing last frame, but is pressing this frame.
    pub fn just_pressed(&self) -> bool {
        self.just_pressed
    }

    /// Returns the current lockout window in frames. Negative values mean the lockout window is
    /// closed
    pub fn lockout_window(&self) -> i32 {
        self.lockout_window
    }

    /// Returns the number of frames since L or R was last input. Always a negative number.
    pub fn frames_since_input(&self) -> i32 {
        -(40 - self.lockout_window)
    }
}
