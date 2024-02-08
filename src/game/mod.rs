mod timing;

use std::time::Duration;

use crate::engine::{kick_tables::SrsPlus, Engine, MoveKind, RotateKind};

pub use self::timing::{DeltaTime, Timer};

#[derive(Debug)]
pub struct InputAction {
    pub input: Input,
    pub action: KeyAction,
}

impl InputAction {
    pub fn new(input: Input, action: KeyAction) -> Self {
        Self { input, action }
    }
}

#[derive(Debug)]
pub enum KeyAction {
    Press,
    Release,
}

#[derive(Debug)]
pub enum Input {
    Rotate(RotateKind),
    Move(MoveKind),
    HardDrop,
    SoftDrop,
}

#[derive(Default)]
pub struct Game {
    // TODO: maybe re-expose necessary engine methods
    //       instead of having the engine public
    pub engine: Engine,

    tick_timer: Timer,
    fast_timer: Timer,
    lock_timer: Timer,

    repeat_move: bool,
    move_repeat_timer: Timer,

    prev_move: Option<MoveKind>,
    current_move: Option<MoveKind>,

    lock_reset: bool,
    lock_moves: i32,

    hard_drop: bool,
    soft_drop: bool,
}

impl Game {
    pub const LOCK_MOVES: i32 = 15;
    pub const SPEED_MULT: u32 = 20;
    pub const LOCK_TIME: Duration = Duration::from_millis(500);
    pub const FIRST_MOVE_DELAY: Duration = Duration::from_millis(300);
    pub const MOVE_REPEAT_DELAY: Duration = Duration::from_millis(35);

    pub(super) fn new(engine: Engine) -> Self {
        Self {
            engine,
            lock_timer: Timer::new(Self::LOCK_TIME),
            lock_moves: Self::LOCK_MOVES,
            ..Default::default()
        }
    }

    //TODO: Have this return if the move was successful
    fn move_cursor(&mut self, kind: MoveKind) {
        let move_res = self.engine.move_cursor(kind);
        if move_res.is_err() {
            return;
        }
        self.lock_reset = true;

        if self.current_move == Some(kind) {
            self.repeat_move = true;
        } else {
            self.repeat_move = false;
            self.prev_move = self.current_move;
            self.current_move = Some(kind);
        }
    }

    fn move_stop(&mut self, kind: MoveKind) {
        if self.current_move == Some(kind) {
            self.current_move = self.prev_move.take();
        } else {
            self.prev_move = None;
        }
    }

    fn rotate_cursor(&mut self, rotation: RotateKind) {
        // let kick = SrsPlus::Kick1(
        //     self.engine.cursor_kind(),
        //     self.engine.cursor_rotation(),
        //     rotation,
        // );
        //
        let kick = SrsPlus::Kick1;

        match self.engine.rotate_cursor(rotation, Some(kick)) {
            Ok(_) => self.lock_reset = true,
            Err(_) => todo!(),
        }
    }

    pub fn handle_input(&mut self, InputAction { input, action }: InputAction) {
        use Input::*;
        use KeyAction::*;
        match (input, action) {
            (Rotate(kind), Press) => self.rotate_cursor(kind),
            (Rotate(_), Release) => {}

            (Move(kind), Press) => self.move_cursor(kind),
            (Move(kind), Release) => self.move_stop(kind),

            (HardDrop, Press) => self.hard_drop = true,
            (HardDrop, Release) => {} // NOTE: Does nothing

            (SoftDrop, Press) => self.soft_drop = true,
            (SoftDrop, Release) => self.soft_drop = false,
        }
    }

    fn update_timers(&mut self, tick_time: Duration, delta_time: DeltaTime) {
        self.tick_timer.set_target(tick_time);
        self.fast_timer.set_target(tick_time / Self::SPEED_MULT);

        self.tick_timer.update(delta_time);
        self.fast_timer.update(delta_time);
        self.lock_timer.update(delta_time);
        self.move_repeat_timer.update(delta_time);
    }

    //TODO: Take a second look at this function to
    //      see if it can be simplified / split up
    //
    //TODO: Return a struct with game state info
    //      to be used for rendering / animation
    //      networking / etc.
    pub fn update(&mut self, delta_time: DeltaTime) {
        if self.engine.cursor_info().is_none() {
            self.engine.add_cursor();
        }

        if self.repeat_move {
            self.move_repeat_timer.set_target(Self::MOVE_REPEAT_DELAY);
        } else {
            self.move_repeat_timer.set_target(Self::FIRST_MOVE_DELAY);
        }

        self.update_timers(self.engine.drop_time(), delta_time);

        if let Some(move_kind) = self.current_move {
            if self.move_repeat_timer.just_finished() {
                //TODO: have this result wrapped and returned
                self.move_cursor(move_kind);
            }
        } else {
            self.repeat_move = false;
            self.move_repeat_timer.reset();
        }

        if self.hard_drop {
            self.engine.hard_drop()
        }

        let mut check_lines = false;
        if self.engine.cursor_has_hit_bottom() {
            if self.lock_reset && self.lock_moves > 0 {
                self.lock_timer.reset();
                self.lock_reset = false;
                self.lock_moves -= 1;
            }

            //TODO: rethink how the hard drop is handled
            if self.hard_drop || self.lock_timer.just_finished() {
                self.hard_drop = false;
                check_lines = true;
                self.engine.place_cursor();
            }
        } else {
            self.lock_timer.reset();

            let tick = self.tick_timer.just_finished();
            let fast_tick = self.fast_timer.just_finished();
            if (self.soft_drop && fast_tick) || tick {
                self.engine.tick_down();
                self.lock_moves = Self::LOCK_MOVES;
            }
        }

        if check_lines {
            //TODO: change this funtion to return the cleared
            //      lines indices
            self.engine.line_clear(|_| (/*canvas animation*/));
        }
    }
}
