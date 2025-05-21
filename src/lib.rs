//! xwdoc - 一个 Rust 实现的 API 文档浏览器
//!
//! 该库提供了处理和展示多种编程语言和框架文档的功能。
//! 它包含了文档抓取、解析、索引和展示等组件。

pub mod cli;
pub mod core;
pub mod docs;
pub mod storage;
pub mod web;

use std::error::Error;

use crate::core::config::Config;
use crate::docs::DocRegistry;
use crate::storage::FileStore;
use crate::web::server::Server;

/// 初始化 xwdoc 应用程序
pub fn init(config: Config) -> Result<(), Box<dyn Error>> {
    // 创建文档注册表
    let mut registry = DocRegistry::new();

    // 初始化存储
    let _store = FileStore::new(&config.docs_path);

    // 加载已有文档
    registry.load_from_disk(&config.docs_path)?;

    println!("xwdoc初始化完成，已加载{}个文档", registry.all().len());
    Ok(())
}

/// 启动Web服务器
pub async fn start(host: &str, port: u16) -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    // 初始化应用
    init(config.clone())?;

    // 创建路由
    let router = web::routes::create_routes(&config);

    // 创建并启动服务器
    let server = Server::new(config, host, port).with_router(router);

    println!("服务器启动在 http://{}:{}", host, port);
    println!("Web界面准备就绪，请在浏览器中访问");

    server.run().await
}

/// 异步抓取文档
pub async fn scrape_async(
    name: &str,
    version: &str,
    output_or_url: &str,
) -> Result<(), Box<dyn Error>> {
    use crate::core::scraper::Scraper as CoreScraper;
    let config = Config::default();
    let result = match name.to_lowercase().as_str() {
        "html" => {
            let mut scraper = docs::html::HtmlScraper::new(version, &config.docs_path);
            scraper.run().await
        }
        "css" => {
            let mut scraper = docs::css::CssScraper::new(version, &config.docs_path);
            scraper.run().await
        }
        "javascript" => {
            let mut scraper = docs::javascript::JavaScriptScraper::new(version, &config.docs_path);
            scraper.run().await
        }
        "rust" => {
            let mut scraper = docs::rust::RustScraper::new(version, &config.docs_path);
            scraper.run().await
        }
        "typescript" => {
            let mut scraper = docs::typescript::TypeScriptScraper::new(version, &config.docs_path);
            scraper.run().await
        }
        "babel" => {
            // 使用输出路径或默认路径
            let output_path = if !output_or_url.is_empty() && !output_or_url.starts_with("http") {
                output_or_url
            } else {
                &config.docs_path
            };
            let mut scraper = docs::babel::BabelScraper::new(output_path, version);
            scraper.run().await
        }
        _ => {
            // 只有当不是内置类型时才需要 url
            if output_or_url.is_empty() || !output_or_url.starts_with("http") {
                return Err("非内置文档类型必须指定 url".into());
            }
            let mut scraper = crate::core::scraper::UrlScraper::new(
                name,
                version,
                output_or_url,
                &config.docs_path,
            );
            scraper.run().await
        }
    };
    result.map_err(|e| Box::new(e) as Box<dyn Error>)
}

/// 获取默认文档列表
pub fn get_default_docs() -> Vec<String> {
    vec!["babel".to_string()]
}

/// 抓取文档（同步接口，仅兼容性保留）
pub fn scrape(name: &str, version: &str, url: &str) -> Result<(), Box<dyn Error>> {
    // 创建运行时并阻塞异步函数
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(scrape_async(name, version, url))
}

/// 生成文档清单
pub fn generate_manifest() -> Result<(), Box<dyn Error>> {
    let config = Config::default();
    let mut registry = DocRegistry::new();

    // 加载已有文档
    registry.load_from_disk(&config.docs_path)?;

    // 生成清单
    registry.generate_manifest(&config.docs_path)?;

    Ok(())
}
