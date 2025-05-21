//! 文档注册表管理

use super::Documentation;
use crate::core::error::Result;

/// 管理可用文档的注册表
pub struct DocRegistry {
    docs: Vec<Documentation>,
}

impl DocRegistry {
    /// 创建新的空注册表
    pub fn new() -> Self {
        Self { docs: Vec::new() }
    }

    /// 添加文档到注册表
    pub fn add(&mut self, doc: Documentation) {
        self.docs.push(doc);
    }

    /// 获取所有可用文档
    pub fn all(&self) -> &[Documentation] {
        &self.docs
    }

    /// 通过别名查找文档
    pub fn find(&self, slug: &str) -> Option<&Documentation> {
        self.docs.iter().find(|doc| doc.slug == slug)
    }

    /// 通过别名和版本查找文档
    pub fn find_with_version(&self, slug: &str, version: &str) -> Option<&Documentation> {
        self.docs
            .iter()
            .find(|doc| doc.slug == slug && doc.version == version)
    }

    /// 加载所有文档从磁盘
    pub fn load_from_disk(&mut self, path: &str) -> Result<()> {
        use crate::core::error::Error;
        use std::fs;
        use std::path::Path;
        use std::time::UNIX_EPOCH;

        let base_path = Path::new(path);
        if !base_path.exists() {
            return Err(Error::Message(format!("文档路径不存在: {}", path)).into());
        }

        // 清空当前文档列表
        self.docs.clear();

        // 遍历文档目录
        let entries = match fs::read_dir(base_path) {
            Ok(entries) => entries,
            Err(e) => return Err(Error::Message(format!("无法读取文档目录: {}", e)).into()),
        };

        for entry_result in entries {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry_path = entry.path();
            if !entry_path.is_dir() {
                continue;
            }

            // 获取文档信息
            if let Some(dirname) = entry_path.file_name().and_then(|n| n.to_str()) {
                // 解析目录名
                let (slug, version) = if dirname.contains('~') {
                    let parts: Vec<&str> = dirname.split('~').collect();
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    (dirname.to_string(), String::new())
                };

                // 尝试读取index.json和meta.json
                let index_path = entry_path.join("index.json");
                let meta_path = entry_path.join("meta.json");
                let db_path = entry_path.join("db.json");

                if !index_path.exists() || !db_path.exists() {
                    continue;
                }

                // 提取基本信息
                let index_size = fs::metadata(&index_path)
                    .map(|m| m.len() as usize)
                    .unwrap_or(0);
                let db_size = fs::metadata(&db_path)
                    .map(|m| m.len() as usize)
                    .unwrap_or(0);

                // 获取修改时间
                let mtime = match fs::metadata(&entry_path) {
                    Ok(metadata) => match metadata.modified() {
                        Ok(modified_time) => match modified_time.duration_since(UNIX_EPOCH) {
                            Ok(duration) => duration.as_secs(),
                            Err(_) => 0,
                        },
                        Err(_) => 0,
                    },
                    Err(_) => 0,
                };

                // 读取元数据
                let mut doc = Documentation::new(&slug, &slug, &version)
                    .with_mtime(mtime)
                    .with_db_size(db_size)
                    .with_index_size(index_size);

                // 尝试读取元数据文件
                if meta_path.exists() {
                    if let Ok(meta_content) = fs::read_to_string(&meta_path) {
                        if let Ok(meta_json) =
                            serde_json::from_str::<serde_json::Value>(&meta_content)
                        {
                            if let Some(release) = meta_json.get("release").and_then(|v| v.as_str())
                            {
                                doc = doc.with_release(release);
                            }
                            if let Some(name) = meta_json.get("name").and_then(|v| v.as_str()) {
                                doc.name = name.to_string();
                            }
                        }
                    }
                }

                // 添加到注册表
                self.add(doc);
            }
        }

        Ok(())
    }

    /// 生成清单JSON
    pub fn generate_manifest(&self, path: &str) -> Result<()> {
        use crate::core::error::Error;
        use serde_json::{json, to_string_pretty};
        use std::fs;
        use std::path::Path;

        let manifest_path = Path::new(path).join("manifest.json");

        // 创建JSON数组
        let docs_json: Vec<serde_json::Value> = self
            .docs
            .iter()
            .map(|doc| {
                json!({
                    "name": doc.name,
                    "slug": doc.slug,
                    "version": doc.version,
                    "release": doc.release,
                    "mtime": doc.mtime,
                    "db_size": doc.db_size,
                    "index_size": doc.index_size
                })
            })
            .collect();

        // 创建整体JSON
        let manifest = json!({
            "docs": docs_json,
            "generated_at": chrono::Utc::now().timestamp()
        });

        // 写入文件
        let content = to_string_pretty(&manifest)
            .map_err(|e| Error::Message(format!("无法序列化清单JSON: {}", e)))?;

        fs::write(&manifest_path, content)
            .map_err(|e| Error::Message(format!("无法写入清单文件: {}", e)))?;

        Ok(())
    }
}

impl Default for DocRegistry {
    fn default() -> Self {
        Self::new()
    }
}
