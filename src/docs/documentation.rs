//! 提供单个文档的结构

use crate::core::types::{ModifiedTime, Release, Size, Slug, Version};

/// 表示单个文档
pub struct Documentation {
    /// 文档名称
    pub name: String,
    /// 文档唯一标识（URL友好用）
    pub slug: Slug,
    /// 文档版本
    pub version: Version,
    /// 文档发布版本
    pub release: Release,
    /// 上次修改时间（UNIX时间戳）
    pub mtime: ModifiedTime,
    /// 文档数据库大小
    pub db_size: Size,
    /// 索引大小
    pub index_size: Size,
}

impl Documentation {
    /// 创建新的文档对象
    pub fn new(name: &str, slug: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            slug: slug.to_string(),
            version: version.to_string(),
            release: String::new(),
            mtime: 0,
            db_size: 0,
            index_size: 0,
        }
    }

    /// 获取文档的完整路径
    pub fn path(&self) -> String {
        if self.version.is_empty() {
            self.slug.clone()
        } else {
            format!("{}~{}", self.slug, self.version)
        }
    }

    /// 设置发布版本
    pub fn with_release(mut self, release: &str) -> Self {
        self.release = release.to_string();
        self
    }

    /// 设置修改时间
    pub fn with_mtime(mut self, mtime: ModifiedTime) -> Self {
        self.mtime = mtime;
        self
    }

    /// 设置数据库大小
    pub fn with_db_size(mut self, size: Size) -> Self {
        self.db_size = size;
        self
    }

    /// 设置索引大小
    pub fn with_index_size(mut self, size: Size) -> Self {
        self.index_size = size;
        self
    }

    /// 获取完整名称（包含版本）
    pub fn full_name(&self) -> String {
        if self.version.is_empty() {
            self.name.clone()
        } else {
            format!("{} {}", self.name, self.version)
        }
    }
}
