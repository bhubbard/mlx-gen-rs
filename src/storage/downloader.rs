use crate::config::ModelRegistry;
use crate::error::{MlxGenError, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::info;

pub struct ModelDownloader {
    client: reqwest::Client,
    cache_dir: PathBuf,
}

impl Default for ModelDownloader {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let cache_dir = home.join(".cache").join("mlx-gen").join("models");
        Self {
            client: reqwest::Client::builder()
                .user_agent("mlx-gen-rs/0.1.0 (Apple Silicon; Rust)")
                .build()
                .unwrap_or_default(),
            cache_dir,
        }
    }
}

impl ModelDownloader {
    pub fn new(cache_dir: Option<PathBuf>) -> Self {
        let cache_dir = cache_dir.unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".cache")
                .join("mlx-gen")
                .join("models")
        });
        Self {
            client: reqwest::Client::builder()
                .user_agent("mlx-gen-rs/0.1.0")
                .build()
                .unwrap_or_default(),
            cache_dir,
        }
    }

    pub fn resolve_local_or_cache(&self, model_handle: &str) -> PathBuf {
        let p = Path::new(model_handle);
        if p.exists() {
            return p.to_path_buf();
        }

        if let Some(desc) = ModelRegistry::find(model_handle) {
            let safe_name = desc.default_repo_id.replace('/', "--");
            let target = self.cache_dir.join(safe_name);
            if target.exists() {
                return target;
            }
        }

        let safe_name = model_handle.replace('/', "--");
        self.cache_dir.join(safe_name)
    }

    pub async fn download_repo_file(
        &self,
        repo_id: &str,
        filename: &str,
        target_dir: &Path,
    ) -> Result<PathBuf> {
        create_dir_all(target_dir)?;
        let target_file = target_dir.join(filename);
        if target_file.exists() {
            info!("File already exists: {}", target_file.display());
            return Ok(target_file);
        }

        let url = format!(
            "https://huggingface.co/{}/resolve/main/{}",
            repo_id, filename
        );
        info!("Downloading {} from {}", filename, url);

        let res = self.client.get(&url).send().await?;
        if !res.status().is_success() {
            return Err(MlxGenError::Custom(format!(
                "Failed to download {}: HTTP status {}",
                url,
                res.status()
            )));
        }

        let total_size = res.content_length().unwrap_or(0);
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
                .unwrap_or_else(|_| ProgressStyle::default_bar())
                .progress_chars("#>-"),
        );
        pb.set_message(filename.to_string());

        let mut file = File::create(&target_file)?;
        let mut stream = res.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
            pb.inc(chunk.len() as u64);
        }

        pb.finish_with_message(format!("Downloaded {}", filename));
        Ok(target_file)
    }

    pub async fn download_model(&self, model_handle: &str, target_path: Option<PathBuf>) -> Result<PathBuf> {
        let repo_id = if let Some(desc) = ModelRegistry::find(model_handle) {
            desc.default_repo_id
        } else {
            model_handle.to_string()
        };

        let dest = target_path.unwrap_or_else(|| {
            let safe_name = repo_id.replace('/', "--");
            self.cache_dir.join(safe_name)
        });

        create_dir_all(&dest)?;
        println!("📦 Downloading model '{}' (Repo: {}) -> {}", model_handle, repo_id, dest.display());

        // Download config.json & model weights
        let _ = self.download_repo_file(&repo_id, "config.json", &dest).await;
        let _ = self.download_repo_file(&repo_id, "model_index.json", &dest).await;

        Ok(dest)
    }
}
