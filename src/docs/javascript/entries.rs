//! JavaScript条目过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use scraper::Html;
use std::any::Any;

/// JavaScript条目过滤器
pub struct JavaScriptEntriesFilter {
    /// 输出路径前缀
    path_prefix: String,
}

impl JavaScriptEntriesFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self { path_prefix: "/en-US/docs/Web/JavaScript/".to_string() }
    }

    /// 创建带有路径前缀的过滤器
    pub fn with_path_prefix(path_prefix: String) -> Self {
        Self { path_prefix }
    }

    /// 获取条目名称
    fn get_name(&self, doc: &Html, slug: &str) -> String {
        if let Some(title) = self.at_css(doc, "h1") {
            title.text().collect::<String>().trim().to_string()
        } else {
            slug.replace('_', " ").trim().to_string()
        }
    }

    /// 获取条目类型
    fn get_type(&self, doc: &Html) -> String {
        if let Some(breadcrumb) = self.at_css(doc, ".breadcrumbs-container") {
            let text = breadcrumb.text().collect::<String>();
            if text.contains("Statements") {
                return "Statements".to_string();
            } else if text.contains("Operators") {
                return "Operators".to_string();
            } else if text.contains("Functions") {
                return "Functions".to_string();
            } else if text.contains("Global Objects") || text.contains("Classes") {
                return "Objects".to_string();
            }
        }
        "Others".to_string()
    }
}

impl Filter for JavaScriptEntriesFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> { // context changed to _context
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self::new())
    }

    fn get_entries(&self, html: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let doc = Html::parse_document(html);
        let name = self.get_name(&doc, &context.current_path);
        let entry_type = self.get_type(&doc);
        
        vec![(name, context.current_path.clone(), entry_type)]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
