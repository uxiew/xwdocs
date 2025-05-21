//! Rust 文档抓取器

use crate::core::error::Result;
use crate::core::filters::{HtmlCleanerFilter, UrlNormalizerFilter};
use crate::core::scraper::{Scraper as CoreScraper, UrlScraper};
use async_trait::async_trait;

/// Rust文档抓取器
pub struct RustScraper {
    /// 基础抓取器
    scraper: UrlScraper,
}

impl RustScraper {
    /// 创建新的Rust文档抓取器
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://doc.rust-lang.org/";
        let mut scraper = UrlScraper::new("Rust", version, base_url, output_path);

        // 添加初始路径
        let initial_paths = vec![
            "std/index.html".to_string(),
            "book/index.html".to_string(),
            "reference/index.html".to_string(),
            "cargo/index.html".to_string(),
            "rustc/index.html".to_string(),
        ];

        // 创建过滤器
        let html_cleaner = Box::new(
            HtmlCleanerFilter::new()
                .with_remove_tag("footer")
                .with_remove_tag("nav")
                .with_remove_attr("data-*"),
        );
        let url_normalizer = Box::new(UrlNormalizerFilter::new(base_url, "/docs/rust/"));

        // 添加过滤器和初始路径
        scraper = scraper
            .with_initial_paths(initial_paths)
            .with_filter(html_cleaner)
            .with_filter(url_normalizer);

        Self { scraper }
    }
}

#[async_trait]
impl CoreScraper for RustScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取Rust文档...");
        self.scraper.run().await
    }
}
