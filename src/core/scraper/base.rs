//! 基础抓取器定义
//! 参考 Ruby 版本的基础抓取器实现

use crate::core::error::Result;
use crate::core::scraper::filter::Filter;
use async_trait::async_trait;
use std::collections::HashMap;

/// 抓取器特质定义
#[async_trait]
pub trait Scraper: Send + Sync {
    /// 获取抓取器名称
    fn name(&self) -> &str;

    /// 获取抓取器版本
    fn version(&self) -> &str;

    /// 运行抓取器
    async fn run(&mut self) -> Result<()>;
}

/// 基础抓取器配置
pub struct ScraperConfig {
    /// 文档名称
    pub name: String,
    /// 文档版本
    pub version: String,
    /// 文档归属信息（版权等）
    pub attribution: Option<String>,
    /// 基础URL
    pub base_url: String,
    /// 根路径
    pub root_path: String,
    /// 输出目录路径
    pub output_path: String,
    /// 初始路径列表
    pub initial_paths: Vec<String>,
    /// 根标题
    pub root_title: String,
    /// 相关链接
    pub links: HashMap<String, String>,
}

impl ScraperConfig {
    /// 创建新的抓取器配置
    pub fn new(name: &str, version: &str, base_url: &str, output_path: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            attribution: None,
            base_url: base_url.to_string(),
            root_path: "".to_string(),
            output_path: output_path.to_string(),
            initial_paths: Vec::new(),
            root_title: name.to_string(),
            links: HashMap::new(),
        }
    }

    /// 设置归属信息
    pub fn with_attribution(mut self, attribution: &str) -> Self {
        self.attribution = Some(attribution.to_string());
        self
    }

    /// 设置根路径
    pub fn with_root_path(mut self, root_path: &str) -> Self {
        self.root_path = root_path.to_string();
        self
    }

    /// 设置初始路径列表
    pub fn with_initial_paths(mut self, paths: Vec<String>) -> Self {
        self.initial_paths = paths;
        self
    }

    /// 设置根标题
    pub fn with_root_title(mut self, title: &str) -> Self {
        self.root_title = title.to_string();
        self
    }

    /// 添加链接
    pub fn with_link(mut self, key: &str, url: &str) -> Self {
        self.links.insert(key.to_string(), url.to_string());
        self
    }

    /// 批量添加链接
    pub fn with_links(mut self, links: HashMap<String, String>) -> Self {
        self.links.extend(links);
        self
    }
}

/// 基础抓取器特质
/// 
/// 所有特定文档类型的抓取器都应该实现这个特质
pub trait BaseScraper: Scraper {
    /// 获取抓取器配置
    fn config(&self) -> &ScraperConfig;
    
    /// 获取可变的抓取器配置
    fn config_mut(&mut self) -> &mut ScraperConfig;
    
    /// 获取过滤器列表
    fn filters(&self) -> &[Box<dyn Filter>];
    
    /// 获取可变的过滤器列表
    fn filters_mut(&mut self) -> &mut Vec<Box<dyn Filter>>;
    
    /// 添加过滤器
    fn add_filter(&mut self, filter: Box<dyn Filter>);
}