//! URL 爬虫实现

use super::base::Scraper;
use super::filter::{Filter, FilterContext};
use crate::core::error::{Error, Result};
use regex::Regex;
use reqwest::Client;
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::sync::Mutex;
use tokio::time::sleep;
use url::Url;

/// 从网络地址爬取文档的爬虫
pub struct UrlScraper {
    /// 文档名称
    pub name: String,
    /// 文档版本
    pub version: String,
    /// 基础网址
    pub base_url: String,
    /// 多基础网址（对应 MultipleBaseUrls 模块）
    pub base_urls: Option<Vec<String>>,
    /// 输出路径
    pub output_path: String,
    /// 根路径
    pub root_path: String,
    /// 文档别名
    pub slug: String,
    /// 发布版本
    pub release: String,
    /// 初始访问路径
    pub initial_paths: Vec<String>,
    /// 需要跳过的路径
    pub skip_paths: Vec<String>,
    /// 需要跳过的模式
    pub skip_patterns: Vec<String>,
    /// 只处理这些路径
    pub only: Option<Vec<String>>,
    /// 只处理匹配这些模式的路径
    pub only_patterns: Option<Vec<String>>,
    /// 是否在路径末尾添加斜杠
    pub trailing_slash: bool,
    /// 文档根标题
    pub root_title: String,
    /// 许可和版权信息
    pub attribution: String,
    /// 相关链接
    pub links: Vec<(String, String)>,
    /// 过滤器列表
    pub filters: Vec<Box<dyn Filter>>,
    /// 跳过链接函数
    pub skip_link: Option<Box<dyn Fn(&str) -> bool + Send + Sync>>,
}

