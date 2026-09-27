pub mod capabilities;
pub mod config;
pub mod error;
pub mod models;
pub mod pipeline;
pub mod storage;
pub mod validation;

pub use capabilities::{CapabilitiesReport, CAPABILITIES_SCHEMA_VERSION};
pub use config::{
    model_config::{ModelDescriptor, ModelFamily, ModelRegistry},
    quantization::{QuantizationMode, QuantizationPolicy},
    task::{CanvasPolicy, GenerationDimensions, OutpaintFillMode, TaskType},
};
pub use error::{MlxGenError, Result};
pub use models::{
    DepthProEstimator, FluxImageGenerator, MiniMaxGenerator, QwenEditGenerator, RestorationEngine,
    WanVideoGenerator,
};
pub use pipeline::{
    events::{PipelineEvent, ProgressTracker},
    outpaint::{OutpaintCanvas, PaddingSpec},
    router::{GenerationRequest, ResolvedRoute, TaskRouter},
    scheduler::{FlowMatchScheduler, SchedulerType},
};
pub use storage::{ModelDownloader, SafeTensorFile, WeightCompiler};
pub use validation::{ValidationProfile, ValidationRegistry};
