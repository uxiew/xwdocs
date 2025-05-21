//! HTTP 请求器模块
//!
//! 参考原始 Ruby 项目中的 requester.rb 实现
//! 提供批量发送 HTTP 请求的功能

use crate::core::error::Result;
use crate::core::instrumentable;
use crate::core::request::{Request, RequestOptions};
use crate::core::response::Response;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

/// HTTP 请求器结构体
pub struct Requester {
    /// 请求选项
    request_options: RequestOptions,
    /// 最大并发请求数
    max_concurrency: usize,
    /// 响应回调函数
    on_response: Vec<Box<dyn Fn(&Response) -> Option<Vec<String>> + Send + Sync>>,
}

impl Requester {
    /// 创建新的 HTTP 请求器
    pub fn new(max_concurrency: Option<usize>, options: Option<RequestOptions>) -> Self {
        Self {
            request_options: options.unwrap_or_default(),
            max_concurrency: max_concurrency.unwrap_or(20),
            on_response: Vec::new(),
        }
    }

    /// 静态方法，创建请求器并运行请求
    pub fn run<F>(urls: Vec<String>, max_concurrency: Option<usize>, options: Option<RequestOptions>, callback: F) -> Result<Self>
    where
        F: Fn(&Response) -> Option<Vec<String>> + 'static + Send + Sync,
    {
        let mut requester = Self::new(max_concurrency, options);
        requester.on_response(callback);
        requester.request(urls)?;
        Ok(requester)
    }

    /// 添加响应回调函数
    pub fn on_response<F>(&mut self, callback: F)
    where
        F: Fn(&Response) -> Option<Vec<String>> + 'static + Send + Sync,
    {
        self.on_response.push(Box::new(callback));
    }

    /// 发送请求
    pub fn request(&self, urls: Vec<String>) -> Result<()> {
        // 创建异步运行时
        let rt = Runtime::new().unwrap();
        
        // 创建队列和已处理 URL 集合
        let queue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(urls));
        let processed: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));

        // 处理队列中的 URL
        rt.block_on(async {
            while !queue.lock().unwrap().is_empty() {
                let mut futures = FuturesUnordered::new();
                
                // 填充并发请求队列
                for _ in 0..self.max_concurrency {
                    if let Some(url) = Self::get_next_url(&queue, &processed) {
                        let request_options = self.request_options.clone();
                        let processed_clone = processed.clone();
                        let queue_clone = queue.clone();
                        let on_response = self.on_response.clone();
                        
                        // 创建异步任务
                        futures.push(tokio::spawn(async move {
                            // 标记为已处理
                            processed_clone.lock().unwrap().insert(url.clone(), true);
                            
                            // 发送请求
                            match Self::send_request(&url, &request_options) {
                                Ok(response) => {
                                    // 调用回调处理响应
                                    for callback in &on_response {
                                        if let Some(new_urls) = callback(&response) {
                                            // 添加新的 URL 到队列
                                            let mut q = queue_clone.lock().unwrap();
                                            q.extend(new_urls);
                                        }
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Error fetching {}: {}", url, err);
                                }
                            }
                        }));
                    } else {
                        break;
                    }
                }
                
                // 等待所有当前请求完成
                while let Some(result) = futures.next().await {
                    // Ignore errors in the spawned tasks
                    let _ = result;
                }
            }
        });

        Ok(())
    }

    /// 获取下一个要处理的 URL
    fn get_next_url(
        queue: &Arc<Mutex<Vec<String>>>,
        processed: &Arc<Mutex<HashMap<String, bool>>>,
    ) -> Option<String> {
        let mut q = queue.lock().unwrap();
        let processed_urls = processed.lock().unwrap();
        
        // 找到第一个未处理的 URL
        let index = q.iter().position(|url| !processed_urls.contains_key(url))?;
        Some(q.remove(index))
    }

    /// 发送单个请求
    fn send_request(url: &str, options: &RequestOptions) -> Result<Response> {
        let payload = HashMap::from([("url".to_string(), url.to_string())]);
        
        instrumentable::instrument("handle_request.requester", payload, || {
            let request = Request::new(url, Some(options.clone()))?;
            request.run()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_requester_callbacks() {
        // 跳过实际网络请求的测试
        // 这里应该使用模拟（mock）对象
    }
}
