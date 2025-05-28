//! JavaScript HTML清理过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use scraper::Html;
use std::any::Any;

/// JavaScript HTML清理过滤器
pub struct JavaScriptCleanHtmlFilter;

impl JavaScriptCleanHtmlFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self
    }
}

impl Filter for JavaScriptCleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> { // context changed to _context
        let document = Html::parse_fragment(html);
        let mut output = String::new();

        // 1. 移除不需要的区域
        let nodes = self.css(&document, "section, div.section, div.row, div.notice, div.deprecated, div.obsolete");
        for node in nodes {
            let html_fragment = node.html();
            let children_html = node.inner_html();
            output = if output.is_empty() {
                html.replace(&html_fragment, &children_html)
            } else {
                output.replace(&html_fragment, &children_html)
            };
        }

        // 2. 移除examples链接
        if let Some(node) = self.at_css(&document, "a[href*='additional_examples']") {
            output = if output.is_empty() {
                html.replace(&node.html(), "")
            } else {
                output.replace(&node.html(), "")
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
