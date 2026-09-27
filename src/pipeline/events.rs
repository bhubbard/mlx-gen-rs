use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum PipelineEvent {
    StepProgress {
        step: u32,
        total_steps: u32,
        elapsed_sec: f32,
        eta_sec: f32,
        fps: f32,
    },
    FrameDecoded {
        frame_idx: u32,
        total_frames: u32,
    },
    VaeDecoding {
        progress: f32,
    },
    AudioSynthesized {
        sample_rate: u32,
        duration_sec: f32,
    },
    Complete {
        total_time_sec: f32,
        output_path: String,
        width: u32,
        height: u32,
        frames: Option<u32>,
    },
    Error {
        message: String,
    },
}

pub struct ProgressTracker {
    start_time: Instant,
    last_step_time: Instant,
    total_steps: u32,
    json_events: bool,
}

impl ProgressTracker {
    pub fn new(total_steps: u32, json_events: bool) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_step_time: now,
            total_steps,
            json_events,
        }
    }

    pub fn emit_step(&mut self, step: u32) {
        let now = Instant::now();
        let elapsed_sec = self.start_time.elapsed().as_secs_f32();
        let step_duration = self.last_step_time.elapsed().as_secs_f32();
        self.last_step_time = now;

        let steps_remaining = self.total_steps.saturating_sub(step);
        let eta_sec = if step > 0 {
            (elapsed_sec / step as f32) * steps_remaining as f32
        } else {
            0.0
        };
        let fps = if step_duration > 0.0 { 1.0 / step_duration } else { 0.0 };

        let event = PipelineEvent::StepProgress {
            step,
            total_steps: self.total_steps,
            elapsed_sec,
            eta_sec,
            fps,
        };

        if self.json_events {
            println!("{}", serde_json::to_string(&event).unwrap_or_default());
        } else {
            let pct = (step as f32 / self.total_steps as f32 * 100.0) as u32;
            print!(
                "\r⚡ Step [{:3}/{:3}] ({:3}%) | Elapsed: {:.1}s | ETA: {:.1}s | Speed: {:.2} it/s",
                step, self.total_steps, pct, elapsed_sec, eta_sec, fps
            );
            use std::io::Write;
            let _ = std::io::stdout().flush();
        }
    }

    pub fn emit_complete(&self, output_path: &str, width: u32, height: u32, frames: Option<u32>) {
        let total_time_sec = self.start_time.elapsed().as_secs_f32();
        let event = PipelineEvent::Complete {
            total_time_sec,
            output_path: output_path.to_string(),
            width,
            height,
            frames,
        };

        if self.json_events {
            println!("{}", serde_json::to_string(&event).unwrap_or_default());
        } else {
            println!("\n✨ Generation complete in {:.2}s -> {}", total_time_sec, output_path);
        }
    }
}
