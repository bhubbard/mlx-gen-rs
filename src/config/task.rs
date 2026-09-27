use crate::error::{MlxGenError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskType {
    Auto,
    TextToImage,
    ImageToImage,
    Edit,
    TextToVideo,
    ImageToVideo,
    VideoToVideo,
}

impl TaskType {
    pub fn is_video(&self) -> bool {
        matches!(
            self,
            TaskType::TextToVideo | TaskType::ImageToVideo | TaskType::VideoToVideo
        )
    }

    pub fn is_image(&self) -> bool {
        matches!(
            self,
            TaskType::TextToImage | TaskType::ImageToImage | TaskType::Edit
        )
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TaskType::Auto => "auto",
            TaskType::TextToImage => "text-to-image",
            TaskType::ImageToImage => "image-to-image",
            TaskType::Edit => "edit",
            TaskType::TextToVideo => "text-to-video",
            TaskType::ImageToVideo => "image-to-video",
            TaskType::VideoToVideo => "video-to-video",
        }
    }
}

impl fmt::Display for TaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for TaskType {
    type Err = MlxGenError;

    fn from_str(s: &str) -> Result<Self> {
        let normalized = s.trim().to_lowercase();
        match normalized.as_str() {
            "auto" => Ok(TaskType::Auto),
            "text-to-image" | "txt2img" | "t2i" => Ok(TaskType::TextToImage),
            "image-to-image" | "img2img" | "i2i" => Ok(TaskType::ImageToImage),
            "edit" | "instruction-edit" => Ok(TaskType::Edit),
            "text-to-video" | "txt2vid" | "t2v" => Ok(TaskType::TextToVideo),
            "image-to-video" | "img2vid" | "i2v" => Ok(TaskType::ImageToVideo),
            "video-to-video" | "vid2vid" | "v2v" => Ok(TaskType::VideoToVideo),
            other => Err(MlxGenError::TaskInference(format!(
                "Unrecognized task '{}'. Valid tasks: auto, text-to-image, image-to-image, edit, text-to-video, image-to-video, video-to-video",
                other
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutpaintFillMode {
    Auto,
    Edge,
    Neutral,
    Solid,
    Blur,
}

impl Default for OutpaintFillMode {
    fn default() -> Self {
        OutpaintFillMode::Auto
    }
}

impl FromStr for OutpaintFillMode {
    type Err = MlxGenError;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "auto" => Ok(OutpaintFillMode::Auto),
            "edge" => Ok(OutpaintFillMode::Edge),
            "neutral" => Ok(OutpaintFillMode::Neutral),
            "solid" => Ok(OutpaintFillMode::Solid),
            "blur" => Ok(OutpaintFillMode::Blur),
            other => Err(MlxGenError::Outpaint(format!(
                "Invalid outpaint fill mode: '{}'. Choices: auto, edge, neutral, solid, blur",
                other
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CanvasPolicy {
    ExactResize,
    SourceAspect,
    Pad,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationDimensions {
    pub width: u32,
    pub height: u32,
    pub frames: Option<u32>,
    pub fps: Option<f32>,
}

impl GenerationDimensions {
    pub fn new_2d(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            frames: None,
            fps: None,
        }
    }

    pub fn new_video(width: u32, height: u32, frames: u32, fps: f32) -> Self {
        Self {
            width,
            height,
            frames: Some(frames),
            fps: Some(fps),
        }
    }
}
