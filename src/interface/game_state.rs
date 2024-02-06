use std::time::Duration;

use crate::engine::Engine;

use super::{
    timing::{DeltaTime, Timer},
    MoveKind, RotateKind,
};

//NOTE: Maybe move engine into GameState?
#[derive(Default)]
pub(super) struct GameState {
    tick_timer: Timer,
    fast_timer: Timer,
    lock_timer: Timer,
    move_timer: Timer,

    first_move: bool,
    prev_move: Option<MoveKind>,

    lock_reset: bool,
    lock_moves: i32,

    pub(super) hard_drop: bool,
    pub(super) soft_drop: bool,
    pub(super) move_left: bool,
    pub(super) move_right: bool,
    pub(super) rotate_left: bool,
    pub(super) rotate_right: bool,
}

impl GameState {
    pub(super) const LOCK_TIME: Duration = Duration::from_millis(500);
    pub(super) const SOFT_DROP_SPEED_UP: u32 = 20;
    pub(super) const LOCK_MOVES: i32 = 15;

    fn update_timers(&mut self, tick_time: Duration, delta_time: DeltaTime) {
        self.tick_timer.set_new_target(tick_time);
        self.fast_timer
            .set_new_target(tick_time / Self::SOFT_DROP_SPEED_UP);
        self.tick_timer.update(delta_time);
        self.fast_timer.update(delta_time);
        self.lock_timer.update(delta_time);
    }

    pub(super) fn new() -> Self {
        Self {
            lock_timer: Timer::new(Self::LOCK_TIME),
            lock_moves: Self::LOCK_MOVES,
            ..Default::default()
        }
    }

    pub(super) fn move_cursor(&mut self, engine: &mut Engine, kind: MoveKind) {
        self.lock_reset = true;
        let _ = engine.move_cursor(kind);
    }

    pub(super) fn rotate_cursor(
        &mut self,
        engine: &mut Engine,
        kind: RotateKind,
    ) {
        self.lock_reset = true;
        let _ = engine.rotate_cursor(kind);
    }

    pub(super) fn update(
        &mut self,
        engine: &mut Engine,
        delta_time: DeltaTime,
    ) {
        let mut check_lines = false;
        self.update_timers(engine.drop_time(), delta_time);

        if self.hard_drop {
            engine.hard_drop()
        }

        if engine.cursor_has_hit_bottom() {
            if self.lock_reset && self.lock_moves > 0 {
                self.lock_timer.reset();
                self.lock_reset = false;
                self.lock_moves -= 1;
            }

            //TODO: rethink how the hard drop is handled
            if self.hard_drop || self.lock_timer.just_finished() {
                self.hard_drop = false;
                check_lines = true;
                engine.place_cursor();
            }
        } else {
            self.lock_timer.reset();

            let tick = self.tick_timer.just_finished();
            let fast_tick = self.fast_timer.just_finished();
            if (self.soft_drop && fast_tick) || tick {
                engine.tick_down();
                self.lock_moves = 15;
            }
        }

        if check_lines {
            //TODO: change this funtion to return the cleared
            //      lines indices
            engine.line_clear(|_| (/*canvas animation*/));
        }
    }
}
