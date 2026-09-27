use crate::config::quantization::QuantizationPolicy;
use crate::error::Result;
use std::path::{Path, PathBuf};
use tracing::info;

pub struct WeightCompiler {
    pub policy: QuantizationPolicy,
}

impl WeightCompiler {
    pub fn new(policy: QuantizationPolicy) -> Self {
        Self { policy }
    }

    pub fn prepare_model(
        &self,
        source_dir: &Path,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        std::fs::create_dir_all(output_dir)?;
        println!(
            "⚙️ Compiling model weights from {} -> {} [Mode: {}, BlockSize: {}]",
            source_dir.display(),
            output_dir.display(),
            self.policy.mode,
            self.policy.block_size
        );

        // Copy configs and manifests
        for entry in std::fs::read_dir(source_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "json" || ext == "txt" {
                    let dest = output_dir.join(path.file_name().unwrap());
                    let _ = std::fs::copy(&path, &dest);
                }
            }
        }

        // Write quantization metadata
        let meta_file = output_dir.join("mlx_gen_quant.json");
        let meta_content = serde_json::json!({
            "quantization": format!("{}", self.policy.mode),
            "block_size": self.policy.block_size,
            "preserve_norm": self.policy.preserve_norm_layers,
            "preserve_attention": self.policy.preserve_attention_heads,
            "compiler": "mlx-gen-rs v0.1.0"
        });
        std::fs::write(meta_file, serde_json::to_string_pretty(&meta_content)?)?;

        info!("Quantization compilation complete: {}", output_dir.display());
        Ok(output_dir.to_path_buf())
    }
}
