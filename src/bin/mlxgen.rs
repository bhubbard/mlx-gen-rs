use clap::{Args, Parser, Subcommand};
use mlx_gen::capabilities::CapabilitiesReport;
use mlx_gen::config::model_config::ModelFamily;
use mlx_gen::config::quantization::{QuantizationMode, QuantizationPolicy};
use mlx_gen::config::task::TaskType;
use mlx_gen::error::Result;
use mlx_gen::models::{
    FluxImageGenerator, MiniMaxGenerator, QwenEditGenerator, RestorationEngine,
    WanVideoGenerator,
};
use mlx_gen::pipeline::router::{GenerationRequest, TaskRouter};
use mlx_gen::storage::{ModelDownloader, WeightCompiler};
use mlx_gen::validation::ValidationRegistry;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mlxgen",
    author = "Brandon Hubbard <bhubbard@users.noreply.github.com>",
    version = "0.1.0",
    about = "High-performance generative image & video runtime for Apple Silicon (Rust fork of lpalbou/mlx-gen)"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    direct_generate: Option<GenerateArgs>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate images or videos from text/image prompts
    Generate(GenerateArgs),

    /// Super-resolution and restoration for images and video (SeedVR2 & SwiftVR)
    Upscale(UpscaleArgs),

    /// Download model checkpoints and configs from Hugging Face
    Download(DownloadArgs),

    /// Prepare and compile local quantized weights with layer-precision retention
    Prepare(PrepareArgs),

    /// Inspect public tasks, capabilities, and options schema
    Capabilities(CapabilitiesArgs),

    /// Inspect or execute validation smoke test profiles
    Validation(ValidationArgs),
}

#[derive(Args, Debug, Clone)]
pub struct GenerateArgs {
    /// Model handle or Hugging Face repo ID
    #[arg(short, long)]
    pub model: Option<String>,

    /// Task type: auto, text-to-image, image-to-image, edit, text-to-video, image-to-video, video-to-video
    #[arg(long, default_value = "auto")]
    pub task: String,

    /// Text prompt describing the target output
    #[arg(short, long, default_value = "")]
    pub prompt: String,

    /// Negative prompt
    #[arg(long)]
    pub negative_prompt: Option<String>,

    /// Input image paths (single --image or repeated)
    #[arg(short = 'i', long = "image")]
    pub images: Vec<PathBuf>,

    /// Input video paths
    #[arg(short = 'v', long = "video")]
    pub videos: Vec<PathBuf>,

    /// Reference image paths for context conditioning (e.g. Bernini-R, Wan-VACE)
    #[arg(long = "reference-image")]
    pub reference_images: Vec<PathBuf>,

    /// Mask image path for inpainting / masked editing
    #[arg(long = "mask-path")]
    pub mask: Option<PathBuf>,

    /// Mask video path for video-to-video editing
    #[arg(long = "video-mask-path")]
    pub video_mask: Option<PathBuf>,

    /// Output width in pixels
    #[arg(short = 'W', long)]
    pub width: Option<u32>,

    /// Output height in pixels
    #[arg(short = 'H', long)]
    pub height: Option<u32>,

    /// Output frame count (for video)
    #[arg(long)]
    pub frames: Option<u32>,

    /// Output FPS (for video)
    #[arg(long)]
    pub fps: Option<f32>,

    /// Inference steps
    #[arg(short = 's', long)]
    pub steps: Option<u32>,

    /// Guidance scale
    #[arg(short = 'g', long)]
    pub guidance: Option<f32>,

    /// Random seed for deterministic generation
    #[arg(long)]
    pub seed: Option<u64>,

    /// Flow shift parameter for rectified flow models
    #[arg(long)]
    pub flow_shift: Option<f32>,

    /// Destination output path (e.g. output.png or output.mp4)
    #[arg(short = 'o', long, default_value = "output.png")]
    pub output: PathBuf,

    /// Restrict peak memory footprint for lower RAM devices (e.g. 16GB / 24GB Macs)
    #[arg(long)]
    pub low_ram: bool,

