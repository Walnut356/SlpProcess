use crate::{
    checks::{just_pressed_any, is_in_hitlag},
    enums::{ActionState, BitFlags, EngineInput, Flags},
};

#[derive(Debug)]
pub struct StateTracker {
    target: ActionState,
    count: u32,
    prev_state: u16,
}

impl StateTracker {
    pub fn new(target: ActionState) -> Self {
        Self {
            target,
            count: 0,
            // you'll probably not ever be in this state, so it's an okay initial value =)
            prev_state: u16::MAX,
        }
    }

    pub fn check_entered(&mut self, state: u16) {
        if state == self.target && state != self.prev_state {
            self.count += 1;
        }
        self.prev_state = state;
    }

    pub fn check_exited(&mut self, state: u16) {
        if state == self.prev_state && state != self.target {
            self.count += 1;
        }
        self.prev_state = state;
    }

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0
    }
}

#[derive(Debug, Default)]
pub struct LockoutTracker {
    pub tech_window: i32,
    pub lockout_window: i32,
    pub prev_inputs: u32,
    pub prev_flags: u64,
    pub just_pressed: bool,
    pub during_hitlag: bool,
}

impl LockoutTracker {
    pub fn update(&mut self, engine_buttons: u32, flags: u64) {
        let just_input = just_pressed_any(EngineInput::R | EngineInput::L, engine_buttons, self.prev_inputs);
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
            self.lockout_window = 40;
        }


        self.lockout_window -= 1;
        self.tech_window -= 1;

        self.prev_inputs = engine_buttons;
        self.prev_flags = flags;
    }

    pub fn is_locked_out(&self) -> bool {
        self.lockout_window >= 0 && self.tech_window < 0
    }
}
