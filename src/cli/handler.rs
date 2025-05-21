//! 命令行处理器

use crate::cli::{Cli, Commands};
use clap::Parser;
use std::error::Error;

/// 处理命令行参数
pub async fn handle_cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Server { host, port } => {
            println!("启动服务器在 {}:{}", host, port);
            crate::start(host, *port).await?;
        }

        // 文档相关命令
        Commands::DocsList => {
            println!("可用文档:");
            for doc in crate::docs::get_available_docs() {
                println!("  {}", doc);
            }
        }
        Commands::DocsDownload {
            docs,
            all,
            default,
            installed,
        } => {
            // 下载指定文档
            if *all {
                println!("下载所有文档");
                crate::docs::download_all_docs().await?;
            } else if *default {
                println!("下载默认文档");
                crate::docs::download_default_docs().await?;
            } else if *installed {
                println!("更新已安装的文档");
                crate::docs::download_installed_docs().await?;
            } else if !docs.is_empty() {
                println!("下载指定文档");
                crate::docs::download_specific_docs(docs).await?;
            } else {
                eprintln!("请指定要下载的文档，使用 --all 或 --default 或提供文档名列表");
            }
        }
        Commands::DocsGenerate { doc, version } => {
            // 生成/抓取文档
            if let Some(doc_name) = doc {
                let doc_version = version.as_deref().unwrap_or("latest");
                println!("抓取文档: {} (版本: {})", doc_name, doc_version);
                crate::docs::generate_doc(doc_name, doc_version).await?;
            } else {
                eprintln!("请指定要抓取的文档名称");
            }
        }
        Commands::DocsPage { doc, page } => {
            // 生成单页
            println!("生成页面: {}/{}", doc, page);
            crate::docs::generate_page(&doc, &page).await?;
        }
        Commands::DocsPackage { doc } => {
            // 打包文档
            println!("打包文档: {}", doc);
            crate::docs::package_doc(&doc)?;
        }
        Commands::DocsClean => {
            // 清理文档
            println!("清理文档包");
            crate::docs::clean_docs()?;
        }
        Commands::DocsManifest => {
            // 生成清单
            println!("生成文档清单");
            crate::docs::generate_manifest()?;
        }

        // 抓取器相关命令
        Commands::ScraperList => {
            crate::cli::list_scrapers()?;
        }
        Commands::ScraperRun {
            name,
            version,
            output,
        } => {
            crate::cli::run_scraper(name, version, output.as_deref()).await?;
        }
    }

    Ok(())
}
