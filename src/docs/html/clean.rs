//! HTML文档清理过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::filters::FilterBase;
use crate::core::scraper::filter::{Filter, FilterContext};
use scraper::{Html, Selector};
use std::any::Any;

/// 清理HTML的过滤器，移除不必要的元素和属性
pub struct CleanHtmlFilter;

impl CleanHtmlFilter {
    /// 创建新的清理过滤器
    pub fn new() -> Self {
        Self
    }
}

impl FilterBase for CleanHtmlFilter {}

impl Filter for CleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // Babel的HTML是由minify压缩的，使用Fragment解析效果更好
        let document = Html::parse_fragment(html);
        let mut output = String::new();

        // 使用CSS选择器查找和处理节点
        let selector = Selector::parse("section, div.section, div.row").unwrap_or_else(|_| {
            Selector::parse("body").unwrap() // 使用一个简单的回退选择器
        });

        let nodes = document.select(&selector);
        for node in nodes {
            let html_fragment = node.html();
            let children_html = node.inner_html();
            output = if output.is_empty() {
                html.replace(&html_fragment, &children_html)
            } else {
                output.replace(&html_fragment, &children_html)
            };
        }

        if output.is_empty() {
            output = html.to_string();
        }
        Ok(output)
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self::new())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
