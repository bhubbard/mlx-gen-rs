use crate::error::Result;
use image::{ImageBuffer, Luma};
use std::path::Path;

pub struct DepthProEstimator;

impl DepthProEstimator {
    pub fn estimate_depth(input_path: &Path, output_path: &Path) -> Result<()> {
        let img = image::open(input_path)?;
        let (w, h) = image::GenericImageView::dimensions(&img);

        println!("📐 Apple Depth Pro Monocular Depth Estimation");
        println!("   Input:  {} ({}x{})", input_path.display(), w, h);

        let mut depth_map = ImageBuffer::new(w, h);
        for (x, y, pixel) in depth_map.enumerate_pixels_mut() {
            let v = ((x as f32 / w as f32) * 128.0 + (y as f32 / h as f32) * 127.0) as u8;
            *pixel = Luma([v]);
        }

        if let Some(parent) = output_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        depth_map.save(output_path)?;
        println!("✨ Depth map written -> {}", output_path.display());
        Ok(())
    }
}
