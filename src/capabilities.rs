use crate::config::ModelRegistry;
use serde::{Deserialize, Serialize};

pub const CAPABILITIES_SCHEMA_VERSION: u32 = 16;
pub const UNIVERSAL_GENERATE_OPTIONS: &[&str] = &["--low-ram"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilityRow {
    pub model: String,
    pub aliases: Vec<String>,
    pub family: String,
    pub repo_id: String,
    pub supported_tasks: Vec<String>,
    pub supports_lora: bool,
    pub supports_mask: bool,
    pub supports_outpaint: bool,
    pub supports_video_mask: bool,
    pub supports_reference_images: bool,
    pub supports_audio: bool,
    pub default_width: u32,
    pub default_height: u32,
    pub default_frames: Option<u32>,
    pub default_fps: Option<f32>,
    pub default_steps: u32,
    pub default_flow_shift: Option<f32>,
    pub outpaint_preservation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorationCapabilityRow {
    pub model: String,
    pub family: String,
    pub repo_id: String,
    pub default_scale: u32,
    pub supported_inputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesReport {
    pub schema_version: u32,
    pub universal_generate_options: Vec<String>,
    pub capabilities: Vec<ModelCapabilityRow>,
    pub restoration: Vec<RestorationCapabilityRow>,
}

impl CapabilitiesReport {
    pub fn current() -> Self {
        let mut capabilities = Vec::new();
        let mut restoration = Vec::new();

        for desc in ModelRegistry::all_models() {
            let fam_str = format!("{:?}", desc.family).to_lowercase();
            if fam_str == "seedvr2" || fam_str == "swiftvr" {
                let inputs = if fam_str == "swiftvr" {
                    vec!["video".to_string()]
                } else {
                    vec!["image".to_string(), "video".to_string()]
                };
                restoration.push(RestorationCapabilityRow {
                    model: desc.handle.clone(),
                    family: fam_str,
                    repo_id: desc.default_repo_id.clone(),
                    default_scale: 2,
                    supported_inputs: inputs,
                });
            } else {
                capabilities.push(ModelCapabilityRow {
                    model: desc.handle.clone(),
                    aliases: desc.aliases.clone(),
                    family: fam_str,
                    repo_id: desc.default_repo_id.clone(),
                    supported_tasks: desc.supported_tasks.iter().map(|t| t.to_string()).collect(),
                    supports_lora: desc.supports_lora,
                    supports_mask: desc.supports_mask,
                    supports_outpaint: desc.supports_outpaint,
                    supports_video_mask: desc.supports_video_mask,
                    supports_reference_images: desc.supports_reference_images,
                    supports_audio: desc.supports_audio,
                    default_width: desc.default_width,
                    default_height: desc.default_height,
                    default_frames: desc.default_frames,
                    default_fps: desc.default_fps,
                    default_steps: desc.default_steps,
                    default_flow_shift: desc.default_flow_shift,
                    outpaint_preservation: if desc.supports_outpaint {
                        Some("adaptive-content-aware-source-blend".to_string())
                    } else {
                        None
                    },
                });
            }
        }

        Self {
            schema_version: CAPABILITIES_SCHEMA_VERSION,
            universal_generate_options: UNIVERSAL_GENERATE_OPTIONS.iter().map(|s| s.to_string()).collect(),
            capabilities,
            restoration,
        }
    }
}
