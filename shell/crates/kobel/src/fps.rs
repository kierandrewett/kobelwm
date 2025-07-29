use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FpsCounter {
    last_frame: Instant,
    frame_times: Vec<f64>,
    max_samples: usize,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            frame_times: Vec::with_capacity(100),
            max_samples: 100,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f64();
        self.last_frame = now;

        self.frame_times.push(dt);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }
    }

    pub fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let avg = self.frame_times.iter().copied().sum::<f64>() / self.frame_times.len() as f64;
        1.0 / avg
    }
}