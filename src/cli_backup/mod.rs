//! 命令行工具模块

use clap::{Parser, Subcommand};
use std::error::Error;

pub mod scraper_cmd;
pub mod handler;

pub use handler::handle_cli;

#[derive(Parser)]
#[command(name = "xwdoc")] // 修改主命令名为 xwdoc
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[derive(Debug)] // Add Debug derive
pub enum Commands {
    /// 启动 Web 服务器
    #[command(name = "server")]
    Server {
        /// 主机地址
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        /// 端口
        #[arg(short, long, default_value_t = 9292)]
        port: u16,
    },

    /// 文档相关命令
    #[command(name = "docs")]
    Docs(DocsCommand),

    /// 控制台命令
    #[command(name = "console")]
    Console(ConsoleCommand),

    /// 测试命令
    #[command(name = "test")]
    Test(TestCommand),

    /// 资源管理命令
    #[command(name = "assets")]
    Assets(AssetsCommand),

    /// 爬虫相关命令
    #[command(name = "scraper")]
    Scraper(ScraperCommand),
}

/// 文档相关子命令
#[derive(Subcommand)]
#[derive(Debug)] // Add Debug derive
pub enum DocsCommand {
    /// 列出可用的文档
    #[command(name = "list")]
    List {},

    /// 下载文档
    #[command(name = "download")]
    Download {
        /// 要下载的文档名称
        #[arg(required = false)]
        docs: Vec<String>,

        /// 下载所有文档
        #[arg(short, long, default_value_t = false)]
        all: bool,

        /// 下载默认文档
        #[arg(short, long, default_value_t = false)]
        default: bool,

        /// 重新下载已安装的文档
        #[arg(short, long, default_value_t = false)]
        installed: bool,
    },

    /// 生成文档
    #[command(name = "generate")]
    Generate {
        /// 要生成的文档名称
        #[arg(required = true)]
        name: String,

        /// 文档版本
        #[arg(short, long)]
        version: Option<String>,

        /// 爬取的URL
        #[arg(short, long)]
        url: Option<String>,
    },
}

/// 控制台相关子命令
#[derive(Subcommand)]
#[derive(Debug)] // Add Debug derive
pub enum ConsoleCommand {
    /// 启动交互式控制台
    Start { },
}

/// 测试相关子命令
#[derive(Subcommand)]
#[derive(Debug)] // Add Debug derive
pub enum TestCommand {
    /// 运行测试
    #[command(name = "run")]
    Run { 
        /// 测试名称
        #[arg(required = false)]
        name: Option<String>,
    },
}

/// 资源管理子命令
#[derive(Subcommand)]
#[derive(Debug)] // Add Debug derive
pub enum AssetsCommand {
    /// 编译资源
    #[command(name = "compile")]
    Compile {},

    /// 清理资源
    #[command(name = "clean")]
    Clean {},
}

// ScraperCommand 从 scraper_cmd.rs 导入
pub use scraper_cmd::ScraperCommand;