//! xwdoc - API 文档浏览器的主入口点

use std::error::Error;
use xwdoc::cli::handle_cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志
    env_logger::init();
    
    // 显示版本信息
    println!("xwdoc {}", env!("CARGO_PKG_VERSION"));
    
    // 处理命令行
    handle_cli().await
}
