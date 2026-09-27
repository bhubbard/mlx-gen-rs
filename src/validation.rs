use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationProfile {
    pub profile_id: String,
    pub model_handle: String,
    pub task: String,
    pub prompt: String,
    pub width: u32,
    pub height: u32,
    pub steps: u32,
    pub seed: u64,
    pub expected_checksum: String,
}

pub struct ValidationRegistry;

impl ValidationRegistry {
    pub fn all_profiles() -> Vec<ValidationProfile> {
        vec![
            ValidationProfile {
                profile_id: "wan2.2-ti2v-5b-smoke".to_string(),
                model_handle: "wan2.2-ti2v-5b".to_string(),
                task: "text-to-video".to_string(),
                prompt: "A neon robot walking in a cyberpunk rain alley".to_string(),
                width: 832,
                height: 480,
                steps: 5,
                seed: 42,
                expected_checksum: "a3f90bc192...".to_string(),
            },
            ValidationProfile {
                profile_id: "flux-schnell-smoke".to_string(),
                model_handle: "flux-schnell".to_string(),
                task: "text-to-image".to_string(),
                prompt: "A cinematic portrait of an astronaut on Mars".to_string(),
                width: 512,
                height: 512,
                steps: 4,
                seed: 1337,
                expected_checksum: "f84b72110c...".to_string(),
            },
            ValidationProfile {
                profile_id: "seedvr2-3b-smoke".to_string(),
                model_handle: "seedvr2-3b".to_string(),
                task: "image-to-image".to_string(),
                prompt: "".to_string(),
                width: 1024,
                height: 1024,
                steps: 5,
                seed: 2026,
                expected_checksum: "d19024bc91...".to_string(),
            },
        ]
    }
}
