//! Babel文档HTML清理过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::filters::BaseCleanHtmlFilter;
use crate::core::scraper::filter::{Filter, FilterContext};
use scraper::{Html, Selector};
use std::any::Any;

/// Babel 文档 HTML 清理过滤器
/// 参考 Ruby 原版 babel/clean_html.rb 实现
pub struct BabelCleanHtmlFilter {
    /// 基础清理过滤器
    base_filter: BaseCleanHtmlFilter,
}

impl BabelCleanHtmlFilter {
    /// 创建新的 Babel HTML 清理过滤器
    pub fn new() -> Self {
        Self {
            base_filter: BaseCleanHtmlFilter::new(),
        }
    }
}

impl Filter for BabelCleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        let mut document = Html::parse_document(html);

        // 获取主文档内容 - 对应原版的 @doc = at_css('.theme-doc-markdown')
        if let Ok(selector) = Selector::parse(".theme-doc-markdown") {
            if let Some(main_content) = document.select(&selector).next() {
                // 创建一个新的HTML文档仅包含主内容
                let main_html = main_content.html();
                document = Html::parse_document(&format!(
                    "<html><head></head><body>{}</body></html>",
                    main_html
                ));
            }
        }

        // 转换为字符串，使用基础过滤器提供的方法继续处理
        let html_str = document.html();

        // 移除指定的元素 - 严格按照原Ruby代码的顺序
        let elements_to_remove = [
            ".fixedHeaderContainer",
            ".toc",
            ".toc-headings",
            ".postHeader > a",
            ".nav-footer",
            ".docs-prevnext",
            ".codeBlockTitle_x_ju",
        ];

        // 使用基础过滤器的方法移除元素
        let html = self
            .base_filter
            .remove_elements(&html_str, &elements_to_remove);

        // 处理代码块
        let html = self.base_filter.process_code_blocks(&html);

        // 移除class和style属性
        let html = self
            .base_filter
            .remove_attributes(&html, &["class", "style"]);

        Ok(html)
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
