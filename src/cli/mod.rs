//! 命令行参数解析模块

use clap::{Parser, Subcommand};

/// xwdoc 命令行参数定义
#[derive(Parser)]
#[clap(name = "xwdoc", about = "轻量级的 API 文档浏览器", version)]
pub struct Cli {
    /// 要执行的命令
    #[clap(subcommand)]
    pub command: Commands,
}

/// 可用命令
#[derive(Subcommand)]
pub enum Commands {
    /// 启动文档服务器
    Server {
        /// 监听的主机地址
        #[clap(long, default_value = "127.0.0.1")]
        host: String,

        /// 监听的端口
        #[clap(long, default_value = "3000")]
        port: u16,
    },

    /// 列出可用文档
    DocsList,

    /// 下载文档
    DocsDownload {
        /// 要下载的文档列表
        #[clap(value_name = "DOCS")]
        docs: Vec<String>,

        /// 下载所有可用文档
        #[clap(long)]
        all: bool,

        /// 下载默认文档集
        #[clap(long)]
        default: bool,

        /// 更新已安装的文档
        #[clap(long)]
        installed: bool,
    },

    /// 生成文档
    DocsGenerate {
        /// 要生成的文档名称
        doc: Option<String>,

        /// 文档版本
        #[clap(long, short)]
        version: Option<String>,
    },

    /// 生成文档的单个页面
    DocsPage {
        /// 文档名称
        #[clap(required = true)]
        doc: String,

        /// 页面路径
        #[clap(required = true)]
        page: String,
    },

    /// 打包文档
    DocsPackage {
        /// 要打包的文档名称
        #[clap(required = true)]
        doc: String,
    },

    /// 清理文档包
    DocsClean,

    /// 生成文档清单
    DocsManifest,

    /// 列出可用的文档抓取器
    ScraperList,

    /// 运行文档抓取器
    ScraperRun {
        /// 抓取器名称
        #[clap(required = true)]
        name: String,

        /// 文档版本
        #[clap(long, default_value = "latest")]
        version: String,

        /// 输出路径或URL（取决于抓取器类型）
        #[clap(long)]
        output: Option<String>,
    },
}

pub mod handler;
pub mod scraper_cmd;

pub use handler::handle_cli;
pub use scraper_cmd::*;
