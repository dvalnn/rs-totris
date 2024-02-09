#![allow(dead_code)]

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct DeltaTime {
    time: Duration,
    last_time: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            time: Duration::default(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.time = now.duration_since(self.last_time);
        self.last_time = now;
    }

    pub fn get(&self) -> Duration {
        self.time
    }

    pub fn fps(&self) -> usize {
        self.time.as_secs_f64().recip().round() as usize
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self {
            last_time: Instant::now(),
            time: Duration::default(),
        }
    }
}

#[derive(Default)]
pub struct Timer {
    accumulator: Duration,
    target: Duration,
    just_finished: bool,
}

impl Timer {
    pub fn new(target: Duration) -> Self {
        Self {
            accumulator: Duration::default(),
            target,
            just_finished: false,
        }
    }

    pub fn update(&mut self, delta_time: DeltaTime) {
        self.update_duration(delta_time.get());
    }

    pub fn update_duration(&mut self, delta: Duration) {
        self.accumulator += delta;
        self.just_finished = self.accumulator >= self.target;
        if self.just_finished {
            self.accumulator -= self.target;
        }
    }

    pub fn set_target(&mut self, target: Duration) {
        self.target = target;
    }

    pub fn reset(&mut self) {
        self.accumulator = Duration::default();
        self.just_finished = false;
    }

    pub fn just_finished(&self) -> bool {
        self.just_finished
    }
}
