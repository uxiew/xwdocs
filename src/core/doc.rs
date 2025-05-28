//! Doc 模块
//!
//! 参考原始 Ruby 项目中的 doc.rb 实现
//! 提供文档的基本属性和操作功能

// Define a new trait for objects that can be serialized to JSON for the store_index method
trait ToJsonOutput {
    fn to_json_output(&mut self) -> String;
}

use crate::core::error::Result;
use crate::core::index_entry::{FullIndex, IndexEntry, IndexType};
use crate::storage::store::Store;
use serde::{Deserialize, Serialize};
// use serde_json::json; // Removed unused import
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH}; // SystemTime and UNIX_EPOCH are used in store_meta

/// 常量定义
pub const INDEX_FILENAME: &str = "index.json";
pub const DB_FILENAME: &str = "db.json";
pub const META_FILENAME: &str = "meta.json";

/// 文档元数据结构
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocMeta {
    pub name: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub doc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_size: Option<usize>,
}

/// 页面数据库，存储页面路径和内容的映射
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageDb {
    pages: HashMap<String, String>,
}

impl PageDb {
    /// 创建新的 PageDb 实例
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    /// 添加页面
    pub fn add(&mut self, path: String, content: String) {
        self.pages.insert(path, content);
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// 获取页面数量
    pub fn len(&self) -> usize {
        self.pages.len()
    }

    /// 转换为 JSON 字符串
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.pages).unwrap_or_else(|_| "{}".to_string())
    }
}

// Implement ToJsonOutput for PageDb
impl ToJsonOutput for PageDb {
    fn to_json_output(&mut self) -> String {
        // PageDb::to_json takes &self, which is fine as &mut self can coerce to &self.
        self.to_json()
    }
}

/// 条目索引，管理条目和类型
#[derive(Clone, Debug)]
pub struct EntryIndex {
    entries: Vec<IndexEntry>,
    index: HashSet<String>,
    types: HashMap<String, IndexType>,
}

impl EntryIndex {
    /// 创建新的 EntryIndex 实例
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: HashSet::new(),
            types: HashMap::new(),
        }
    }

    /// 添加条目
    pub fn add(&mut self, entry: IndexEntry) {
        let entry_json = serde_json::to_string(&entry).unwrap_or_default();
        
        if self.index.insert(entry_json) {
            if let Some(entry_type) = self.types.get_mut(&entry.entry_type) {
                entry_type.count += 1;
            } else {
                self.types.insert(
                    entry.entry_type.clone(),
                    IndexType {
                        name: entry.entry_type.clone(),
                        count: 1,
                        slug: entry.entry_type.to_lowercase(),
                    },
                );
            }
            self.entries.push(entry);
        }
    }

    /// 添加多个条目
    pub fn add_multiple(&mut self, entries: Vec<IndexEntry>) {
        for entry in entries {
            self.add(entry);
        }
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 获取条目数量
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 转换为完整索引结构
    pub fn to_full_index(&mut self) -> FullIndex {
        // 按照名称排序条目
        self.entries.sort_by(|a, b| sort_entries(&a.name, &b.name));
        
        // 转换类型列表并排序
        let mut types: Vec<_> = self.types.values().cloned().collect();
        types.sort_by(|a, b| sort_entries(&a.name, &b.name));
        
        FullIndex {
            entries: self.entries.clone(),
            types,
        }
    }

    /// 转换为 JSON 字符串
    pub fn to_json(&mut self) -> String {
        serde_json::to_string(&self.to_full_index()).unwrap_or_else(|_| "{}".to_string())
    }
}

// Implement ToJsonOutput for EntryIndex
impl ToJsonOutput for EntryIndex {
    fn to_json_output(&mut self) -> String {
        self.to_json()
    }
}

/// 文档接口，定义文档需要实现的基本方法
pub trait Doc {
    /// 获取文档名称
    fn name(&self) -> &str;
    
    /// 获取文档 slug
    fn slug(&self) -> &str;
    
    /// 获取文档类型
    fn doc_type(&self) -> &str;
    
    /// 获取文档版本
    fn version(&self) -> Option<&str> {
        None
    }
    
    /// 获取发布信息
    fn release(&self) -> Option<&str> {
        None
    }
    
