//! HTML documentation scraper
//!
//! 严格按照原版 Ruby 实现转换的 HTML 文档抓取器
//! 参考文件: lib/docs/scrapers/mdn/html.rb

use crate::core::error::Result;
use crate::core::scraper::{Scraper as CoreScraper, UrlScraper};
use crate::docs::html::HtmlEntriesFilter;
use crate::core::filters::{HtmlCleanerFilter, UrlNormalizerFilter};
use async_trait::async_trait;

/// HTML文档爬虫
pub struct HtmlScraper {
    /// 基础爬虫
    scraper: UrlScraper,
}

impl HtmlScraper {
    /// 创建新的HTML文档爬虫（仅抓取页面和图片，过滤无用资源）
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://developer.mozilla.org/en-US/docs/Web/HTML";
        let mut scraper = UrlScraper::new("HTML", version, base_url, output_path);

        // 只抓取首页、元素、全局属性等主要入口
        let initial_paths = vec![
            "/".to_string(),
            "/Element".to_string(),
            "/Global_attributes".to_string(),
        ];

        // 过滤器：
        // 1. 清理无用标签
        let html_cleaner = Box::new(HtmlCleanerFilter::new());
        // 2. 只允许页面和图片链接
        let html_entries = Box::new(HtmlEntriesFilter::new());
        // 3. 规范化链接
        let url_normalizer = Box::new(UrlNormalizerFilter::new(base_url, "/docs/html/"));

        // 组合过滤器和初始路径
        scraper = scraper
            .with_initial_paths(initial_paths)
            .with_filter(html_cleaner)
            .with_filter(url_normalizer)
            .with_filter(html_entries);

        Self { scraper }
    }
}

#[async_trait]
impl CoreScraper for HtmlScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取HTML文档...");
        self.scraper.run().await
    }
}
