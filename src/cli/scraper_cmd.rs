//! 文档抓取命令处理

use std::error::Error;

/// 列出所有可用的抓取器
pub fn list_scrapers() -> Result<(), Box<dyn Error>> {
    println!("可用的文档抓取器:");
    println!("  babel - Babel 文档抓取器");
    println!("  html - HTML 文档抓取器");
    println!("  css - CSS 文档抓取器");
    println!("  javascript - JavaScript 文档抓取器");
    println!("  typescript - TypeScript 文档抓取器");
    println!("  rust - Rust 文档抓取器");
    println!("  url - 通用URL抓取器 (需要指定URL)");
    
    Ok(())
}

/// 运行指定的抓取器
pub async fn run_scraper(name: &str, version: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
    println!("运行抓取器: {} (版本: {})", name, version);
    
    let output_str = output.unwrap_or("");
    
    crate::scrape_async(name, version, output_str).await
}
