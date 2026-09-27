# mlx-gen-rs (`mlxgen`)

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-edition%202021-orange.svg)](Cargo.toml)
[![Apple Silicon](https://img.shields.io/badge/Apple%20Silicon-M1%20%7C%20M2%20%7C%20M3%20%7C%20M4%20%7C%20M5-black.svg?logo=apple)](https://apple.com)
[![Capabilities](https://img.shields.io/badge/schema%20version-16-06B6D4.svg)](#schema-v16-capabilities)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-38BDF8.svg)](https://code.brandonhubbard.com/mlx-gen-rs/)

**`mlx-gen-rs`** is a high-performance, native Rust generative image and video runtime for **Apple Silicon**.

It is a pure Rust fork and systems-level reimplementation of [**`lpalbou/mlx-gen`**](https://github.com/lpalbou/mlx-gen) (originating from [mflux](https://github.com/filipstrand/mflux) and the [AbstractFramework](https://abstractframework.ai/) ecosystem). It compiles down to a single zero-dependency native binary (`mlxgen`) and exposes an ergonomic, memory-safe library for building multimodal generative applications.

---

## ⚡ Key Highlights & Architecture

- **Zero-Python Runtime Overhead**: Native Rust execution engine eliminating Python interpreter overhead, GIL contention, and dynamic type resolution.
- **Apple Silicon Unified Memory Architecture**: Engineered specifically for Apple Silicon (M1/M2/M3/M4/M5 Pro, Max, and Ultra) leveraging unified memory zero-copy tensor buffers and Metal compute shaders.
- **Multimodal Video & Audio Generation**:
  - **Wan 2.2 & 2.1**: Text-to-Video (T2V), Image-to-Video (I2V), Video-to-Video (V2V), masked video editing, and Wan-VACE context conditioning.
  - **MiniMax-H3 / Hailuo**: Synchronized 24 fps video generation paired with an **aligned stereo AAC soundtrack** (ambient audio, orchestral score, and lip-synced speech).
- **Flagship Image Generation & Editing**:
  - **FLUX.1 & FLUX.2 Klein**: Text-to-Image (T2I), Image-to-Image (I2I), in-context generation, and source-locked outpainting with adaptive content-aware canvas padding.
  - **Qwen-Image & Qwen-Edit**: Prompt-guided instruction editing, 2509/2511 routing, and masked inpainting.
  - **Z-Image Turbo**: Sub-second distilled text-to-image synthesis.
- **Super-Resolution & Restoration (`mlxgen upscale`)**:
  - **SeedVR2**: 3B/7B multi-scale image and video restoration with Lanczos3 resampling and high-frequency edge recovery.
  - **SwiftVR**: 1-step real-time video restoration running at 40× the throughput of iterative diffusion backends.
- **Hardware-Aware Quantization (`mlxgen prepare`)**:
  - Mixed-precision policies (BF16, FP16, Q8, Q4) with sensitive layer preservation (preserving normalization layers, position embeddings, and attention projections).
- **Host GUI Integration**:
  - Full support for `--json-events` streaming stdout progress to orchestrators and node-based visual workflow environments like **AbstractFlow**.
  - Strict compliance with **Capabilities Schema v16** (ADR-0001 through ADR-0007).

---

## 📦 Supported Model Catalog

| Model Handle | Family | Supported Tasks | Default Resolution | Default Steps | Flow Shift |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `wan2.2-ti2v-5b` | Wan | T2V, I2V, V2V | 832 × 480 @ 16 fps (81 frames) | 40 | `3.0` |
| `wan2.2-t2v-14b` | Wan | T2V, I2V, V2V | 1280 × 720 @ 16 fps (81 frames) | 50 | `5.0` |
| `minimax-h3` | MiniMax | T2V, I2V (Audio-Synced) | 960 × 544 @ 24 fps (120 frames) | 30 | `4.0` |
| `flux-schnell` | FLUX | T2I, I2I | 1024 × 1024 | 4 | — |
| `flux-dev` | FLUX | T2I, I2I, Edit, Outpaint | 1024 × 1024 | 25 | — |
| `flux2-klein-4b` | FLUX.2 | T2I, I2I, Edit, Outpaint | 1024 × 1024 | 8 | — |
| `qwen-image-edit`| Qwen | T2I, I2I, Edit, Masked | 1024 × 1024 | 20 | — |
| `z-image-turbo` | Z-Image | T2I, I2I | 1024 × 1024 | 8 | — |
| `depth-pro` | Depth Pro | Depth Map Estimation | 1536 × 1536 | 1 | — |
| `seedvr2-3b` | SeedVR2 | Restoration (Image/Video) | 1920 × 1080 (up to 4x) | 15 | — |
| `swiftvr-5b` | SwiftVR | 1-Step Video Restoration | 1280 × 720 (1x) | 1 | — |

---

## 🚀 Installation & Build

### From Source (Cargo)
```bash
git clone https://github.com/bhubbard/mlx-gen-rs.git
cd mlx-gen-rs
cargo build --release
```
The compiled native executable will be located at `target/release/mlxgen`.

---

## 🛠️ Command-Line Interface (`mlxgen`)

### 1. Text-to-Image Generation
```bash
mlxgen generate \
  --model flux-schnell \
  --prompt "An obsidian monolith radiating bioluminescent neon cyan circuits, 8k, cinematic" \
  --steps 4 \
  --output monolith.png
```

### 2. Text-to-Video Generation (Wan 2.2)
```bash
mlxgen generate \
  --model wan2.2-ti2v-5b \
  --prompt "A soaring cybernetic bird gliding over neon city skyscrapers at twilight" \
  --width 832 --height 480 \
  --frames 81 --fps 16 \
  --output cyberbird.mp4
```

### 3. Image-to-Video Animation
```bash
mlxgen generate \
  --model wan2.2-ti2v-5b \
  --image landscape.png \
  --prompt "Camera zooms forward as gentle mist rolls through the mountains" \
  --output animated_landscape.mp4
```

### 4. Audio-Synchronized Video (MiniMax-H3)
```bash
mlxgen generate \
  --model minimax-h3 \
  --prompt "A jazz musician performing a saxophone solo under streetlights with ambient rainfall" \
  --output jazz_clip.mp4
```

### 5. Super-Resolution & Restoration (`mlxgen upscale`)
```bash
# 2x upscale using SeedVR2
mlxgen upscale \
  --model seedvr2-3b \
  --input photo.png \
  --scale 2 \
  --output photo_2x.png
```

### 6. Inspect Model Capabilities Matrix
```bash
# Human readable summary
mlxgen capabilities

# Machine-readable JSON output (Schema v16)
mlxgen capabilities --json
```

### 7. Compile Local Quantized Weights (`mlxgen prepare`)
```bash
mlxgen prepare \
  --model /path/to/source-model \
  --path /path/to/quantized-q8 \
  --quantize q8
```

---

## 💻 Programmatic Rust API

You can embed `mlx-gen` directly into your Rust applications:

```rust
use mlx_gen::{
    config::task::TaskType,
    pipeline::router::{GenerationRequest, TaskRouter},
    models::FluxImageGenerator,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> mlx_gen::Result<()> {
    let request = GenerationRequest {
        model: "flux-schnell".to_string(),
        task: TaskType::TextToImage,
        prompt: "A cyberpunk laboratory in Tokyo, dramatic lighting".to_string(),
        negative_prompt: None,
        images: vec![],
        videos: vec![],
        reference_images: vec![],
        mask: None,
        video_mask: None,
        width: Some(1024),
        height: Some(1024),
        frames: None,
        fps: None,
        steps: Some(4),
        guidance: Some(0.0),
        seed: Some(42),
        flow_shift: None,
        output_path: PathBuf::from("laboratory.png"),
        low_ram: false,
        json_events: false,
    };

    let route = TaskRouter::resolve(&request)?;
    let generator = FluxImageGenerator::new(route);
    generator.generate(&request.prompt, &request.output_path, false)?;

    println!("Image successfully created!");
    Ok(())
}
```

---

## 🧪 Testing & Verification

Run the comprehensive test suite:
```bash
cargo test
```

Test coverage includes:
- ✅ Automatic task inference (T2I, I2I, T2V, I2V, V2V, Edit, Restoration)
- ✅ Capabilities Schema v16 compliance
- ✅ Flow-matching Euler & UniPC multi-step scheduler math
- ✅ Quantization layer preservation heuristics
- ✅ Multi-axis outpaint padding and auto-splitting calculations

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

---

## 🙏 Acknowledgements

- **[lpalbou/mlx-gen](https://github.com/lpalbou/mlx-gen)** by Laurent Palbou & contributors
- **[mflux](https://github.com/filipstrand/mflux)** by Filip Strand
- **[AbstractFramework](https://abstractframework.ai/)**
- **Apple Silicon MLX Team**
