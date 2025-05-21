//! 基础过滤器基类
//! 严格按照 Ruby 原版 filter.rb 实现

use crate::core::scraper::filter::{Filter, FilterContext};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

/// 正则表达式常量：URL 协议模式
const SCHEME_RGX: &str = r"\A[^:\/?#]+:";
/// 数据 URL 前缀
const DATA_URL: &str = "data:";

/// 基础过滤器类型
/// 提供一套通用的过滤器功能，模拟原版 Ruby Filter 类
pub trait FilterBase: Filter {
    /// 获取当前文档
    fn doc(&self, html: &str) -> Html {
        Html::parse_document(html)
    }

    /// 选择所有匹配的元素
    fn css<'a>(&self, doc: &'a Html, selector: &str) -> Vec<ElementRef<'a>> {
        match Selector::parse(selector) {
            Ok(selector) => doc.select(&selector).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// 选择第一个匹配的元素
    fn at_css<'a>(&self, doc: &'a Html, selector: &str) -> Option<ElementRef<'a>> {
        Selector::parse(selector)
            .ok()
            .and_then(|selector| doc.select(&selector).next())
    }

    /// XPath 查询
    fn xpath<'a>(&self, _doc: &'a Html, _xpath: &str) -> Vec<ElementRef<'a>> {
        // 注意：scraper 不直接支持 XPath，需要额外的库
        // 为了兼容性，这里返回空数组
        Vec::new()
    }

    /// XPath 查询第一个匹配的元素
    fn at_xpath<'a>(&self, _doc: &'a Html, _xpath: &str) -> Option<ElementRef<'a>> {
        // 同上，不直接支持 XPath
        None
    }

    /// 获取基础 URL
    fn base_url<'a>(&self, context: &'a FilterContext) -> &'a str {
        &context.base_url
    }

    /// 获取链接
    fn links<'a>(&self, context: &'a FilterContext) -> &'a [String] {
        &context.links
    }

    /// 获取当前 URL
    fn current_url<'a>(&self, context: &'a FilterContext) -> &'a str {
        &context.current_url
    }

    /// 获取根 URL
    fn root_url<'a>(&self, context: &'a FilterContext) -> &'a str {
        &context.root_url
    }

    /// 获取根路径
    fn root_path<'a>(&self, context: &'a FilterContext) -> &'a str {
        &context.root_path
    }

    /// 获取版本
    fn version<'a>(&self, context: &'a FilterContext) -> &'a str {
        &context.version
    }

    /// 获取发布版本
    fn release<'a>(&self, context: &'a FilterContext) -> &'a str {
        &context.release
    }

    /// 获取当前 URL 的子路径
    fn filter_subpath(&self, context: &FilterContext) -> String {
        self.filter_subpath_to(context, &context.current_url)
    }

    /// 获取指定 URL 的子路径
    fn filter_subpath_to(&self, context: &FilterContext, url: &str) -> String {
        if url.starts_with(&context.base_url) {
            let base_len = context.base_url.len();
            let subpath = &url[base_len..];
            if !subpath.starts_with('/') {
                format!("/{}", subpath)
            } else {
                subpath.to_string()
            }
        } else {
            url.to_string()
        }
    }

    /// 获取 slug (移除开头的 / 和结尾的 .html)
    fn filter_slug(&self, context: &FilterContext) -> String {
        let subpath = self.filter_subpath(context);
        let without_prefix = subpath.trim_start_matches('/');
        let without_suffix = without_prefix.trim_end_matches(".html");
        without_suffix.to_string()
    }

    /// 检查是否为根页面
    fn filter_root_page(&self, context: &FilterContext) -> bool {
        let subpath = self.filter_subpath(context);
        subpath.is_empty() || subpath == "/" || subpath == self.root_path(context)
    }

    /// 检查是否为初始页面
    fn filter_initial_page(&self, context: &FilterContext) -> bool {
        self.filter_root_page(context)
            || context
                .initial_paths
                .contains(&self.filter_subpath(context))
    }

    /// 检查是否为片段 URL (# 开头)
    fn fragment_url_string(&self, str: &str) -> bool {
        !str.is_empty() && str.starts_with('#')
    }

    /// 检查是否为数据 URL (data: 开头)
    fn data_url_string(&self, str: &str) -> bool {
        str.starts_with(DATA_URL)
    }

    /// 检查是否为相对 URL
    fn relative_url_string(&self, str: &str) -> bool {
        let scheme_regex = Regex::new(SCHEME_RGX).unwrap();
        !scheme_regex.is_match(str) && !self.fragment_url_string(str) && !self.data_url_string(str)
    }

    /// 检查是否为绝对 URL
    fn absolute_url_string(&self, str: &str) -> bool {
        let scheme_regex = Regex::new(SCHEME_RGX).unwrap();
        scheme_regex.is_match(str)
    }

    /// 清理路径 (替换特殊字符)
    fn clean_path(&self, path: &str) -> String {
        let path = path.replace(|c| c == '!' || c == ';' || c == ':', "-");
        path.replace("+", "_plus_")
    }
}

// 注意：不再自动实现 Filter trait，避免与具体过滤器实现冲突
// 每个 FilterBase 的实现者必须自己实现 Filter trait
