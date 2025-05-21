//! 爬虫命令行接口
//! 用于从命令行运行文档爬虫

use clap::{App, Arg, ArgMatches, SubCommand};
use crate::scrapers::factory::ScraperFactory;
use crate::filters::registry;
use std::error::Error;

/// 爬虫命令处理
pub struct ScraperCommand;

impl ScraperCommand {
    /// 创建命令行配置
    pub fn cli() -> App<'static, 'static> {
        App::new("scraper")
            .about("运行文档爬虫")
            .subcommand(
                SubCommand::with_name("list")
                    .about("列出所有可用的爬虫类型")
            )
            .subcommand(
                SubCommand::with_name("run")
                    .about("运行指定类型的爬虫")
                    .arg(
                        Arg::with_name("type")
                            .help("爬虫类型")
                            .required(true)
                            .index(1)
                    )
                    .arg(
                        Arg::with_name("version")
                            .help("文档版本")
                            .required(true)
                            .index(2)
                    )
            )
    }

    /// 运行命令
    pub fn run(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // 初始化过滤器注册表
        registry::initialize();

        // 创建爬虫工厂
        let factory = ScraperFactory::new();

        // 处理子命令
        match matches.subcommand() {
            ("list", _) => {
                // 列出所有可用的爬虫类型
                println!("可用的爬虫类型:");
                for scraper_type in factory.available_scrapers() {
                    println!("  - {}", scraper_type);
                }
                Ok(())
            },
            ("run", Some(run_matches)) => {
                // 获取爬虫类型和版本
                let scraper_type = run_matches.value_of("type").unwrap();
                let version = run_matches.value_of("version").unwrap();

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
                        Ok(())
                    },
                    None => {
                        Err(format!("未知的爬虫类型: {}", scraper_type).into())
                    }
                }
            },
            _ => {
                Err("未知的子命令".into())
            }
        }
    }
}