impl UrlScraper {
    /// 创建新的URL抓取器
    pub fn new(name: &str, version: &str, base_url: &str, output_path: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            base_url: base_url.to_string(),
            base_urls: None,
            output_path: output_path.to_string(),
            root_path: "/".to_string(),
            slug: name.to_lowercase().replace(' ', "_"),
            release: version.to_string(),
            initial_paths: vec!["/".to_string()],
            skip_paths: Vec::new(),
            skip_patterns: Vec::new(),
            only: None,
            only_patterns: None,
            trailing_slash: false,
            root_title: name.to_string(),
            attribution: String::new(),
            links: Vec::new(),
            filters: Vec::new(),
            skip_link: None,
        }
    }

    /// 添加过滤器
    pub fn with_filter(mut self, filter: Box<dyn Filter>) -> Self {
        self.filters.push(filter);
        self
    }

    /// 设置根路径
    pub fn with_root_path(mut self, root_path: &str) -> Self {
        self.root_path = root_path.to_string();
        self
    }

    /// 设置文档别名
    pub fn with_slug(mut self, slug: &str) -> Self {
        self.slug = slug.to_string();
        self
    }

    /// 设置发布版本
    pub fn with_release(mut self, release: &str) -> Self {
        self.release = release.to_string();
        self
    }

    /// 设置初始访问路径
    pub fn with_initial_paths(mut self, paths: Vec<String>) -> Self {
        self.initial_paths = paths;
        self
    }

    /// 设置需要跳过的路径
    pub fn with_skip_paths(mut self, paths: Vec<String>) -> Self {
        self.skip_paths = paths;
        self
    }

    /// 添加需要跳过的模式
    pub fn with_skip_patterns(mut self, patterns: Vec<&str>) -> Self {
        self.skip_patterns = patterns.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// 只处理指定路径
    pub fn with_only(mut self, paths: Vec<String>) -> Self {
        self.only = Some(paths);
        self
    }

    /// 只处理匹配指定模式的路径
    pub fn with_only_patterns(mut self, patterns: Vec<String>) -> Self {
        self.only_patterns = Some(patterns);
        self
    }

    /// 设置是否在路径末尾添加斜杠
    pub fn with_trailing_slash(mut self, should_add: bool) -> Self {
        self.trailing_slash = should_add;
        self
    }

    /// 设置文档根标题
    pub fn with_root_title(mut self, title: &str) -> Self {
        self.root_title = title.to_string();
        self
    }

    /// 设置许可和版权信息
    pub fn with_attribution(mut self, attribution: &str) -> Self {
        self.attribution = attribution.to_string();
        self
    }

    /// 设置相关链接
    pub fn with_links(mut self, links: Vec<(&str, &str)>) -> Self {
        self.links = links
            .into_iter()
            .map(|(name, url)| (name.to_string(), url.to_string()))
            .collect();
        self
    }

    /// 设置相关链接（使用String类型）
    pub fn with_string_links(mut self, links: Vec<(String, String)>) -> Self {
        self.links = links;
        self
    }

    /// 设置跳过链接函数
    pub fn with_skip_link<F>(mut self, skip_fn: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.skip_link = Some(Box::new(skip_fn));
        self
    }

    /// 设置多基础URL
    pub fn with_base_urls(mut self, urls: Vec<String>) -> Self {
        if !urls.is_empty() {
            // 第一个URL作为主基础URL
            self.base_url = urls[0].clone();
            self.base_urls = Some(urls);
        }
        self
    }

    /// 获取所有基础URL
    fn get_base_urls(&self) -> Vec<String> {
        if let Some(ref urls) = self.base_urls {
            urls.clone()
        } else {
            vec![self.base_url.clone()]
        }
    }

    /// 获取初始URL列表（包括所有基础URL）
    fn get_initial_urls(&self) -> Result<Vec<String>> {
        let mut urls = Vec::new();

        // 添加主基础URL的初始路径
        for path in &self.initial_paths {
            urls.push(self.normalize_url(&self.base_url, path)?);
        }

        // 添加其他基础URL
        if let Some(ref base_urls) = self.base_urls {
            for base_url in base_urls.iter().skip(1) {
                urls.push(base_url.clone());
            }
        }

        Ok(urls)
    }

    /// 检查URL是否应该处理
    fn should_process_url(&self, url: &str) -> bool {
        // 从多个基础URL中检查
        let base_urls = self.get_base_urls();
        if !base_urls.iter().any(|base| url.starts_with(base)) {
            return false;
        }

        // 检查skip_link回调
        if let Some(ref skip_fn) = self.skip_link {
            if skip_fn(url) {
                return false;
            }
        }

        // 检查跳过路径和模式
        let path = self.url_to_path(url);

        // 检查跳过路径
        if self
            .skip_paths
            .iter()
            .any(|p| path == *p || path.starts_with(&format!("{}/", p)))
        {
            return false;
        }

        // 检查跳过模式
        for pattern in &self.skip_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if regex.is_match(&path) {
                    return false;
                }
            }
        }

        // 检查only路径和模式
        if let Some(ref only) = self.only {
            if !only
                .iter()
                .any(|p| path == *p || path.starts_with(&format!("{}/", p)))
            {
                return false;
            }
        }

        if let Some(ref only_patterns) = self.only_patterns {
            let mut match_any = false;
            for pattern in only_patterns {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(&path) {
                        match_any = true;
                        break;
                    }
                }
            }
            if !match_any {
                return false;
            }
        }

        true
    }

    /// 规范化URL
    fn normalize_url(&self, base_url: &str, path: &str) -> Result<String> {
        if path.starts_with("http://") || path.starts_with("https://") {
            return Ok(path.to_string());
        }

        let base = if base_url.ends_with("/") {
            base_url.to_string()
        } else {
            format!("{}/", base_url)
        };

        let normalized_path = if path.starts_with("/") {
            path[1..].to_string()
        } else {
            path.to_string()
        };

        Ok(format!("{}{}", base, normalized_path))
    }

    /// 从URL中提取路径
    fn url_to_path(&self, url: &str) -> String {
        // 查找匹配的基础URL
        let base_urls = self.get_base_urls();
        for base_url in &base_urls {
            if url.starts_with(base_url) {
                let path = url.trim_start_matches(base_url).trim_start_matches('/');
                if path.is_empty() {
                    return "index".to_string();
                } else {
                    return path.to_string();
                }
            }
        }

        // 如果没有匹配的基础URL，尝试解析URL
        match Url::parse(url) {
            Ok(parsed_url) => {
                let path = parsed_url.path().trim_start_matches('/');
                if path.is_empty() {
                    "index".to_string()
                } else {
                    path.to_string()
                }
            }
            Err(_) => "unknown".to_string(),
        }
    }

    /// 发送HTTP请求获取URL内容
    async fn fetch_url(&self, client: &Client, url: &str) -> Result<reqwest::Response> {
        client
            .get(url)
            .header("User-Agent", "DevDocs Rust Scraper")
            .send()
            .await
            .map_err(|e| Error::Message(format!("请求失败: {}", e)))
    }

    /// 检查响应是否应该处理
    fn should_process_response(&self, response: &reqwest::Response, url: &str) -> Result<bool> {
        // 检查状态码
        if !response.status().is_success() {
            return Ok(false);
        }

        // 检查内容类型
        if let Some(content_type) = response.headers().get(reqwest::header::CONTENT_TYPE) {
            if let Ok(content_type_str) = content_type.to_str() {
                if !content_type_str.contains("text/html") {
                    return Ok(false);
                }
            }
        }

        // 检查URL
        Ok(self.should_process_url(url))
    }

    /// 从HTML中提取链接
    fn extract_links(&self, html: &str, base_url: &str) -> Result<Vec<String>> {
        let mut urls = Vec::new();
        let document = scraper::Html::parse_document(html);

        // 查找所有链接
        if let Ok(selector) = scraper::Selector::parse("a[href]") {
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    // 规范化URL
                    if let Ok(normalized) = self.normalize_url(base_url, href) {
                        urls.push(normalized);
                    }
                }
            }
        }

        Ok(urls)
    }

    /// 创建条目
    fn create_entry(&self, path: &str) -> (String, String, String) {
        // 使用路径作为标题
        let title = path.to_string();

        // 条目路径
        let entry_path = path.to_string();

        // 条目类型 (默认为 "其他")
        let entry_type = "Other".to_string();

        (title, entry_path, entry_type)
    }

    // 更多实现方法...
}

