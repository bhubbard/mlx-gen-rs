use crate::config::task::OutpaintFillMode;
use crate::error::Result;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaddingSpec {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl PaddingSpec {
    pub fn uniform(pad: u32) -> Self {
        Self {
            top: pad,
            right: pad,
            bottom: pad,
            left: pad,
        }
    }

    pub fn axis(vertical: u32, horizontal: u32) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }

    pub fn total_width(&self, base_width: u32) -> u32 {
        base_width + self.left + self.right
    }

    pub fn total_height(&self, base_height: u32) -> u32 {
        base_height + self.top + self.bottom
    }

    pub fn should_split_two_pass(&self, base_w: u32, base_h: u32) -> bool {
        let h_ratio = (self.left + self.right) as f32 / base_w as f32;
        let v_ratio = (self.top + self.bottom) as f32 / base_h as f32;
        h_ratio > 0.4 && v_ratio > 0.4
    }
}

pub struct OutpaintCanvas;

impl OutpaintCanvas {
    pub fn create_expanded_canvas(
        source_image: &DynamicImage,
        padding: &PaddingSpec,
        fill_mode: OutpaintFillMode,
    ) -> Result<(DynamicImage, DynamicImage)> {
        let (src_w, src_h) = source_image.dimensions();
        let new_w = padding.total_width(src_w);
        let new_h = padding.total_height(src_h);

        let mut expanded_img = ImageBuffer::new(new_w, new_h);
        let mut mask_img = ImageBuffer::new(new_w, new_h);

        // Pre-fill background based on mode
        let fill_color = match fill_mode {
            OutpaintFillMode::Auto | OutpaintFillMode::Edge => Rgba([128, 128, 128, 255]),
            OutpaintFillMode::Neutral => Rgba([128, 128, 128, 255]),
            OutpaintFillMode::Solid => Rgba([0, 255, 0, 255]), // Pure green chroma canvas
            OutpaintFillMode::Blur => Rgba([64, 64, 64, 255]),
        };

        for pixel in expanded_img.pixels_mut() {
            *pixel = fill_color;
        }

        // Mask: 255 where outpaint is needed (new pixels), 0 where source exists
        for pixel in mask_img.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]);
        }

        // Paste original image at offset (padding.left, padding.top)
        for y in 0..src_h {
            for x in 0..src_w {
                let px = source_image.get_pixel(x, y);
                expanded_img.put_pixel(x + padding.left, y + padding.top, px);
                // Mark source area as 0 in mask
                mask_img.put_pixel(x + padding.left, y + padding.top, Rgba([0, 0, 0, 255]));
            }
        }

        Ok((
            DynamicImage::ImageRgba8(expanded_img),
            DynamicImage::ImageRgba8(mask_img),
        ))
    }
}
