use std::time::{Duration, Instant};

pub struct FpsCounter {
    pub fps: f32,
    fps_updated: bool,
    frames: u32,
    last_time: Instant,
    refresh_rate: f32,
    report_fps_callback: Box<dyn FnMut(f32)>,
}

impl FpsCounter {
    pub fn new(refresh_rate: f32) -> FpsCounter {
        FpsCounter {
            fps: 0.0,
            fps_updated: false,
            frames: 0,
            last_time: Instant::now(),
            refresh_rate,
            report_fps_callback: Box::new(|_| {}),
        }
    }

    pub fn set_report_fps_callback(&mut self, callback: Box<dyn FnMut(f32)>) {
        self.report_fps_callback = callback;
    }

    pub fn tick(&mut self) {
        self.frames += 1;
        let current_time = Instant::now();
        let frame_duration = current_time - self.last_time;
        let refresh_duration = Duration::from_secs_f32(1.0 / self.refresh_rate);
        self.fps_updated = false;

        if frame_duration >= refresh_duration {
            self.fps = self.frames as f32 / frame_duration.as_secs_f32();
            self.frames = 0;
            self.last_time = current_time;
            self.fps_updated = true;
            (self.report_fps_callback)(self.fps);
        }
    }

    pub fn is_refreshed(&self) -> bool {
        self.fps_updated
    }
}
