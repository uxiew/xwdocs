//! HTTP 响应模块
//!
//! 参考原始 Ruby 项目中的 response.rb 实现
//! 提供 HTTP 响应处理功能

use crate::core::error::Result;
use crate::core::url::DocUrl;
use reqwest::blocking;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

/// HTTP 响应结构体
#[derive(Debug)]
pub struct Response {
    /// 响应状态码
    pub code: u16,
    /// 响应体
    pub body: String,
    /// 头部信息
    pub headers: HashMap<String, String>,
    /// 请求的 URL
    pub url: DocUrl,
    /// 有效的 URL（考虑重定向后）
    pub effective_url: DocUrl,
    /// 是否请求超时
    pub timed_out: bool,
}

impl Response {
    /// 从 reqwest 响应创建
    pub fn from_reqwest(response: blocking::Response, url: &DocUrl) -> Result<Self> {
        let code = response.status().as_u16();
        let effective_url = if let Some(location) = response.url().to_string().strip_prefix("http") {
            // 修复 URL，确保它是有效的
            DocUrl::parse(&format!("http{}", location))?
        } else {
            DocUrl::parse(response.url().as_str())?
        };
        
        let headers = Self::convert_headers(response.headers());
        let body = response.text().unwrap_or_default();
        let timed_out = false; // reqwest 会直接返回错误而不是设置 timed_out 标志

        Ok(Self {
            code,
            body,
            headers,
            url: url.clone(),
            effective_url,
            timed_out,
        })
    }

    /// 将 reqwest 头部映射转换为哈希映射
    fn convert_headers(headers: &HeaderMap<HeaderValue>) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        for (name, value) in headers {
            if let Ok(value_str) = value.to_str() {
                result.insert(name.to_string(), value_str.to_string());
            }
        }
        
        result
    }

    /// 检查响应是否成功（状态码为 200）
    pub fn success(&self) -> bool {
        self.code == 200
    }

    /// 检查响应是否出现错误
    pub fn error(&self) -> bool {
        self.code == 0 || (self.code != 404 && self.code != 403 && self.code >= 400 && self.code <= 599)
    }

    /// 检查响应是否为空
    pub fn blank(&self) -> bool {
        self.body.is_empty()
    }

    /// 获取内容长度
    pub fn content_length(&self) -> usize {
        self.headers
            .get("Content-Length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0)
    }

    /// 获取 MIME 类型
    pub fn mime_type(&self) -> &str {
        self.headers
            .get("Content-Type")
            .map(|s| s.as_str())
            .unwrap_or("text/plain")
    }

    /// 检查响应是否为 HTML
    pub fn is_html(&self) -> bool {
        self.mime_type().contains("html")
    }

    /// 获取 URL 路径
    pub fn path(&self) -> &str {
        self.url.path()
    }

    /// 获取有效 URL 路径
    pub fn effective_path(&self) -> &str {
        self.effective_url.path()
    }

    /// 检查响应是否超时
    pub fn timed_out(&self) -> bool {
        self.timed_out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_response_methods() {
        let url = DocUrl::parse("https://example.com").unwrap();
        let effective_url = DocUrl::parse("https://example.com/page").unwrap();
        
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Content-Length".to_string(), "100".to_string());
        
        let response = Response {
            code: 200,
            body: "<html></html>".to_string(),
            headers,
            url,
            effective_url,
            timed_out: false,
        };
        
        assert_eq!(response.success(), true);
        assert_eq!(response.error(), false);
        assert_eq!(response.blank(), false);
        assert_eq!(response.content_length(), 100);
        assert_eq!(response.mime_type(), "text/html");
        assert_eq!(response.is_html(), true);
        assert_eq!(response.timed_out(), false);
    }
}
