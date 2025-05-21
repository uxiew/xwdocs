//! 清单模块
//!
//! 参考原始 Ruby 项目中的 manifest.rb 实现
//! 提供文档清单的管理功能

use crate::core::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 文档清单结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// 所有文档的集合，键为 slug
    docs: HashMap<String, DocSpec>,
}

/// 文档规格结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSpec {
    /// 文档名称
    pub name: String,
    /// 文档 slug
    pub slug: String,
    /// 文档类型
    #[serde(rename = "type")]
    pub doc_type: String,
    /// 文档版本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 发布信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    /// 文档链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, String>>,
    /// 最后修改时间
    pub mtime: u64,
    /// 数据库大小
    pub db_size: usize,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            docs: HashMap::new(),
        }
    }
}

impl Manifest {
    /// 创建新的清单
    pub fn new() -> Self {
        Self::default()
    }

    /// 加载来自 JSON 字符串的清单
    pub fn from_json(json: &str) -> Result<Self> {
        let manifest: Self = serde_json::from_str(json)?;
        Ok(manifest)
    }

    /// 将清单转换为 JSON 字符串
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }

    /// 将清单转换为美化的 JSON 字符串
    pub fn to_json_pretty(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(&self)?;
        Ok(json)
    }

    /// 添加文档到清单
    pub fn add(&mut self, doc: DocSpec) {
        self.docs.insert(doc.slug.clone(), doc);
    }

    /// 从清单中移除文档
    pub fn remove(&mut self, slug: &str) -> Option<DocSpec> {
        self.docs.remove(slug)
    }

    /// 获取清单中的所有文档
    pub fn docs(&self) -> Vec<&DocSpec> {
        self.docs.values().collect()
    }

    /// 获取可变的文档集合
    pub fn docs_mut(&mut self) -> &mut HashMap<String, DocSpec> {
        &mut self.docs
    }

    /// 获取特定的文档
    pub fn get(&self, slug: &str) -> Option<&DocSpec> {
        self.docs.get(slug)
    }

    /// 获取特定文档的可变引用
    pub fn get_mut(&mut self, slug: &str) -> Option<&mut DocSpec> {
        self.docs.get_mut(slug)
    }

    /// 按类型分组文档
    pub fn docs_by_type(&self) -> HashMap<String, Vec<DocSpec>> {
        let mut result = HashMap::new();

        for doc in self.docs.values() {
            result
                .entry(doc.doc_type.clone())
                .or_insert_with(Vec::new)
                .push(doc.clone());
        }

        // 按名称排序每个类型中的文档
        for docs in result.values_mut() {
            docs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        }

        result
    }

    /// 清空清单
    pub fn clear(&mut self) {
        self.docs.clear();
    }

    /// 检查清单是否为空
    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }

    /// 获取文档数量
    pub fn len(&self) -> usize {
        self.docs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest() {
        let mut manifest = Manifest::new();
        
        // 添加文档
        let doc = DocSpec {
            name: "Rust".to_string(),
            slug: "rust".to_string(),
            doc_type: "programming".to_string(),
            version: Some("1.0.0".to_string()),
            release: Some("2023".to_string()),
            links: Some(HashMap::new()),
            mtime: 123456789,
            db_size: 1000,
        };
        
        manifest.add(doc.clone());
        
        // 测试获取
        let retrieved = manifest.get("rust").unwrap();
        assert_eq!(retrieved.name, "Rust");
        assert_eq!(retrieved.version, Some("1.0.0".to_string()));
        
        // 测试分组
        let grouped = manifest.docs_by_type();
        assert_eq!(grouped.len(), 1);
        assert!(grouped.contains_key("programming"));
        assert_eq!(grouped["programming"].len(), 1);
        
        // 测试 JSON 序列化
        let json = manifest.to_json().unwrap();
        assert!(json.contains("Rust"));
        
        // 测试 JSON 反序列化
        let loaded = Manifest::from_json(&json).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded.get("rust").unwrap().name, "Rust");
    }
}
