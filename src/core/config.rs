//! 应用配置模块

/// 应用全局配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 文档存储路径
    pub docs_path: String,
    /// 默认文档列表
    pub default_docs: Vec<String>,
    /// 服务器主机名
    pub host: String,
    /// 服务器端口
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            docs_path: "docs".to_string(),
            default_docs: vec![
                "html".to_string(),
                "css".to_string(),
                "javascript".to_string(),
                "rust".to_string(),
            ],
            host: "127.0.0.1".to_string(),
            port: 8000,
        }
    }
}

impl Config {
    /// 创建新的配置实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文档存储路径
    pub fn with_docs_path(mut self, path: &str) -> Self {
        self.docs_path = path.to_string();
        self
    }

    /// 设置默认文档列表
    pub fn with_default_docs(mut self, docs: Vec<String>) -> Self {
        self.default_docs = docs;
        self
    }

    /// 设置服务器主机名
    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// 设置服务器端口
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
}