    /// 获取文档链接
    fn links(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    
    /// 获取文档路径
    fn path(&self) -> String {
        self.slug().to_string()
    }
    
    /// 获取索引文件路径
    fn index_path(&self) -> String {
        format!("{}/{}", self.path(), INDEX_FILENAME)
    }
    
    /// 获取数据库文件路径
    fn db_path(&self) -> String {
        format!("{}/{}", self.path(), DB_FILENAME)
    }
    
    /// 获取元数据文件路径
    fn meta_path(&self) -> String {
        format!("{}/{}", self.path(), META_FILENAME)
    }
    
    /// 构建单个页面
    fn build_page(&self, id: &str) -> Result<Option<HashMap<String, serde_json::Value>>>;
    
    /// 构建所有页面
    fn build_pages<F>(&self, callback: F) -> Result<()>
    where
        F: FnMut(HashMap<String, serde_json::Value>);
    
    /// 获取抓取器版本
    fn get_scraper_version(&self, opts: &HashMap<String, String>) -> Result<String>;
    
    /// 获取最新版本
    fn get_latest_version(&self, opts: &HashMap<String, String>) -> Result<String>;
    
    /// 转换为 JSON 格式的元数据
    fn as_json(&self) -> DocMeta {
        DocMeta {
            name: self.name().to_string(),
            slug: self.slug().to_string(),
            doc_type: self.doc_type().to_string(),
            version: self.version().map(String::from),
            release: self.release().map(String::from),
            links: self.links(),
            mtime: None,
            db_size: None,
        }
    }
    
    /// 存储单个页面
    fn store_page(&self, store: &mut dyn Store, id: &str) -> Result<bool> {
        let mut index = EntryIndex::new();
        let mut pages = PageDb::new();
        
        if let Some(page) = self.build_page(id)? {
            if let Some(entries) = page.get("entries").and_then(|e| e.as_array()) {
                // 处理并添加条目
                for entry in entries {
                    if let Ok(entry) = serde_json::from_value::<IndexEntry>(entry.clone()) {
                        index.add(entry);
                    }
                }
                
                if !index.is_empty() {
                    let path = page.get("path").and_then(|p| p.as_str()).unwrap_or("");
                    let output = page.get("output").and_then(|o| o.as_str()).unwrap_or("");
                    let store_path = page.get("store_path").and_then(|p| p.as_str()).unwrap_or("");
                    
                    pages.add(path.to_string(), output.to_string());
                    self.store_index(store, INDEX_FILENAME, &mut index, false)?;
                    self.store_index(store, DB_FILENAME, &mut pages, false)?;
                    store.write(store_path, output)?;
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// 存储所有页面
    fn store_pages(&self, store: &mut dyn Store) -> Result<bool> {
        let mut index = EntryIndex::new();
        let mut pages = PageDb::new();
        
        self.build_pages(|page| {
            if let Some(entries) = page.get("entries").and_then(|e| e.as_array()) {
                // 处理并添加条目
                let mut has_entries = false;
                for entry in entries {
                    if let Ok(entry) = serde_json::from_value::<IndexEntry>(entry.clone()) {
                        index.add(entry);
                        has_entries = true;
                    }
                }
                
                if has_entries {
                    let path = page.get("path").and_then(|p| p.as_str()).unwrap_or("");
                    let output = page.get("output").and_then(|o| o.as_str()).unwrap_or("");
                    let store_path = page.get("store_path").and_then(|p| p.as_str()).unwrap_or("");
                    
                    store.write(store_path, output).unwrap_or(());
                    pages.add(path.to_string(), output.to_string());
                }
            }
        })?;
        
        if !index.is_empty() {
            self.store_index(store, INDEX_FILENAME, &mut index, true)?;
            self.store_index(store, DB_FILENAME, &mut pages, true)?;
            self.store_meta(store)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// 存储索引
    fn store_index<T>(&self, store: &mut dyn Store, filename: &str, index: &mut T, read_write: bool) -> Result<()>
    where
        T: ToJsonOutput, // Use the new ToJsonOutput trait for the constraint
    {
        let old_json = if read_write {
            store.read(filename).unwrap_or_else(|_| "{}".to_string())
        } else {
            "{}".to_string()
        };
        
        let new_json = index.to_json_output(); // Call the method from ToJsonOutput trait
        
        // TODO: 实现 instrument 功能
        // instrument(format!("{}.doc", filename.replacen(".json", "", 1)), old_json, new_json);
        
        if read_write {
            store.write(filename, &new_json)?;
        }
        
        Ok(())
    }
    
    /// 存储元数据
    fn store_meta(&self, store: &mut dyn Store) -> Result<()> {
        let mut meta = self.as_json();
        meta.mtime = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        
        meta.db_size = Some(store.size(DB_FILENAME)?);
        let json = serde_json::to_string(&meta)?;
        
        store.write(META_FILENAME, &json)?;
        
        Ok(())
    }
    
    /// 判断文档版本状态
    fn outdated_state(&self, scraper_version: &str, latest_version: &str) -> String {
        let scraper_parts: Vec<_> = scraper_version
            .split(|c| c == '-' || c == '.')
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .collect();
        
        let latest_parts: Vec<_> = latest_version
            .split(|c| c == '-' || c == '.')
            .map(|s| s.parse::<u32>().unwrap_or(0))
            .collect();
        
        // 只检查前两部分，第三部分是补丁更新
        for i in 0..2 {
            if i >= scraper_parts.len() || i >= latest_parts.len() {
                break;
            }
            
            if i == 0 && latest_parts[i] > scraper_parts[i] {
                return "Outdated major version".to_string();
            }
            
            if i == 1 && latest_parts[i] > scraper_parts[i] {
                if (latest_parts[0] == 0 && scraper_parts[0] == 0) || 
                   (latest_parts[0] == 1 && scraper_parts[0] == 1) {
                    return "Outdated major version".to_string();
                }
                return "Outdated minor version".to_string();
            }
            
            if latest_parts[i] < scraper_parts[i] {
                return "Up-to-date".to_string();
            }
        }
        
        "Up-to-date".to_string()
    }
}

/// 辅助函数 - 分割整数
fn split_ints(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut last_was_digit = false;
    
    for c in s.chars() {
        let is_digit = c.is_digit(10);
        
        if is_digit && !last_was_digit && !current.is_empty() {
            result.push(current);
            current = String::new();
        }
        
        current.push(c);
        last_was_digit = is_digit;
    }
    
    if !current.is_empty() {
        result.push(current);
    }
    
    result
}

/// 条目排序函数
fn sort_entries(a: &str, b: &str) -> std::cmp::Ordering {
    let a_first = a.chars().next().map(|c| c.is_digit(10)).unwrap_or(false);
    let b_first = b.chars().next().map(|c| c.is_digit(10)).unwrap_or(false);
    
    if a_first || b_first {
        let a_split = split_ints(a);
        let b_split = split_ints(b);
        
        let a_len = a_split.len();
        let b_len = b_split.len();
        
        if a_len == 1 && b_len == 1 {
            return a.to_lowercase().cmp(&b.to_lowercase());
        }
        
        if a_len == 1 {
            return std::cmp::Ordering::Greater;
        }
        
        if b_len == 1 {
            return std::cmp::Ordering::Less;
        }
        
        // 处理数值部分
        let mut a_processed: Vec<_> = a_split.iter().enumerate()
            .map(|(i, s)| {
                if i == a_len - 1 {
                    s.to_string()
                } else {
                    s.parse::<u32>().unwrap_or(0).to_string()
                }
            })
            .collect();
        
        let mut b_processed: Vec<_> = b_split.iter().enumerate()
            .map(|(i, s)| {
                if i == b_len - 1 {
                    s.to_string()
                } else {
                    s.parse::<u32>().unwrap_or(0).to_string()
                }
            })
            .collect();
        
        // 调整长度使之相等
        if b_len > a_len {
            for _ in 0..(b_len - a_len) {
                a_processed.insert(a_processed.len() - 1, "0".to_string());
            }
        } else if a_len > b_len {
            for _ in 0..(a_len - b_len) {
                b_processed.insert(b_processed.len() - 1, "0".to_string());
            }
        }
        
        a_processed.cmp(&b_processed)
    } else {
        a.to_lowercase().cmp(&b.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sort_entries() {
        // 测试普通字符串排序
        assert_eq!(sort_entries("a", "b"), std::cmp::Ordering::Less);
        assert_eq!(sort_entries("B", "a"), std::cmp::Ordering::Greater);
        
        // 测试带数字的排序
        assert_eq!(sort_entries("1.1", "1.2"), std::cmp::Ordering::Less);
        assert_eq!(sort_entries("1.10", "1.2"), std::cmp::Ordering::Greater);
        assert_eq!(sort_entries("2.1", "1.2"), std::cmp::Ordering::Greater);
        
        // 测试混合内容的排序
        assert_eq!(sort_entries("item", "1.item"), std::cmp::Ordering::Greater);
        assert_eq!(sort_entries("1.item", "2.item"), std::cmp::Ordering::Less);
    }
    
    #[test]
    fn test_split_ints() {
        // 测试数字分割
        assert_eq!(split_ints("1.2"), vec!["1", ".2"]);
        assert_eq!(split_ints("10.20.30"), vec!["10", ".20", ".30"]);
        assert_eq!(split_ints("v1.2.3"), vec!["v1", ".2", ".3"]);
    }
}
