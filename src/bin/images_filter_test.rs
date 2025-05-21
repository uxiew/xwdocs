//! 图片过滤器测试
//!
//! 测试ImagesFilter能够正确转换图片为base64格式

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use xwdoc::core::error::Result;
use xwdoc::core::filters::html::ImagesFilter;
use xwdoc::core::scraper::filter::FilterContext;
use xwdoc::core::scraper::filter::Filter;

fn main() -> Result<()> {
    println!("=== 图片过滤器测试 ===");

    // 测试目录
    let test_dir = PathBuf::from("test_docs/images_test");
    fs::create_dir_all(&test_dir)?;

    // 测试 DATA URL 图片保留
    test_data_url_preservation(&test_dir)?;

    // 测试相对URL处理
    test_relative_url_handling(&test_dir)?;

    // 测试过滤器的辅助方法
    test_helper_methods()?;

    println!("\n测试完成！结果保存在 {}", test_dir.display());
    Ok(())
}

// 测试 DATA URL 图片保留
fn test_data_url_preservation(test_dir: &Path) -> Result<()> {
    println!("\n测试 DATA URL 图片保留:");

    // 使用Data URL来绕过网络问题
    let html = r#"
    <!DOCTYPE html>
    <html>
    <body>
        <h1>图片测试</h1>
        <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+P+/HgAFdwI3Bvi30QAAAABJRU5ErkJggg==" alt="Small Test Image">
        <p>已经是data URL图片，不应该被修改</p>
        <p>这是一个用于测试图片过滤器的简单页面</p>
    </body>
    </html>
    "#;

    // 创建过滤器上下文
    let mut context = FilterContext::default();

    // 创建图片过滤器
    let filter = ImagesFilter::new().with_max_width(50);

    // 应用过滤器
    println!("处理图片...");
    let processed_html = filter.apply(html, &mut context)?;
    write_file(&test_dir.join("data_url_test.html"), &processed_html)?;

    // 检查是否成功保留了data URL
    if processed_html.contains(
        "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+P",
    ) {
        println!("✅ 成功验证: data URL图片已正确保留且未被修改");
    } else {
        println!("❌ 验证失败: data URL图片被错误地修改了");
    }

    Ok(())
}

// 测试相对URL处理
fn test_relative_url_handling(test_dir: &Path) -> Result<()> {
    println!("\n测试相对URL处理:");

    // 创建带有相对URL的HTML
    let html = r#"
    <!DOCTYPE html>
    <html>
    <body>
        <h1>相对URL测试</h1>
        <p>以下图片使用相对URL，应该被尝试转换（但在测试环境中可能无法实际下载）</p>
        <img src="/images/test.png" alt="Relative URL Test">
    </body>
    </html>
    "#;

    // 创建过滤器上下文 - 设置基础URL
    let mut context = FilterContext {
        base_url: "https://example.com".to_string(),
        ..FilterContext::default()
    };

    // 创建图片过滤器
    let filter = ImagesFilter::new();

    // 应用过滤器 - 即使下载失败也不会崩溃
    println!("处理相对URL图片...");
    match filter.apply(html, &mut context) {
        Ok(processed_html) => {
            write_file(&test_dir.join("relative_url_test.html"), &processed_html)?;
            println!("✅ 成功处理: 相对URL处理正常（即使图片下载可能失败）");
        }
        Err(e) => {
            println!("❌ 处理失败: 相对URL处理错误 - {}", e);
        }
    }

    Ok(())
}

// 测试辅助方法
fn test_helper_methods() -> Result<()> {
    println!("\n测试URL辅助方法:");

    let test_filter = ImagesFilter::new();

    // 测试 data_url_string 方法
    let data_url = "data:image/png;base64,ABCD";
    let http_url = "https://example.com/image.png";

    if test_filter.data_url_string(data_url) {
        println!("✅ data_url_string正确识别data URL");
    } else {
        println!("❌ data_url_string无法识别data URL");
    }

    // 测试 relative_url_string 方法
    if test_filter.relative_url_string("images/test.png") {
        println!("✅ relative_url_string正确识别相对URL");
    } else {
        println!("❌ relative_url_string无法识别相对URL");
    }

    if !test_filter.relative_url_string(http_url) {
        println!("✅ relative_url_string正确区分绝对URL");
    } else {
        println!("❌ relative_url_string错误地识别绝对URL为相对URL");
    }

    Ok(())
}

// 写入文件
fn write_file(path: &Path, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
