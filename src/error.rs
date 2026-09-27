use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MlxGenError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization/Deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Safetensors error: {0}")]
    Safetensors(#[from] safetensors::SafeTensorError),

    #[error("Network download error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Model configuration error: {0}")]
    ModelConfig(String),

    #[error("Task inference error: {0}")]
    TaskInference(String),

    #[error("Route resolution error: {0}")]
    RouteResolution(String),

    #[error("Quantization error: {0}")]
    Quantization(String),

    #[error("LoRA adapter application error: {0}")]
    LoraApplication(String),

    #[error("Outpaint error: {0}")]
    Outpaint(String),

    #[error("Device/Metal execution error: {0}")]
    DeviceExecution(String),

    #[error("Model file not found at path: {0}")]
    ModelNotFound(PathBuf),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("{0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, MlxGenError>;
