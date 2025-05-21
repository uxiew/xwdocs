//! u57fau4e8eu6587u4ef6u7cfbu7edfu7684u5b58u50a8u5b9eu73b0

use super::store::Store;
use crate::core::error::{Error, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// u57fau4e8eu6587u4ef6u7cfbu7edfu7684u5b58u50a8
pub struct FileStore {
    /// u57fau7840u8def u5f84
    root_path: PathBuf,
}

impl FileStore {
    /// u521bu5efau65b0u7684u6587u4ef6u5b58u50a8
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        // u786eu4fddu8def u5f84u5b58u5728
        let path = root_path.as_ref().to_path_buf();
        fs::create_dir_all(&path).expect("Failed to create directory");

        Self { root_path: path }
    }

    /// u83b7u53d6u5b8cu6574u8def u5f84
    fn full_path(&self, path: &str) -> PathBuf {
        self.root_path.join(path)
    }
}

impl Store for FileStore {
    fn read(&self, path: &str) -> Result<String> {
        let full_path = self.full_path(path);
        let mut file = File::open(&full_path).map_err(Error::Io)?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(Error::Io)?;

        Ok(content)
    }

    fn write(&self, path: &str, content: &str) -> Result<()> {
        let full_path = self.full_path(path);

        // u786eu4fddu76eeu5f55u5b58u5728
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).map_err(Error::Io)?;
        }

        let mut file = File::create(&full_path).map_err(Error::Io)?;

        file.write_all(content.as_bytes())
            .map_err(Error::Io)?;

        Ok(())
    }

    fn exists(&self, path: &str) -> Result<bool> {
        let full_path = self.full_path(path);
        Ok(full_path.exists())
    }

    fn list(&self, dir: &str) -> Result<Vec<String>> {
        let full_path = self.full_path(dir);

        if !full_path.exists() {
            return Ok(Vec::new());
        }

        if !full_path.is_dir() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{} is not a directory", dir),
            )));
        }

        let base_path = self.root_path.to_string_lossy().to_string();
        let entries = fs::read_dir(&full_path)
            .map_err(Error::Io)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                let path_str = path.to_string_lossy().to_string();

                // u5c06u7edddu5bf9u8def u5f84u8f6cu6362u4e3au76f8u5bf9u4e8eu6839u76eeu5f55u7684u8def u5f84
                if path_str.starts_with(&base_path) {
                    Some(path_str[base_path.len() + 1..].to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(entries)
    }

    fn delete(&self, path: &str) -> Result<()> {
        let full_path = self.full_path(path);

        if !full_path.exists() {
            return Ok(());
        }

        if full_path.is_dir() {
            fs::remove_dir_all(&full_path).map_err(Error::Io)?;
        } else {
            fs::remove_file(&full_path).map_err(Error::Io)?;
        }

        Ok(())
    }
    
    fn size(&self, path: &str) -> Result<usize> {
        let full_path = self.full_path(path);
        
        if !full_path.exists() {
            return Ok(0);
        }
        
        let metadata = fs::metadata(&full_path).map_err(Error::Io)?;
        Ok(metadata.len() as usize)
    }
}
