//! 基础HTML清理过滤器
//!
//! 提供通用的HTML清理功能，可以被特定文档类型的过滤器继承

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use scraper::{Html, Node, Selector};
use std::any::Any;

/// 过滤器基础特质
pub trait FilterBase {}

/// 基础HTML清理过滤器
///
/// 该过滤器提供了基本的HTML清理功能，如移除脚本、样式、注释等。
/// 特定文档类型的清理过滤器应该继承此过滤器并根据需要扩展功能。
pub struct BaseCleanHtmlFilter;

impl BaseCleanHtmlFilter {
    /// 创建新的基础HTML清理过滤器
    pub fn new() -> Self {
        Self
    }

    /// 移除指定选择器匹配的元素
    pub fn remove_elements(&self, html: &str, selectors: &[&str]) -> String {
        let document = Html::parse_document(html);
        let mut result = html.to_string();

        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                // 查找匹配的元素并移除
                for element in document.select(&selector) {
                    let html_fragment = element.html();
                    result = result.replace(&html_fragment, "");
                }
            }
        }

        result
    }

    /// 处理代码块，提取语言信息和代码内容
    pub fn process_code_blocks(&self, html: &str) -> String {
        if let Ok(selector) = Selector::parse("pre") {
            let document = Html::parse_document(html);
            let mut new_html = String::new();

            document.select(&selector).for_each(|pre_node| {
                let mut pre_html = pre_node.html();

                // 提取语言信息
                if let Ok(lang_selector) = Selector::parse("[class*='language-']") {
                    if let Some(lang_node) = pre_node.select(&lang_selector).next() {
                        let class_attr = lang_node.value().attr("class").unwrap_or("");
                        let language = class_attr
                            .split_whitespace()
                            .find(|c| c.starts_with("language-"))
                            .and_then(|c| c.strip_prefix("language-"))
                            .unwrap_or("");

                        // 添加data-language属性
                        pre_html = pre_html
                            .replace("<pre", &format!("<pre data-language=\"{}\"", language));
                    }
                }

                // 提取代码内容
                if let Ok(token_line_selector) = Selector::parse(".token-line") {
                    let token_lines: Vec<String> = pre_node
                        .select(&token_line_selector)
                        .map(|node| node.text().collect::<String>())
                        .collect();

                    if !token_lines.is_empty() {
                        let code_content = token_lines.join("\n");
                        let start_pre = pre_html.find('>').map(|i| i + 1).unwrap_or(0);
                        let end_pre = pre_html.rfind("</pre>").unwrap_or(pre_html.len());
                        pre_html = format!(
                            "{}{}{}",
                            &pre_html[..start_pre],
                            code_content,
                            &pre_html[end_pre..]
                        );
                    }
                }

                new_html.push_str(&pre_html);
            });

            if !new_html.is_empty() {
                return new_html;
            }
        }

        html.to_string()
    }

    /// 移除所有元素的class和style属性
    pub fn remove_attributes(&self, html: &str, attributes: &[&str]) -> String {
        let document = Html::parse_document(html);
        let mut result = html.to_string();

        for attr in attributes {
            // 使用简单的字符串替换来移除属性
            // 注意：这是一个简化的实现，对于复杂的HTML可能不够健壮
            let pattern = format!(r#" {}="[^"]*""#, attr);
            result = result.replace(&pattern, "");
        }

        result
    }
}

impl FilterBase for BaseCleanHtmlFilter {}

impl Filter for BaseCleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // 基本的HTML清理 - 移除脚本、样式和注释
        let selectors_to_remove = ["script", "style", "link", "comment()"];
        let html = self.remove_elements(html, &selectors_to_remove);

        // 处理代码块
        let html = self.process_code_blocks(&html);

        // 移除class和style属性
        let html = self.remove_attributes(&html, &["class", "style"]);

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
