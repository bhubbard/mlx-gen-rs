use crate::error::{MlxGenError, Result};
use memmap2::Mmap;
use safetensors::tensor::SafeTensors;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct SafeTensorFile {
    pub path: PathBuf,
    mmap: Mmap,
}

impl SafeTensorFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        if !path_buf.exists() {
            return Err(MlxGenError::ModelNotFound(path_buf));
        }

        let file = File::open(&path_buf)?;
        let mmap = unsafe { Mmap::map(&file)? };

        Ok(Self {
            path: path_buf,
            mmap,
        })
    }

    pub fn tensors(&self) -> Result<SafeTensors<'_>> {
        SafeTensors::deserialize(&self.mmap).map_err(MlxGenError::Safetensors)
    }

    pub fn tensor_names(&self) -> Result<Vec<String>> {
        let st = self.tensors()?;
        Ok(st.names().into_iter().map(|s| s.to_string()).collect())
    }

    pub fn inspect_summary(&self) -> Result<SafeTensorSummary> {
        let st = self.tensors()?;
        let mut total_bytes = 0usize;
        let mut tensor_count = 0usize;
        let mut dtypes = std::collections::HashSet::new();

        for name in st.names() {
            tensor_count += 1;
            let tensor = st.tensor(name)?;
            total_bytes += tensor.data().len();
            dtypes.insert(format!("{:?}", tensor.dtype()));
        }

        Ok(SafeTensorSummary {
            path: self.path.clone(),
            tensor_count,
            total_bytes,
            data_types: dtypes.into_iter().collect(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SafeTensorSummary {
    pub path: PathBuf,
    pub tensor_count: usize,
    pub total_bytes: usize,
    pub data_types: Vec<String>,
}