#[async_trait::async_trait]
impl Scraper for UrlScraper {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    async fn run(&mut self) -> Result<()> {
        println!("Running URL scraper for: {}", self.base_url);

        // 确保输出目录存在
        let doc_dir = Path::new(&self.output_path).join(&self.slug);
        fs::create_dir_all(&doc_dir)
            .await
            .map_err(|e| Error::Message(format!("无法创建输出目录 {:?}: {}", doc_dir, e)))?;

        // 创建空的 entries.json 文件以便索引生成可以进行
        let entries_file = doc_dir.join("entries.json");
        fs::write(&entries_file, "[]")
            .await
            .map_err(|e| Error::Message(format!("无法创建 entries.json 文件: {}", e)))?;

        // 创建基本的 db.json 文件
        let db_file = doc_dir.join("db.json");
        fs::write(&db_file, "{}")
            .await
            .map_err(|e| Error::Message(format!("无法创建 db.json 文件: {}", e)))?;

        // 实现完整的抓取逻辑
        let client = Client::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut entries = Vec::new();
        let mut pages = HashMap::new();
        let redirections: Arc<Mutex<HashMap<String, String>>> =
            Arc::new(Mutex::new(HashMap::new()));

        // 是否限制速率（默认每分钟60次请求）
        let rate_limit = 60;
        let mut last_request_time = Instant::now();

        // 初始化要访问的URL
        let initial_urls = self.get_initial_urls()?;
        for url in initial_urls {
            queue.push_back(url);
        }

