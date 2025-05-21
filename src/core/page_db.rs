//! 页面数据库模块
//!
//! 参考原始 Ruby 项目中的 page_db.rb 实现
//! 提供页面内容的存储和检索功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 页面数据库结构体
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PageDb {
    /// 页面映射，键为页面路径，值为页面内容
    pages: HashMap<String, String>,
}

impl PageDb {
    /// 创建新的页面数据库
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    /// 添加页面到数据库
    pub fn add(&mut self, path: String, content: String) {
        self.pages.insert(path, content);
    }

    /// 从数据库中获取页面内容
    pub fn get(&self, path: &str) -> Option<&String> {
        self.pages.get(path)
    }

    /// 检查页面是否存在
    pub fn has(&self, path: &str) -> bool {
        self.pages.contains_key(path)
    }

    /// 从 JSON 字符串加载页面数据库
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        let pages: HashMap<String, String> = serde_json::from_str(json)?;
        Ok(Self { pages })
    }

    /// 将页面数据库转换为 JSON 字符串
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.pages)
    }

    /// 将页面数据库转换为美化的 JSON 字符串
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self.pages)
    }

    /// 检查数据库是否为空
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// 获取页面数量
    pub fn len(&self) -> usize {
        self.pages.len()
    }

    /// 获取所有页面路径
    pub fn paths(&self) -> Vec<&String> {
        self.pages.keys().collect()
    }

    /// 获取所有页面内容
    pub fn contents(&self) -> Vec<&String> {
        self.pages.values().collect()
    }

    /// 获取所有页面的键值对
    pub fn entries(&self) -> impl Iterator<Item = (&String, &String)> {
        self.pages.iter()
    }

    /// 获取可变的页面映射
    pub fn pages_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.pages
    }

    /// 获取不可变的页面映射
    pub fn pages(&self) -> &HashMap<String, String> {
        &self.pages
    }

    /// 清空数据库
    pub fn clear(&mut self) {
        self.pages.clear();
    }

    /// 移除页面
    pub fn remove(&mut self, path: &str) -> Option<String> {
        self.pages.remove(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_db() {
        let mut db = PageDb::new();
        
        // 添加页面
        db.add("path/to/page".to_string(), "content".to_string());
        
        // 测试获取
        assert_eq!(db.get("path/to/page"), Some(&"content".to_string()));
        assert_eq!(db.has("path/to/page"), true);
        assert_eq!(db.has("nonexistent"), false);
        
        // 测试长度
        assert_eq!(db.len(), 1);
        assert_eq!(db.is_empty(), false);
        
        // 测试路径和内容
        assert_eq!(db.paths(), vec![&"path/to/page".to_string()]);
        assert_eq!(db.contents(), vec![&"content".to_string()]);
        
        // 测试 JSON 序列化和反序列化
        let json = db.to_json().unwrap();
        let loaded = PageDb::from_json(&json).unwrap();
        assert_eq!(loaded.get("path/to/page"), Some(&"content".to_string()));
        
        // 测试移除
        db.remove("path/to/page");
        assert_eq!(db.has("path/to/page"), false);
        assert_eq!(db.is_empty(), true);
    }
}
