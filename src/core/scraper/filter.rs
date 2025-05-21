//! HTML过滤器定义
//! 参考 Ruby 版本 filter.rb 重新设计

use crate::core::error::Result;
use scraper::{ElementRef, Html, Selector};
use std::any::Any;
use std::collections::HashMap;

/// 过滤器上下文，包含过滤时需要的上下文信息
#[derive(Default, Clone)]
pub struct FilterContext {
    /// 过滤器选项
    pub options: HashMap<String, String>,
    /// 基础URL
    pub base_url: String,
    /// 链接列表
    pub links: Vec<String>,

    /// 根URL
    pub root_url: String,
    /// 根路径
    pub root_path: String,
    /// 版本
    pub version: String,
    /// 发布版本
    pub release: String,
    /// 初始路径列表
    pub initial_paths: Vec<String>,
    /// 当前文档的slug
    pub slug: String,
    /// 当前页面的路径
    pub current_path: String,
    /// 当前页面的URL
    pub current_url: String,
    /// 归属信息
    pub attribution: Option<String>,

    /// 当前页面的HTML内容
    pub html: String,
    /// 页面标题
    pub title: String,
    /// 处理后的内容
    pub content: String,
    /// 附加条目
    pub additional_entries: Vec<(String, String, String)>,
}

impl FilterContext {
    /// 创建新的过滤器上下文
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置过滤器选项
    pub fn with_option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }

    /// 获取过滤器选项
    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }
}

/// 对HTML内容进行过滤函数的特质
pub trait Filter: Send + Sync + 'static {
    /// 应用过滤器到HTML内容
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String>;

    /// 创建过滤器的克隆
    fn box_clone(&self) -> Box<dyn Filter>;

    /// 获取所有匹配的元素
    fn css<'a>(&self, doc: &'a Html, selector: &str) -> Vec<ElementRef<'a>> {
        Selector::parse(selector)
            .map(|selector| doc.select(&selector).collect())
            .unwrap_or_default()
    }

    /// 获取第一个匹配的元素
    fn at_css<'a>(&self, doc: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
        Selector::parse(selector)
            .ok()
            .and_then(move |selector| doc.select(&selector).next())
    }

    /// 获取当前URL的子路径
    fn subpath(&self, context: &FilterContext) -> String {
        self.subpath_to(context, &context.root_url)
    }

    /// 获取指定URL的子路径
    fn subpath_to(&self, context: &FilterContext, url: &str) -> String {
        if let Some(base) = url.strip_prefix(&context.base_url) {
            base.trim_start_matches('/').to_string()
        } else {
            url.to_string()
        }
    }

    /// 是否为根页面
    fn root_page(&self, context: &FilterContext) -> bool {
        let subpath = self.subpath(context);
        subpath.is_empty() || subpath == "/" || subpath == context.root_path
    }

    /// 是否为初始页面
    fn initial_page(&self, context: &FilterContext) -> bool {
        self.root_page(context) || context.initial_paths.contains(&self.subpath(context))
    }

    /// 是否为片段URL (以 # 开头)
    fn is_fragment_url(&self, url: &str) -> bool {
        url.starts_with('#')
    }

    /// 是否为数据URL (以 data: 开头)
    fn is_data_url(&self, url: &str) -> bool {
        url.starts_with("data:")
    }

    /// 是否为相对URL
    fn is_relative_url(&self, url: &str) -> bool {
        !url.contains("://") && !self.is_fragment_url(url) && !self.is_data_url(url)
    }

    /// 是否为绝对URL
    fn is_absolute_url(&self, url: &str) -> bool {
        url.contains("://")
    }

    /// 是否为内部URL
    fn is_internal_url(&self, url: &str, context: &FilterContext) -> bool {
        if self.is_relative_url(url) {
            true
        } else if let Some(base) = url.strip_prefix(&context.base_url) {
            !base.is_empty() && base != "/"
        } else {
            false
        }
    }

    /// 获取条目信息
    /// 默认返回空集合，但子类可以重写以提供条目信息
    ///
    /// # 参数
    ///
    /// * `html` - 要处理的HTML内容字符串
    /// * `context` - 过滤器上下文，包含URL、路径等信息
    ///
    /// # 返回
    ///
    /// 返回一个元组向量，每个元组包含三个字符串：
    /// * 条目名称
    /// * 条目路径
    /// * 条目类型
    fn get_entries(&self, _html: &str, _context: &FilterContext) -> Vec<(String, String, String)> {
        Vec::new()
    }

    /// 允许 trait 对象进行类型转换和 downcast
    fn as_any(&self) -> &dyn Any;

    /// 允许 trait 对象进行可变类型转换和 downcast
    fn as_any_mut(&mut self) -> &mut dyn Any {
        panic!("as_any_mut not implemented for this filter");
    }
}

/// 默认的过滤器实现，用于快速创建不需要特殊处理的过滤器
pub struct DefaultFilter<T: Clone + 'static>(pub T);

impl<T: Clone + 'static> DefaultFilter<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

impl<T: Clone + 'static + Send + Sync> Filter for DefaultFilter<T> {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self(self.0.clone()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
