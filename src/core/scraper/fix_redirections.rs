//! 修复重定向行为
//! 参考 Ruby 版本的 FixRedirectionsBehavior 模块实现

use crate::core::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

/// 重定向映射类型
pub type Redirections = Arc<Mutex<HashMap<String, String>>>;

/// 重定向辅助函数
pub struct FixRedirections {
    redirections: Redirections,
}

impl FixRedirections {
    /// 创建新的重定向辅助对象
    pub fn new() -> Self {
        Self {
            redirections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 获取重定向映射的共享引用
    pub fn redirections(&self) -> Redirections {
        self.redirections.clone()
    }

    /// 添加重定向映射
    pub async fn add_redirection(&self, from_url: &str, to_url: &str) {
        let mut redirects = self.redirections.lock().await;
        redirects.insert(from_url.to_string(), to_url.to_string());
    }

    /// 获取有效的URL（处理重定向后）
    pub async fn effective_url(&self, url: &str) -> String {
        let redirects = self.redirections.lock().await;
        match redirects.get(url) {
            Some(redirect_url) => redirect_url.clone(),
            None => url.to_string(),
        }
    }

    /// 从URL获取路径部分
    pub fn path_from_url(&self, url: &str) -> Result<String> {
        match Url::parse(url) {
            Ok(parsed_url) => Ok(parsed_url.path().to_string()),
            Err(e) => Err(Error::Message(format!("无法解析URL: {}", e))),
        }
    }

    /// 应用重定向到路径
    pub async fn apply_redirections_to_paths(&self, paths: &mut HashMap<String, String>) -> Result<()> {
        let redirects = self.redirections.lock().await;
        
        // 创建一个临时映射，保存重定向后的路径
        let mut path_redirections = HashMap::new();
        
        // 对所有重定向URL进行处理
        for (from_url, to_url) in redirects.iter() {
            // 获取URL对应的路径
            let from_path = self.path_from_url(from_url)?;
            let to_path = self.path_from_url(to_url)?;
            
            // 只有当路径不同时才添加重定向
            if from_path != to_path {
                path_redirections.insert(from_path.to_lowercase(), to_path);
            }
        }
        
        // 更新路径映射
        // 注意：这里我们只处理路径本身，不修改对应的内容
        // 内容会在提取阶段处理
        for (path, _) in paths.clone().iter() {
            if let Some(redirect_path) = path_redirections.get(&path.to_lowercase()) {
                if let Some(content) = paths.remove(path) {
                    paths.insert(redirect_path.clone(), content);
                }
            }
        }
        
        Ok(())
    }
}