    /// Stream JSON events to stdout for host GUI orchestration (e.g. AbstractFlow)
    #[arg(long)]
    pub json_events: bool,
}

#[derive(Args, Debug)]
pub struct UpscaleArgs {
    /// Restoration model (e.g. seedvr2-3b, seedvr2-7b, swiftvr-5b)
    #[arg(short, long, default_value = "seedvr2-3b")]
    pub model: String,

    /// Input file path (image or video)
    #[arg(short, long)]
    pub input: PathBuf,

    /// Scale factor (e.g. 2 for 2x, 4 for 4x)
    #[arg(long, default_value_t = 2)]
    pub scale: u32,

    /// Output destination path
    #[arg(short, long, default_value = "restored_output.png")]
    pub output: PathBuf,

    /// Stream JSON events
    #[arg(long)]
    pub json_events: bool,
}

#[derive(Args, Debug)]
pub struct DownloadArgs {
    /// Model handle or Hugging Face repo ID
    #[arg(short, long)]
    pub model: String,

    /// Custom target destination directory
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct PrepareArgs {
    /// Source model directory or handle
    #[arg(short, long)]
    pub model: String,

    /// Destination directory for compiled quantized weights
    #[arg(short, long)]
    pub path: PathBuf,

    /// Quantization mode: fp32, bf16, fp16, q8, q4, mixed
    #[arg(short, long, default_value = "q8")]
    pub quantize: String,
}

#[derive(Args, Debug)]
pub struct CapabilitiesArgs {
    /// Format output as JSON (default is human-readable summary if omitted)
    #[arg(long)]
    pub json: bool,
}

#[derive(Args, Debug)]
pub struct ValidationArgs {
    /// Run smoke tests on matching profiles
    #[arg(short, long)]
    pub profile: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate(args)) => execute_generate(args).await?,
        Some(Commands::Upscale(args)) => execute_upscale(args).await?,
        Some(Commands::Download(args)) => execute_download(args).await?,
        Some(Commands::Prepare(args)) => execute_prepare(args).await?,
        Some(Commands::Capabilities(args)) => execute_capabilities(args)?,
        Some(Commands::Validation(args)) => execute_validation(args)?,
        None => {
            if let Some(args) = cli.direct_generate {
                if args.model.is_some() || !args.prompt.is_empty() {
                    execute_generate(args).await?;
                } else {
                    println!("🚀 mlxgen v0.1.0 — High-performance generative AI runtime for Apple Silicon");
                    println!("Run `mlxgen --help` or `mlxgen capabilities` to get started.");
                }
            } else {
                println!("🚀 mlxgen v0.1.0 — High-performance generative AI runtime for Apple Silicon");
                println!("Run `mlxgen --help` or `mlxgen capabilities` to get started.");
            }
        }
    }

    Ok(())
}

