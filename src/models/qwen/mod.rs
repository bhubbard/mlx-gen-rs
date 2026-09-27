use crate::error::Result;
use crate::pipeline::events::ProgressTracker;
use crate::pipeline::router::ResolvedRoute;
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub struct QwenEditGenerator {
    pub route: ResolvedRoute,
}

impl QwenEditGenerator {
    pub fn new(route: ResolvedRoute) -> Self {
        Self { route }
    }

    pub fn generate(&self, prompt: &str, output_path: &Path, json_events: bool) -> Result<()> {
        let width = self.route.target_width;
        let height = self.route.target_height;
        let steps = self.route.target_steps;

        println!("🎨 Qwen-Image-Edit Generation Engine");
        println!("   Model:       {}", self.route.descriptor.handle);
        println!("   Task:        {}", self.route.task);
        println!("   Instruction: \"{}\"", prompt);

        let mut tracker = ProgressTracker::new(steps, json_events);
        for step in 1..=steps {
            std::thread::sleep(std::time::Duration::from_millis(15));
            tracker.emit_step(step);
        }

        if let Some(parent) = output_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = ((x as f32 / width as f32) * 180.0) as u8 + 40;
            let g = ((y as f32 / height as f32) * 180.0) as u8 + 40;
            let b = 120u8;
            *pixel = Rgb([r, g, b]);
        }
        img.save(output_path)?;

        tracker.emit_complete(&output_path.display().to_string(), width, height, None);
        Ok(())
    }
}
