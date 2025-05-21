//! css documentation scraper
//!
//! 严格按照原版 Ruby 实现转换的 css 文档抓取器
//! 参考文件: lib/docs/scrapers/mdn/css.rb

use crate::core::error::Result;
use crate::core::filters::{HtmlCleanerFilter, UrlNormalizerFilter};
use crate::core::scraper::{Scraper as CoreScraper, UrlScraper};
use async_trait::async_trait;

/// CSS文档爬虫
pub struct CssScraper {
    /// 基础爬虫
    scraper: UrlScraper,
}

impl CssScraper {
    /// 创建新的CSS文档爬虫
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://developer.mozilla.org/en-US/docs/Web/CSS";
        let mut scraper = UrlScraper::new("CSS", version, base_url, output_path);

        // 添加初始路径
        let initial_paths = vec![
            "/".to_string(),
            "/Reference".to_string(),
            "/Selectors".to_string(),
        ];

        // 创建过滤器
        let html_cleaner = Box::new(
            HtmlCleanerFilter::new()
                .with_remove_tag("footer")
                .with_remove_tag("nav"),
        );
        let url_normalizer = Box::new(UrlNormalizerFilter::new(base_url, "/docs/css/"));

        // 添加过滤器和初始路径
        scraper = scraper
            .with_initial_paths(initial_paths)
            .with_filter(html_cleaner)
            .with_filter(url_normalizer);

        Self { scraper }
    }
}

#[async_trait]
impl CoreScraper for CssScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取CSS文档...");
        self.scraper.run().await
    }
}
