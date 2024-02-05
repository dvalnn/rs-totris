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
}
