//! 命令行处理器

use crate::cli::{Cli, Commands, DocsCommand, ConsoleCommand, TestCommand, AssetsCommand, ScraperCommand};
use clap::Parser;
use std::error::Error;

/// 处理命令行参数
pub async fn handle_cli() -> Result<(), Box<dyn Error>> {
    println!("[cli/handler.rs] Entered handle_cli.");
    let cli = Cli::parse();
    println!("[cli/handler.rs] CLI args parsed: {:?}", cli.command);

    match &cli.command {
        Commands::Server { host, port } => {
            println!("启动服务器在 {}:{}", host, port);
            crate::start(host, port).await?;
        },
        Commands::Docs(docs_cmd) => match docs_cmd {
            DocsCommand::List {} => {
                println!("可用文档:");
                for doc in crate::get_default_docs() {
                    println!("  {}", doc);
                }
            },
            DocsCommand::Download { docs, all, default, installed: _ } => {
                // 下载指定文档
                if *all {
                    println!("下载所有文档");
                    // TODO: 实现所有文档下载
                } else if *default {
                    println!("下载默认文档");
                    for doc in crate::get_default_docs() {
                        println!("下载文档: {}", doc);
                        // TODO: 实现文档下载
                    }
                } else if !docs.is_empty() {
                    println!("下载指定文档");
                    for doc in docs {
                        println!("下载文档: {}", doc);
                        // TODO: 实现文档下载
                    }
                } else {
                    eprintln!("请指定要下载的文档，使用 --all 或 --default 或提供文档名列表");
                }
            },
            DocsCommand::Generate { doc, version } => {
                // 生成/抓取文档
                if let Some(doc_name) = doc {
                    let doc_version = version.as_deref().unwrap_or(""); // Use provided version or default
                    println!("抓取文档: {} (版本: {})", doc_name, if doc_version.is_empty() { "latest" } else { doc_version });
                    crate::scrape_async(doc_name, doc_version, "").await?;
                } else {
                    eprintln!("请指定要抓取的文档名称");
                }
            },
            DocsCommand::Page { doc, page } => {
                println!("生成/抓取单页: {} {}", doc, page);
                // TODO: 实现单页抓取逻辑
            },
            DocsCommand::Package { doc } => {
                println!("打包文档: {}", doc);
                // TODO: 实现打包逻辑
            },
            DocsCommand::Clean {} => {
                println!("删除文档包");
                // TODO: 实现清理逻辑
            },
            DocsCommand::Manifest {} => {
                println!("生成文档清单");
                // TODO: 实现清单生成
            },
        },
        Commands::Console(console_cmd) => match console_cmd {
            ConsoleCommand::Console => {
                println!("启动 REPL");
                // TODO: 实现 REPL
            },
            ConsoleCommand::Docs => {
                println!("启动 Docs 模块 REPL");
                // TODO: 实现 Docs REPL
            },
        },
        Commands::Test(test_cmd) => match test_cmd {
            TestCommand::All => {
                println!("运行所有测试");
                // TODO: 实现所有测试
            },
            TestCommand::Docs => {
                println!("运行 Docs 测试");
                // TODO: 实现 Docs 测试
            },
            TestCommand::App => {
                println!("运行 App 测试");
                // TODO: 实现 App 测试
            },
        },
        Commands::Assets(assets_cmd) => match assets_cmd {
            AssetsCommand::Compile => {
                println!("编译前端资源");
                // TODO: 实现资源编译
            },
            AssetsCommand::Clean => {
                println!("清理旧资源");
                // TODO: 实现资源清理
            },
        },
        Commands::Scraper(scraper_cmd) => match scraper_cmd {
            ScraperCommand::List {} => {
                // 初始化过滤器注册表
                crate::filters::registry::initialize();

                // 创建爬虫工厂
                let factory = crate::scrapers::ScraperFactory::new();

                // 列出所有可用的爬虫类型
                println!("可用的爬虫类型:");
                for scraper_type in factory.available_scrapers() {
                    println!("  - {}", scraper_type);
                }
            },
            ScraperCommand::Run { scraper_type, version } => {
                // 初始化过滤器注册表
                crate::filters::registry::initialize();

                // 创建爬虫工厂
                let factory = crate::scrapers::ScraperFactory::new();

                // 创建爬虫
                match factory.create(scraper_type, version) {
                    Some(scraper) => {
                        // 显示爬虫信息
                        let doc = scraper.doc_scraper();
                        println!("爬取 {} 文档 (版本: {})", doc.name, doc.version);
                        println!("基础URL: {}", doc.base_url);
                        println!("输出路径: {}", doc.output_path);
                        println!("初始路径: {:?}", doc.initial_paths);

                        // 这里可以添加实际的爬取逻辑
                        println!("爬取完成");
                    },
                    None => {
                        return Err(format!("未知的爬虫类型: {}", scraper_type).into());
                    }
                }
            },
        },
    }
    Ok(())
}