        // 广度优先搜索抓取页面
        while let Some(url) = queue.pop_front() {
            if visited.contains(&url) {
                continue;
            }

            // 检查是否应该处理该URL
            if !self.should_process_url(&url) {
                continue;
            }

            println!("爬取: {}", url);
            visited.insert(url.clone());

            // 实现简单的速率限制
            let elapsed = last_request_time.elapsed();
            if elapsed < Duration::from_millis(1000 * 60 / rate_limit as u64) {
                // 等待，确保不超过速率限制
                sleep(Duration::from_millis(1000 * 60 / rate_limit as u64) - elapsed).await;
            }
            last_request_time = Instant::now();

            // 发送HTTP请求
            match self.fetch_url(&client, &url).await {
                Ok(response) => {
                    // 更新重定向映射
                    let effective_url = response.url().to_string();
                    if effective_url != url {
                        let mut redirects = redirections.lock().await;
                        redirects.insert(url.clone(), effective_url.clone());
                    }

                    // 检查响应是否应该处理
                    if !self.should_process_response(&response, &url)? {
                        continue;
                    }

                    // 处理响应内容
                    let html = response
                        .text()
                        .await
                        .map_err(|e| Error::Message(format!("无法获取响应内容: {}", e)))?;

                    // 创建过滤上下文
                    let mut context = FilterContext {
                        options: HashMap::new(),
                        base_url: self.base_url.clone(),
                        links: Vec::new(),
                        root_url: self.base_url.clone(),
                        root_path: self.root_path.clone(),
                        version: self.version.clone(),
                        release: self.release.clone(),
                        initial_paths: self.initial_paths.clone(),
                        slug: self.slug.clone(),
                        current_path: self.url_to_path(&url),
                        current_url: url.clone(),
                        attribution: Some(self.attribution.clone()),
                        html: html.clone(),
                        title: String::new(),
                        content: String::new(),
                        additional_entries: Vec::new(),
                    };

                    // 应用所有过滤器
                    for filter in &self.filters {
                        // 从context获取当前HTML
                        let current_html = context.html.clone();
                        // 应用过滤器
                        let filtered_html = filter.apply(&current_html, &mut context)?;
                        // 更新context中的HTML
                        context.html = filtered_html;
                    }

                    // 提取新链接添加到队列
                    let new_urls = self.extract_links(&context.html, &url)?;
                    for new_url in new_urls {
                        if !visited.contains(&new_url) {
                            queue.push_back(new_url);
                        }
                    }

                    // 保存处理后的页面
                    if !context.content.is_empty() {
                        let path = self.url_to_path(&url);
                        let entry = self.create_entry(&path);
                        entries.push(entry);
                        pages.insert(path, context.content);
                    }

                    // 处理附加条目
                    for additional_entry in context.additional_entries {
                        entries.push(additional_entry);
                    }
                }
                Err(e) => {
                    println!("访问 {} 失败: {}", url, e);
                }
            }
        }

        // 保存条目到文件
        let entries_json = serde_json::to_string_pretty(&entries)
            .map_err(|e| Error::Message(format!("无法序列化条目数据: {}", e)))?;
        fs::write(&entries_file, entries_json)
            .await
            .map_err(|e| Error::Message(format!("无法写入 entries.json 文件: {}", e)))?;

        // 应用重定向修复到路径映射
        // 在这里，我们检查所有重定向，并更新页面路径映射
        let redirects = redirections.lock().await;
        let mut path_redirections = HashMap::new();

        // 处理重定向映射
        for (from_url, to_url) in redirects.iter() {
            // 从URL获取路径
            let from_path = self.url_to_path(from_url);
            let to_path = self.url_to_path(to_url);

            // 只有当路径不同时才添加重定向
            if from_path != to_path {
                path_redirections.insert(from_path.to_lowercase(), to_path);
            }
        }

        // 更新路径映射
        for (path, _) in pages.clone().iter() {
            if let Some(redirect_path) = path_redirections.get(&path.to_lowercase()) {
                if let Some(content) = pages.remove(path) {
                    pages.insert(redirect_path.clone(), content);
                }
            }
        }

        // 保存页面内容到数据库文件
        let db_json = serde_json::to_string_pretty(&pages)
            .map_err(|e| Error::Message(format!("无法序列化页面数据: {}", e)))?;
        fs::write(&db_file, db_json)
            .await
            .map_err(|e| Error::Message(format!("无法写入 db.json 文件: {}", e)))?;

        println!(
            "已完成抓取，处理了 {} 个页面，生成了 {} 个条目",
            pages.len(),
            entries.len()
        );
        println!("保存结果到: {:?}", doc_dir);
        Ok(())
    }
}