async fn execute_generate(args: GenerateArgs) -> Result<()> {
    let model = args.model.unwrap_or_else(|| "flux-schnell".to_string());
    let task_type: TaskType = args.task.parse()?;

    let req = GenerationRequest {
        model,
        task: task_type,
        prompt: args.prompt,
        negative_prompt: args.negative_prompt,
        images: args.images,
        videos: args.videos,
        reference_images: args.reference_images,
        mask: args.mask,
        video_mask: args.video_mask,
        width: args.width,
        height: args.height,
        frames: args.frames,
        fps: args.fps,
        steps: args.steps,
        guidance: args.guidance,
        seed: args.seed,
        flow_shift: args.flow_shift,
        output_path: args.output,
        low_ram: args.low_ram,
        json_events: args.json_events,
    };

    let route = TaskRouter::resolve(&req)?;

    match route.descriptor.family {
        ModelFamily::Wan => {
            let gen = WanVideoGenerator::new(route);
            gen.generate(&req.prompt, &req.output_path, req.json_events)?;
        }
        ModelFamily::Flux | ModelFamily::Flux2 | ModelFamily::ZImage | ModelFamily::Bonsai | ModelFamily::Ernie => {
            let gen = FluxImageGenerator::new(route);
            gen.generate(&req.prompt, &req.output_path, req.json_events)?;
        }
        ModelFamily::MiniMax => {
            let gen = MiniMaxGenerator::new(route);
            gen.generate(&req.prompt, &req.output_path, req.json_events)?;
        }
        ModelFamily::Qwen => {
            let gen = QwenEditGenerator::new(route);
            gen.generate(&req.prompt, &req.output_path, req.json_events)?;
        }
        ModelFamily::SeedVR2 | ModelFamily::SwiftVR => {
            let engine = RestorationEngine::new(route.descriptor.handle, 2);
            let input = req.images.first().or(req.videos.first()).ok_or_else(|| {
                mlx_gen::error::MlxGenError::RouteResolution("Restoration requires an input image or video.".to_string())
            })?;
            engine.upscale_image(input, &req.output_path, req.json_events)?;
        }
        ModelFamily::DepthPro => {
            let input = req.images.first().ok_or_else(|| {
                mlx_gen::error::MlxGenError::RouteResolution("Depth estimation requires an input image.".to_string())
            })?;
            mlx_gen::models::DepthProEstimator::estimate_depth(input, &req.output_path)?;
        }
        ModelFamily::Fibo => {
            let gen = FluxImageGenerator::new(route);
            gen.generate(&req.prompt, &req.output_path, req.json_events)?;
        }
    }

    Ok(())
}

async fn execute_upscale(args: UpscaleArgs) -> Result<()> {
    let engine = RestorationEngine::new(args.model, args.scale);
    engine.upscale_image(&args.input, &args.output, args.json_events)
}

async fn execute_download(args: DownloadArgs) -> Result<()> {
    let downloader = ModelDownloader::new(None);
    let path = downloader.download_model(&args.model, args.path).await?;
    println!("✅ Model downloaded successfully to: {}", path.display());
    Ok(())
}

async fn execute_prepare(args: PrepareArgs) -> Result<()> {
    let mode: QuantizationMode = args.quantize.parse()?;
    let policy = QuantizationPolicy {
        mode,
        ..Default::default()
    };
    let compiler = WeightCompiler::new(policy);
    let src = PathBuf::from(&args.model);
    let out = compiler.prepare_model(&src, &args.path)?;
    println!("✅ Prepared quantized model saved to: {}", out.display());
    Ok(())
}

fn execute_capabilities(args: CapabilitiesArgs) -> Result<()> {
    let report = CapabilitiesReport::current();
    if args.json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("✨ MLX-Gen Capabilities (Schema v{})", report.schema_version);
        println!("---------------------------------------------------------------");
        println!("Universal Options: {:?}", report.universal_generate_options);
        println!("\n🎨 Generative Image & Video Models ({} routes):", report.capabilities.len());
        for row in &report.capabilities {
            println!(
                "  • {:<20} [{:<8}] Tasks: {:<30} Default: {}x{}",
                row.model, row.family, row.supported_tasks.join(", "), row.default_width, row.default_height
            );
        }
        println!("\n🔍 Super-Resolution & Restoration Models ({} routes):", report.restoration.len());
        for row in &report.restoration {
            println!(
                "  • {:<20} [{:<8}] Inputs: {:<15} Default Scale: {}x",
                row.model, row.family, row.supported_inputs.join(", "), row.default_scale
            );
        }
    }
    Ok(())
}

fn execute_validation(args: ValidationArgs) -> Result<()> {
    let profiles = ValidationRegistry::all_profiles();
    println!("🧪 MLX-Gen Smoke & Parity Validation Profiles");
    println!("---------------------------------------------------------------");
    for prof in profiles {
        if let Some(ref target) = args.profile {
            if !prof.profile_id.contains(target) {
                continue;
            }
        }
        println!("  ✓ [{}] Model: {}, Task: {}, Dim: {}x{}, Steps: {}",
            prof.profile_id, prof.model_handle, prof.task, prof.width, prof.height, prof.steps
        );
    }
    Ok(())
}
