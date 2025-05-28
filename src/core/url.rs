//! URL 处理模块
//!
//! 参考原始 Ruby 项目中的 url.rb 实现
//! 提供 URL 解析、合并和操作功能

use std::collections::HashMap;
use std::path::PathBuf;
use url::{ParseError, Url};

/// URL 处理结构体，封装 url 库的功能，同时提供与原 Ruby 项目相似的 API
#[derive(Debug, Clone)]
pub struct DocUrl {
    inner: Url,
}

impl DocUrl {
    /// 创建一个新的 DocUrl 实例
    pub fn new(url: &str) -> Result<Self, ParseError> {
        Ok(Self {
            inner: Url::parse(url)?,
        })
    }

    /// 解析 URL 字符串
    pub fn parse(url: &str) -> Result<Self, ParseError> {
        Self::new(url)
    }

    /// 合并基本 URL 和相对路径
    pub fn join(&self, path: &str) -> Result<Self, ParseError> {
        let joined = self.inner.join(path)?;
        Ok(Self { inner: joined })
    }

    /// 静态方法 - 合并 URL
    pub fn join_urls(base: &str, path: &str) -> Result<Self, ParseError> {
        let base_url = Url::parse(base)?;
        let joined = base_url.join(path)?;
        Ok(Self { inner: joined })
    }

    /// 更新 URL 的参数
    pub fn merge(&self, params: HashMap<&str, &str>) -> Result<Self, ParseError> {
        let mut new_url = self.inner.clone();

        for (key, value) in params {
            match key {
                "scheme" => new_url
                    .set_scheme(value)
                    .map_err(|_| ParseError::RelativeUrlWithoutBase)?, // Using RelativeUrlWithoutBase as a placeholder
                "username" => {
                    let password = new_url.password().unwrap_or("");
                    new_url
                        .set_username(value)
                        .map_err(|_| ParseError::RelativeUrlWithoutBase)?; // Using RelativeUrlWithoutBase
                    new_url
                        .set_password(Some(password))
                        .map_err(|_| ParseError::RelativeUrlWithoutBase)?; // Using RelativeUrlWithoutBase
                }
                "password" => {
                    new_url
                        .set_password(Some(value))
                        .map_err(|_| ParseError::RelativeUrlWithoutBase)?; // Using RelativeUrlWithoutBase
                }
                "host" => new_url
                    .set_host(Some(value))
                    .map_err(|_| ParseError::RelativeUrlWithoutBase)?, // Using RelativeUrlWithoutBase
                "port" => {
                    if let Ok(port) = value.parse::<u16>() {
                        new_url
                            .set_port(Some(port))
                            .map_err(|_| ParseError::InvalidPort)?;
                    } else {
                        return Err(ParseError::InvalidPort);
                    }
                }
                "path" => new_url.set_path(value),
                "query" => new_url.set_query(Some(value)),
                "fragment" => new_url.set_fragment(Some(value)),
                _ => {}
            }
        }

        Ok(Self { inner: new_url })
    }

    /// 获取 URL 的源（origin）部分
    pub fn origin(&self) -> String {
        let scheme = self.inner.scheme();
        let host = match self.inner.host_str() {
            Some(h) => h,
            None => return String::new(),
        };

        let mut origin = format!("{}://{}", scheme, host.to_lowercase());
        if let Some(port) = self.inner.port() {
            if !((scheme == "http" && port == 80) || (scheme == "https" && port == 443)) {
                origin.push_str(&format!(":{}", port));
            }
        }

        origin
    }

    /// 获取相对路径（从 URL 提取路径部分）
    pub fn relative(&self) -> String {
        let mut result = self.inner.path().to_string();
        if let Some(query) = self.inner.query() {
            result.push_str(&format!("?{}", query));
        }
        if let Some(fragment) = self.inner.fragment() {
            result.push_str(&format!("#{}", fragment));
        }
        result
    }

    /// 将路径和查询字符串结合
    pub fn path_and_query(&self) -> String {
        self.relative()
    }

    /// 获取路径（不包括查询字符串和片段）
    pub fn path(&self) -> &str {
        self.inner.path()
    }

    /// 获取查询字符串
    pub fn query(&self) -> Option<&str> {
        self.inner.query()
    }

    /// 获取片段部分
    pub fn fragment(&self) -> Option<&str> {
        self.inner.fragment()
    }

    /// 获取内部 Url 实例的可变引用
    pub fn inner_mut(&mut self) -> &mut Url {
        &mut self.inner
    }

    /// 获取内部 Url 实例的不可变引用
    pub fn inner(&self) -> &Url {
        &self.inner
    }

    /// 将 URL 转换为字符串
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    /// 将 URL 的路径转换为文件系统路径
    pub fn to_filepath(&self) -> PathBuf {
        let path = self.inner.path();
        let path = path.trim_start_matches('/');
        PathBuf::from(path)
    }
}

impl From<Url> for DocUrl {
    fn from(url: Url) -> Self {
        Self { inner: url }
    }
}

impl AsRef<str> for DocUrl {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse() {
        let url = DocUrl::parse("https://example.com/path").unwrap();
        assert_eq!(url.inner().scheme(), "https");
        assert_eq!(url.inner().host_str(), Some("example.com"));
        assert_eq!(url.inner().path(), "/path");
    }

    #[test]
    fn test_join() {
        let url = DocUrl::parse("https://example.com/").unwrap();
        let joined = url.join("subpath").unwrap();
        assert_eq!(joined.to_string(), "https://example.com/subpath");
    }

    #[test]
    fn test_join_urls() {
        let joined = DocUrl::join_urls("https://example.com/", "subpath").unwrap();
        assert_eq!(joined.to_string(), "https://example.com/subpath");
    }

    #[test]
    fn test_origin() {
        let url = DocUrl::parse("https://example.com:8080/path").unwrap();
        assert_eq!(url.origin(), "https://example.com:8080");
    }

    #[test]
    fn test_relative() {
        let url = DocUrl::parse("https://example.com/path?query=value#fragment").unwrap();
        assert_eq!(url.relative(), "/path?query=value#fragment");
    }

    #[test]
    fn test_merge() {
        let url = DocUrl::parse("https://example.com/path").unwrap();
        let mut params = HashMap::new();
        params.insert("path", "/newpath");
        params.insert("query", "key=value");
        let merged = url.merge(params).unwrap();
        assert_eq!(merged.to_string(), "https://example.com/newpath?key=value");
    }
}
