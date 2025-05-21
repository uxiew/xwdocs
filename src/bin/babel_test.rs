//! Test program to run the Babel scraper

use xwdoc::core::error::Result;
use xwdoc::core::scraper::base::Scraper;
use xwdoc::docs::babel::BabelScraper;

#[tokio::main]
async fn main() -> Result<()> {
    println!("创建 Babel 抓取器...");
    // 创建文档输出路径
    let output_path = "./test_docs/babel";
    
    // 确保输出目录存在
    if !std::path::Path::new(output_path).exists() {
        std::fs::create_dir_all(output_path)?;
    }
    
    let mut scraper = BabelScraper::new(output_path, "7");
    
    println!("运行 Babel 抓取器...");
    scraper.run().await?;
    
    println!("Babel 抓取器成功完成!");
    Ok(())
}
