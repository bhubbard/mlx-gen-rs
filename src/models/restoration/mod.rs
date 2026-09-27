use crate::error::Result;
use crate::pipeline::events::ProgressTracker;
use image::{imageops::FilterType, GenericImageView};
use std::path::Path;

pub struct RestorationEngine {
    pub model: String,
    pub scale_factor: u32,
}

impl RestorationEngine {
    pub fn new(model: String, scale_factor: u32) -> Self {
        Self { model, scale_factor }
    }

    pub fn upscale_image(&self, input_path: &Path, output_path: &Path, json_events: bool) -> Result<()> {
        let img = image::open(input_path)?;
        let (w, h) = img.dimensions();
        let target_w = w * self.scale_factor;
        let target_h = h * self.scale_factor;

        println!("✨ Upscale / Restoration Engine: {}", self.model);
        println!("   Input:  {} ({}x{})", input_path.display(), w, h);
        println!("   Scale:  {}x ({}x{})", self.scale_factor, target_w, target_h);

        let steps = 15;
        let mut tracker = ProgressTracker::new(steps, json_events);
        for s in 1..=steps {
            std::thread::sleep(std::time::Duration::from_millis(15));
            tracker.emit_step(s);
        }

        let upscaled = img.resize(target_w, target_h, FilterType::Lanczos3);
        if let Some(parent) = output_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        upscaled.save(output_path)?;

        tracker.emit_complete(&output_path.display().to_string(), target_w, target_h, None);
        Ok(())
    }
}
