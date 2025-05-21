//! 速率限制器
//! 参考 Ruby 版本的 RateLimiter 类实现

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

/// 速率限制器，限制每分钟的请求数量
pub struct RateLimiter {
    /// 限制值（每分钟的最大请求数）
    limit: u32,
    /// 当前分钟
    current_minute: u32,
    /// 当前计数
    counter: u32,
    /// 上次请求时间
    last_request_time: Option<Instant>,
}

impl RateLimiter {
    /// 创建新的速率限制器
    pub fn new(limit: u32) -> Self {
        Self {
            limit,
            current_minute: Self::current_minute(),
            counter: 0,
            last_request_time: None,
        }
    }

    /// 获取当前分钟
    fn current_minute() -> u32 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        (now.as_secs() / 60) as u32
    }

    /// 设置限制值
    pub fn set_limit(&mut self, limit: u32) {
        self.limit = limit;
    }

    /// 等待请求，确保不超过速率限制
    pub async fn wait(&mut self) {
        // 检查是否进入新的一分钟
        let current_minute = Self::current_minute();
        if current_minute != self.current_minute {
            self.current_minute = current_minute;
            self.counter = 0;
        }

        // 增加计数
        self.counter += 1;

        // 如果达到限制，等待到下一分钟开始
        if self.counter >= self.limit {
            // 计算需要等待的时间：当前分钟剩余秒数 + 1秒
            let current_seconds = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() % 60;
            
            let wait_seconds = 61 - current_seconds as u64;
            println!("达到速率限制 ({}次/分钟)，等待 {} 秒...", self.limit, wait_seconds);
            
            sleep(Duration::from_secs(wait_seconds)).await;
            
            // 重置计数器
            self.current_minute = Self::current_minute();
            self.counter = 1;
        }
        // 否则，如果不是第一次请求，添加一个小延迟以避免服务器过载
        else if let Some(last_time) = self.last_request_time {
            let elapsed = last_time.elapsed();
            if elapsed < Duration::from_millis(100) {
                sleep(Duration::from_millis(100) - elapsed).await;
            }
        }
        
        // 更新上次请求时间
        self.last_request_time = Some(Instant::now());
    }
}
