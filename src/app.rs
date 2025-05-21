//! The app module handles the web user interface of DevDocs Rust.
//! It's responsible for displaying documentation, handling search requests,
//! and providing a responsive user interface.

use std::error::Error;

/// Configuration for the web application
pub struct AppConfig {
    pub port: u16,
    pub host: String,
    pub static_files_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 9292,  // Same port as original DevDocs
            host: "127.0.0.1".to_string(),
            static_files_path: "public".to_string(),
        }
    }
}

/// Starts the web application server
pub fn start() -> Result<(), Box<dyn Error>> {
    let config = AppConfig::default();
    
    // 创建一个运行时环境
    let rt = tokio::runtime::Runtime::new()?;
    
    // 在 tokio 运行时中执行异步代码
    rt.block_on(async {
        // 创建一个简单的路由
        let app = axum::Router::new()
            .route("/", axum::routing::get(|| async { "欢迎使用 DevDocs Rust!" }));
        
        // 构建服务器地址
        let addr = format!("{}:{}", config.host, config.port).parse()?;
        
        println!("应用程序启动在 {}:{}", config.host, config.port);
        
        // 启动服务器
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    })
}

/// Handles search requests
pub fn search(query: &str, docs: &[String]) -> Vec<SearchResult> {
    println!("Searching for '{}' in {:?}", query, docs);
    vec![] // Placeholder
}

/// Represents a search result
pub struct SearchResult {
    pub doc_name: String,
    pub doc_path: String,
    pub title: String,
    pub snippet: String,
    pub score: f32,
}
