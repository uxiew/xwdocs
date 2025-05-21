//! Babel documentation scraper
//!
//! 严格按照原版 Ruby 实现转换的 Babel 文档抓取器
//! 参考文件: lib/docs/scrapers/babel.rb

use crate::core::error::Result;
use crate::core::scraper::base::Scraper;
use crate::core::scraper::url_scraper::UrlScraper;
use crate::docs::babel::{BabelCleanHtmlFilter, BabelEntriesFilter};
use async_trait::async_trait;

/// Babel documentation scraper
///
/// 使用UrlScraper作为基类，更接近Ruby原版实现，易于扩展
pub struct BabelScraper {
    /// The underlying URL scraper
    scraper: UrlScraper,
}

impl BabelScraper {
    /// Create a new Babel scraper for the specified version
    pub fn new(output_path: &str, version: &str) -> Self {
        // Resolve version to a specific version number
        let resolved_version = if version == "6" {
            "6.26.1"
        } else {
            // Default to latest v7 for empty string, "7", or any other value
            "7.21.4"
        };

        // 配置文档抓取器基础参数
        let mut scraper = UrlScraper::new(
            "Babel",
            resolved_version,
            "https://babeljs.io/docs/",
            output_path,
        ); // 设置根标题
        scraper = scraper.with_root_title("Babel");

        // 设置归属信息
        scraper = scraper.with_attribution(
            "© 2014-present Sebastian McKenzie<br>Licensed under the MIT License.",
        );

        // 设置相关链接
        scraper = scraper.with_string_links(vec![
            ("home".to_string(), "https://babeljs.io/".to_string()),
            (
                "code".to_string(),
                "https://github.com/babel/babel".to_string(),
            ),
        ]);

        // 配置路径末尾斜杠
        scraper = scraper.with_trailing_slash(true);

        // 配置跳过模式 - 直接从原始Ruby代码复制
        scraper = scraper.with_skip_patterns(vec![
            "usage/.*",
            "configuration/.*",
            "learn/.*",
            "v7-migration/.*",
            "v7-migration-api/.*",
            "editors/.*",
            "presets/.*",
            "caveats/.*",
            "faq/.*",
            "roadmap/.*",
        ]);

        // 设置跳过链接函数 - 与原始代码保持一致
        scraper = scraper
            .with_skip_link(|href: &str| -> bool { href.contains("https://babeljs.io/docs/en/") });

        // 添加过滤器 - 严格按照原始Ruby代码的顺序
        scraper = scraper.with_filter(Box::new(BabelCleanHtmlFilter::new()));
        scraper = scraper.with_filter(Box::new(BabelEntriesFilter::new()));

        Self { scraper }
    }

    /// 获取最新版本（类似于Ruby原版的get_latest_version方法）
    pub async fn get_latest_version(&self) -> Result<String> {
        // 具体实现可以使用 GitHub API 获取最新发布版本
        // 这里暂时返回一个固定版本作为示例
        Ok("7.21.4".to_string())
    }
}

#[async_trait]
impl Scraper for BabelScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("Starting Babel documentation scraping...");
        self.scraper.run().await
    }
}
