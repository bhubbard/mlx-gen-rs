use crate::config::model_config::{ModelDescriptor, ModelFamily, ModelRegistry};
use crate::config::task::TaskType;
use crate::error::{MlxGenError, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GenerationRequest {
    pub model: String,
    pub task: TaskType,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub images: Vec<PathBuf>,
    pub videos: Vec<PathBuf>,
    pub reference_images: Vec<PathBuf>,
    pub mask: Option<PathBuf>,
    pub video_mask: Option<PathBuf>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frames: Option<u32>,
    pub fps: Option<f32>,
    pub steps: Option<u32>,
    pub guidance: Option<f32>,
    pub seed: Option<u64>,
    pub flow_shift: Option<f32>,
    pub output_path: PathBuf,
    pub low_ram: bool,
    pub json_events: bool,
}

#[derive(Debug, Clone)]
pub struct ResolvedRoute {
    pub descriptor: ModelDescriptor,
    pub task: TaskType,
    pub target_width: u32,
    pub target_height: u32,
    pub target_frames: Option<u32>,
    pub target_fps: Option<f32>,
    pub target_steps: u32,
    pub target_guidance: f32,
    pub flow_shift: Option<f32>,
}

pub struct TaskRouter;

impl TaskRouter {
    pub fn resolve(req: &GenerationRequest) -> Result<ResolvedRoute> {
        let descriptor = ModelRegistry::find(&req.model).ok_or_else(|| {
            MlxGenError::RouteResolution(format!(
                "Model '{}' not found in registry. Check `mlxgen capabilities` for supported models.",
                req.model
            ))
        })?;

        let inferred_task = if req.task == TaskType::Auto {
            Self::infer_task(&descriptor, req)?
        } else {
            req.task
        };

        if !descriptor.supported_tasks.contains(&inferred_task) {
            return Err(MlxGenError::RouteResolution(format!(
                "Model '{}' does not support task '{}'. Supported tasks: {:?}",
                descriptor.handle, inferred_task, descriptor.supported_tasks
            )));
        }

        // Validate required inputs
        if inferred_task == TaskType::ImageToImage && req.images.is_empty() {
            return Err(MlxGenError::RouteResolution(
                "Image-to-image task requires at least one --image input.".to_string(),
            ));
        }

        if inferred_task == TaskType::ImageToVideo && req.images.is_empty() {
            return Err(MlxGenError::RouteResolution(
                "Image-to-video task requires an initial --image input.".to_string(),
            ));
        }

        if inferred_task == TaskType::VideoToVideo && req.videos.is_empty() {
            return Err(MlxGenError::RouteResolution(
                "Video-to-video task requires at least one --video input.".to_string(),
            ));
        }

        let target_width = req.width.unwrap_or(descriptor.default_width);
        let target_height = req.height.unwrap_or(descriptor.default_height);
        let target_frames = req.frames.or(descriptor.default_frames);
        let target_fps = req.fps.or(descriptor.default_fps);
        let target_steps = req.steps.unwrap_or(descriptor.default_steps);
        let target_guidance = req.guidance.unwrap_or(descriptor.default_guidance);
        let flow_shift = req.flow_shift.or(descriptor.default_flow_shift);

        Ok(ResolvedRoute {
            descriptor,
            task: inferred_task,
            target_width,
            target_height,
            target_frames,
            target_fps,
            target_steps,
            target_guidance,
            flow_shift,
        })
    }

    fn infer_task(descriptor: &ModelDescriptor, req: &GenerationRequest) -> Result<TaskType> {
        let has_image = !req.images.is_empty();
        let has_video = !req.videos.is_empty();

        match descriptor.family {
            ModelFamily::Wan | ModelFamily::MiniMax => {
                if has_video {
                    Ok(TaskType::VideoToVideo)
                } else if has_image {
                    Ok(TaskType::ImageToVideo)
                } else {
                    Ok(TaskType::TextToVideo)
                }
            }
            ModelFamily::Flux | ModelFamily::Flux2 | ModelFamily::ZImage | ModelFamily::Bonsai | ModelFamily::Ernie => {
                if has_image {
                    Ok(TaskType::ImageToImage)
                } else {
                    Ok(TaskType::TextToImage)
                }
            }
            ModelFamily::Qwen => {
                if req.mask.is_some() || has_image {
                    Ok(TaskType::Edit)
                } else {
                    Ok(TaskType::TextToImage)
                }
            }
            ModelFamily::SeedVR2 | ModelFamily::SwiftVR => {
                if has_video {
                    Ok(TaskType::VideoToVideo)
                } else {
                    Ok(TaskType::ImageToImage)
                }
            }
            ModelFamily::DepthPro | ModelFamily::Fibo => Ok(TaskType::ImageToImage),
        }
    }
}
