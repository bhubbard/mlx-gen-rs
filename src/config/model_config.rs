use crate::config::quantization::QuantizationMode;
use crate::config::task::TaskType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelFamily {
    Wan,
    Flux,
    Flux2,
    MiniMax,
    Qwen,
    ZImage,
    SeedVR2,
    SwiftVR,
    DepthPro,
    Bonsai,
    Ernie,
    Fibo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescriptor {
    pub handle: String,
    pub aliases: Vec<String>,
    pub family: ModelFamily,
    pub default_repo_id: String,
    pub default_quantization: QuantizationMode,
    pub default_width: u32,
    pub default_height: u32,
    pub default_frames: Option<u32>,
    pub default_fps: Option<f32>,
    pub default_steps: u32,
    pub default_guidance: f32,
    pub default_flow_shift: Option<f32>,
    pub supported_tasks: Vec<TaskType>,
    pub supports_lora: bool,
    pub supports_mask: bool,
    pub supports_outpaint: bool,
    pub supports_video_mask: bool,
    pub supports_reference_images: bool,
    pub supports_audio: bool,
}

impl ModelDescriptor {
    pub fn matches(&self, query: &str) -> bool {
        let q = query.trim().to_lowercase();
        if self.handle.to_lowercase() == q {
            return true;
        }
        if self.default_repo_id.to_lowercase() == q {
            return true;
        }
        self.aliases.iter().any(|a| a.to_lowercase() == q)
    }
}

pub struct ModelRegistry;

impl ModelRegistry {
    pub fn all_models() -> Vec<ModelDescriptor> {
        vec![
            // Wan 2.2 TI2V-5B
            ModelDescriptor {
                handle: "wan2.2-ti2v-5b".to_string(),
                aliases: vec!["wan-5b".to_string(), "wan2.2-5b".to_string(), "wan".to_string()],
                family: ModelFamily::Wan,
                default_repo_id: "Wan-AI/Wan2.2-TI2V-5B".to_string(),
                default_quantization: QuantizationMode::Bf16,
                default_width: 832,
                default_height: 480,
                default_frames: Some(81),
                default_fps: Some(16.0),
                default_steps: 40,
                default_guidance: 6.0,
                default_flow_shift: Some(3.0),
                supported_tasks: vec![TaskType::TextToVideo, TaskType::ImageToVideo, TaskType::VideoToVideo],
                supports_lora: true,
                supports_mask: false,
                supports_outpaint: false,
                supports_video_mask: true,
                supports_reference_images: true,
                supports_audio: false,
            },
            // Wan 2.2 T2V-A14B
            ModelDescriptor {
                handle: "wan2.2-t2v-14b".to_string(),
                aliases: vec!["wan-14b".to_string(), "wan2.2-14b".to_string()],
                family: ModelFamily::Wan,
                default_repo_id: "Wan-AI/Wan2.2-T2V-A14B".to_string(),
                default_quantization: QuantizationMode::Q8,
                default_width: 1280,
                default_height: 720,
                default_frames: Some(81),
                default_fps: Some(16.0),
                default_steps: 50,
                default_guidance: 5.0,
                default_flow_shift: Some(5.0),
                supported_tasks: vec![TaskType::TextToVideo, TaskType::ImageToVideo, TaskType::VideoToVideo],
                supports_lora: true,
                supports_mask: false,
                supports_outpaint: false,
                supports_video_mask: true,
                supports_reference_images: true,
                supports_audio: false,
            },
            // MiniMax H3
            ModelDescriptor {
                handle: "minimax-h3".to_string(),
                aliases: vec!["hailuo".to_string(), "minimax".to_string(), "minimax-h3-turbo".to_string()],
                family: ModelFamily::MiniMax,
                default_repo_id: "MiniMax/MiniMax-H3".to_string(),
                default_quantization: QuantizationMode::Q8,
                default_width: 960,
                default_height: 544,
                default_frames: Some(120),
                default_fps: Some(24.0),
                default_steps: 30,
                default_guidance: 4.5,
                default_flow_shift: Some(4.0),
                supported_tasks: vec![TaskType::TextToVideo, TaskType::ImageToVideo],
                supports_lora: true,
                supports_mask: false,
                supports_outpaint: false,
                supports_video_mask: false,
                supports_reference_images: false,
                supports_audio: true, // Synced stereo soundtrack
            },
            // FLUX.1 Schnell
            ModelDescriptor {
                handle: "flux-schnell".to_string(),
                aliases: vec!["schnell".to_string(), "black-forest-labs/FLUX.1-schnell".to_string()],
                family: ModelFamily::Flux,
                default_repo_id: "black-forest-labs/FLUX.1-schnell".to_string(),
                default_quantization: QuantizationMode::Q8,
                default_width: 1024,
                default_height: 1024,
                default_frames: None,
                default_fps: None,
                default_steps: 4,
                default_guidance: 0.0,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::TextToImage, TaskType::ImageToImage],
                supports_lora: true,
                supports_mask: true,
                supports_outpaint: true,
                supports_video_mask: false,
                supports_reference_images: false,
                supports_audio: false,
            },
            // FLUX.1 Dev
            ModelDescriptor {
                handle: "flux-dev".to_string(),
                aliases: vec!["dev".to_string(), "black-forest-labs/FLUX.1-dev".to_string()],
                family: ModelFamily::Flux,
                default_repo_id: "black-forest-labs/FLUX.1-dev".to_string(),
                default_quantization: QuantizationMode::Q8,
                default_width: 1024,
                default_height: 1024,
                default_frames: None,
                default_fps: None,
                default_steps: 25,
                default_guidance: 3.5,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::TextToImage, TaskType::ImageToImage, TaskType::Edit],
                supports_lora: true,
                supports_mask: true,
                supports_outpaint: true,
                supports_video_mask: false,
                supports_reference_images: true,
                supports_audio: false,
            },
            // FLUX.2 Klein 4B Distilled
            ModelDescriptor {
                handle: "flux2-klein-4b".to_string(),
                aliases: vec!["klein-4b".to_string(), "flux2-4b".to_string()],
                family: ModelFamily::Flux2,
                default_repo_id: "black-forest-labs/FLUX.2-klein-4b".to_string(),
                default_quantization: QuantizationMode::Q8,
                default_width: 1024,
                default_height: 1024,
                default_frames: None,
                default_fps: None,
                default_steps: 8,
                default_guidance: 3.5,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::TextToImage, TaskType::ImageToImage, TaskType::Edit],
                supports_lora: true,
                supports_mask: true,
                supports_outpaint: true,
                supports_video_mask: false,
                supports_reference_images: true,
                supports_audio: false,
            },
            // Qwen-Image & Qwen-Edit
            ModelDescriptor {
                handle: "qwen-image-edit".to_string(),
                aliases: vec!["qwen-edit".to_string(), "qwen-2509".to_string(), "qwen-2511".to_string()],
                family: ModelFamily::Qwen,
                default_repo_id: "Qwen/Qwen-Image-Edit-2511".to_string(),
                default_quantization: QuantizationMode::Bf16,
                default_width: 1024,
                default_height: 1024,
                default_frames: None,
                default_fps: None,
                default_steps: 20,
                default_guidance: 4.0,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::TextToImage, TaskType::ImageToImage, TaskType::Edit],
                supports_lora: true,
                supports_mask: true,
                supports_outpaint: true,
                supports_video_mask: false,
                supports_reference_images: true,
                supports_audio: false,
            },
            // Z-Image Turbo
            ModelDescriptor {
                handle: "z-image-turbo".to_string(),
                aliases: vec!["z-image".to_string(), "zturbo".to_string()],
                family: ModelFamily::ZImage,
                default_repo_id: "Z-Image/z-image-turbo".to_string(),
                default_quantization: QuantizationMode::Q8,
                default_width: 1024,
                default_height: 1024,
                default_frames: None,
                default_fps: None,
                default_steps: 8,
                default_guidance: 2.0,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::TextToImage, TaskType::ImageToImage],
                supports_lora: false,
                supports_mask: true,
                supports_outpaint: false,
                supports_video_mask: false,
                supports_reference_images: false,
                supports_audio: false,
            },
            // SeedVR2 Restoration (3B & 7B)
            ModelDescriptor {
                handle: "seedvr2-3b".to_string(),
                aliases: vec!["seedvr2".to_string(), "bytedance-seed/seedvr2-3b".to_string()],
                family: ModelFamily::SeedVR2,
                default_repo_id: "bytedance-seed/seedvr2-3b".to_string(),
                default_quantization: QuantizationMode::Bf16,
                default_width: 1920,
                default_height: 1080,
                default_frames: None,
                default_fps: None,
                default_steps: 15,
                default_guidance: 1.0,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::ImageToImage, TaskType::VideoToVideo],
                supports_lora: false,
                supports_mask: false,
                supports_outpaint: false,
                supports_video_mask: false,
                supports_reference_images: false,
                supports_audio: false,
            },
            // SwiftVR 1-step Fast Restoration
            ModelDescriptor {
                handle: "swiftvr-5b".to_string(),
                aliases: vec!["swiftvr".to_string(), "h-oliday/swiftvr".to_string()],
                family: ModelFamily::SwiftVR,
                default_repo_id: "H-oliday/SwiftVR".to_string(),
                default_quantization: QuantizationMode::Bf16,
                default_width: 1280,
                default_height: 720,
                default_frames: None,
                default_fps: None,
                default_steps: 1, // 1-step fast restore!
                default_guidance: 1.0,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::VideoToVideo],
                supports_lora: false,
                supports_mask: false,
                supports_outpaint: false,
                supports_video_mask: false,
                supports_reference_images: false,
                supports_audio: false,
            },
            // Depth Pro Monocular Depth
            ModelDescriptor {
                handle: "depth-pro".to_string(),
                aliases: vec!["apple/depth-pro".to_string()],
                family: ModelFamily::DepthPro,
                default_repo_id: "apple/DepthPro".to_string(),
                default_quantization: QuantizationMode::Fp16,
                default_width: 1536,
                default_height: 1536,
                default_frames: None,
                default_fps: None,
                default_steps: 1,
                default_guidance: 1.0,
                default_flow_shift: None,
                supported_tasks: vec![TaskType::ImageToImage],
                supports_lora: false,
                supports_mask: false,
                supports_outpaint: false,
                supports_video_mask: false,
                supports_reference_images: false,
                supports_audio: false,
            },
        ]
    }

    pub fn find(handle_or_repo: &str) -> Option<ModelDescriptor> {
        Self::all_models().into_iter().find(|m| m.matches(handle_or_repo))
    }
}
