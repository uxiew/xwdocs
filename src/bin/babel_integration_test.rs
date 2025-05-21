//! Babel 抓取器集成测试
//!
//! 这个测试文件比较单一简化版 Babel 抓取器的不同参数设置下的结果
//! 特别关注 URL 处理问题

use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use xwdoc::core::scraper::base::Scraper;
use xwdoc::docs::babel::BabelScraper;

/// 检查目录中是否存在格式错误的 URL
fn check_for_malformed_urls(dir_path: &str) -> Result<(usize, Vec<String>), Box<dyn Error>> {
    let mut malformed_count = 0;
    let mut malformed_examples = Vec::new();

    // 递归遍历目录
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // 递归检查子目录
            let (count, examples) = check_for_malformed_urls(path.to_str().unwrap())?;
            malformed_count += count;
            malformed_examples.extend(examples);
        } else if let Some(ext) = path.extension() {
            // 检查 HTML 文件
            if ext == "html" || ext == "htm" {
                let file = File::open(&path)?;
                let reader = BufReader::new(file);

                // 逐行检查文件内容
                for line in reader.lines() {
                    let line = line?;

                    // 检查格式错误的 URL (例如 https://babeljs.io/docs/https://github.com)
                    if line.contains("https://babeljs.io/docs/http")
                        || (line.contains("://") && line.matches("://").count() > 1)
                    {
                        malformed_count += 1;

                        // 保存前几个示例用于报告
                        if malformed_examples.len() < 5 {
                            malformed_examples.push(format!(
                                "File: {}, Line: {}",
                                path.display(),
                                line
                            ));
                        }
                    }
                }
            }
        }
    }

    Ok((malformed_count, malformed_examples))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Babel Scraper Improvement Test ===");

    // 测试输出目录
    let version1_output = "./test_docs/babel_v1";
    let version2_output = "./test_docs/babel_v2";

    // 确保测试目录存在
    fs::create_dir_all(version1_output)?;
    fs::create_dir_all(version2_output)?;

    // 运行第一个版本的 Babel 抓取器
    println!("\n>> Running Babel scraper with version 6...");
    let mut version1_scraper = BabelScraper::new(version1_output, "6");
    version1_scraper.run().await?;

    // 运行第二个版本的 Babel 抓取器
    println!("\n>> Running Babel scraper with version 7...");
    let mut version2_scraper = BabelScraper::new(version2_output, "7");
    version2_scraper.run().await?;

    // 分析结果
    println!("\n>> Analyzing results...");

    // 检查版本1中的格式错误的 URL
    let (version1_count, version1_examples) = check_for_malformed_urls(version1_output)?;
    println!("\nVersion 6 scraper malformed URLs: {}", version1_count);
    if !version1_examples.is_empty() {
        println!("Examples:");
        for example in version1_examples {
            println!("  {}", example);
        }
    }

    // 检查版本2中的格式错误的 URL
    let (version2_count, version2_examples) = check_for_malformed_urls(version2_output)?;
    println!("\nVersion 7 scraper malformed URLs: {}", version2_count);
    if !version2_examples.is_empty() {
        println!("Examples:");
        for example in version2_examples {
            println!("  {}", example);
        }
    }

    // 报告两个版本的区别
    println!("\n>> Comparison results:");
    println!("Version 6 malformed URLs: {}", version1_count);
    println!("Version 7 malformed URLs: {}", version2_count);

    if version1_count > 0 || version2_count > 0 {
        println!("\nNote: Check the examples above for details on the malformed URLs.");
    } else {
        println!("\nNo malformed URLs found in either version. Good job!");
    }

    // 计算改进百分比
    let improvement = if version1_count > 0 {
        ((version1_count as f64 - version2_count as f64) / version1_count as f64) * 100.0
    } else {
        0.0
    };

    println!("\nImprovement: {:.2}%", improvement);

    if version2_count == 0 {
        println!("\n✅ Success! The improved Babel scraper has fixed all malformed URL issues.");
    } else {
        println!("\n⚠️ The improved scraper has reduced malformed URLs but some issues remain.");
    }

    Ok(())
}
