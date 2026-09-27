use crate::error::{MlxGenError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QuantizationMode {
    Fp32,
    Bf16,
    Fp16,
    Q8,
    Q4,
    Mixed,
}

impl QuantizationMode {
    pub fn bits(&self) -> u32 {
        match self {
            QuantizationMode::Fp32 => 32,
            QuantizationMode::Bf16 | QuantizationMode::Fp16 => 16,
            QuantizationMode::Q8 => 8,
            QuantizationMode::Q4 => 4,
            QuantizationMode::Mixed => 8, // Typical effective average
        }
    }
}

impl fmt::Display for QuantizationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantizationMode::Fp32 => write!(f, "fp32"),
            QuantizationMode::Bf16 => write!(f, "bf16"),
            QuantizationMode::Fp16 => write!(f, "fp16"),
            QuantizationMode::Q8 => write!(f, "q8"),
            QuantizationMode::Q4 => write!(f, "q4"),
            QuantizationMode::Mixed => write!(f, "mixed"),
        }
    }
}

impl FromStr for QuantizationMode {
    type Err = MlxGenError;

    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_lowercase().as_str() {
            "32" | "fp32" | "float32" => Ok(QuantizationMode::Fp32),
            "16" | "fp16" | "float16" => Ok(QuantizationMode::Fp16),
            "bf16" | "bfloat16" => Ok(QuantizationMode::Bf16),
            "8" | "q8" | "int8" => Ok(QuantizationMode::Q8),
            "4" | "q4" | "int4" => Ok(QuantizationMode::Q4),
            "mixed" | "adaptive" => Ok(QuantizationMode::Mixed),
            other => Err(MlxGenError::Quantization(format!(
                "Unsupported quantization '{}'. Supported: fp32, bf16, fp16, q8, q4, mixed",
                other
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationPolicy {
    pub mode: QuantizationMode,
    pub block_size: usize,
    pub preserve_attention_heads: bool,
    pub preserve_norm_layers: bool,
    pub preserve_embeddings: bool,
    pub excluded_layer_patterns: Vec<String>,
}

impl Default for QuantizationPolicy {
    fn default() -> Self {
        Self {
            mode: QuantizationMode::Bf16,
            block_size: 64,
            preserve_attention_heads: true,
            preserve_norm_layers: true,
            preserve_embeddings: true,
            excluded_layer_patterns: vec![
                "norm".to_string(),
                "time_embed".to_string(),
                "pos_embed".to_string(),
            ],
        }
    }
}

impl QuantizationPolicy {
    pub fn q8_balanced() -> Self {
        Self {
            mode: QuantizationMode::Q8,
            block_size: 64,
            preserve_attention_heads: true,
            preserve_norm_layers: true,
            preserve_embeddings: true,
            excluded_layer_patterns: vec![
                "norm".to_string(),
                "bias".to_string(),
                "embed".to_string(),
            ],
        }
    }

    pub fn q4_aggressive() -> Self {
        Self {
            mode: QuantizationMode::Q4,
            block_size: 32,
            preserve_attention_heads: false,
            preserve_norm_layers: true,
            preserve_embeddings: true,
            excluded_layer_patterns: vec!["norm".to_string(), "embed".to_string()],
        }
    }

    pub fn should_preserve_tensor(&self, tensor_name: &str) -> bool {
        if self.preserve_norm_layers && (tensor_name.contains("norm") || tensor_name.contains("ln_")) {
            return true;
        }
        if self.preserve_embeddings && tensor_name.contains("embed") {
            return true;
        }
        self.excluded_layer_patterns.iter().any(|pattern| tensor_name.contains(pattern))
    }
}
