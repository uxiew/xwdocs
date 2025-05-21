//! HTML清理过滤器
//! 通用HTML清理功能

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use scraper::{Html, Selector};
use std::any::Any;

/// HTML清理过滤器
pub struct HtmlCleanerFilter {
    /// 要移除的标签列表
    remove_tags: Vec<String>,
    /// 要移除的属性列表
    remove_attrs: Vec<String>,
    /// 要移除的类列表
    remove_classes: Vec<String>,
}

impl HtmlCleanerFilter {
    /// 创建新的HTML清理过滤器
    pub fn new() -> Self {
        Self {
            remove_tags: Vec::new(),
            remove_attrs: Vec::new(),
            remove_classes: Vec::new(),
        }
    }

    /// 添加要移除的标签
    pub fn with_remove_tag(mut self, tag: &str) -> Self {
        self.remove_tags.push(tag.to_string());
        self
    }

    /// 添加要移除的属性
    pub fn with_remove_attr(mut self, attr: &str) -> Self {
        self.remove_attrs.push(attr.to_string());
        self
    }
    
    /// 添加要移除的类
    pub fn with_remove_class(mut self, class: &str) -> Self {
        self.remove_classes.push(class.to_string());
        self
    }
}

impl Filter for HtmlCleanerFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // 解析HTML
        let document = Html::parse_fragment(html);
        let mut result = html.to_string();

        // 移除指定的标签
        for tag in &self.remove_tags {
            if let Ok(selector) = Selector::parse(tag) {
                for element in document.select(&selector) {
                    let html_fragment = element.html();
                    result = result.replace(&html_fragment, "");
                }
            }
        }
        
        // 移除指定的类
        for class in &self.remove_classes {
            let selector_str = format!(".{}", class);
            // 创建局部变量，确保selector_str在使用时仍然存在
            let selector_result = Selector::parse(&selector_str);
            if let Ok(selector) = selector_result {
                for element in document.select(&selector) {
                    let html_fragment = element.html();
                    result = result.replace(&html_fragment, "");
                }
            }
        }

        // 返回处理后的HTML
        Ok(result)
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self {
            remove_tags: self.remove_tags.clone(),
            remove_attrs: self.remove_attrs.clone(),
            remove_classes: self.remove_classes.clone(),
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
