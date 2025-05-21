//! JavaScript条目过滤器

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
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self { path_prefix: self.path_prefix.clone() })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_entries(&self, html: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let doc = Html::parse_document(html);
        let slug = &context.slug;

        let mut entries = Vec::new();

        // 收集条目信息
        if slug == "index" || slug.is_empty() || slug == "/" || slug == "Global_Objects" {
            // 处理主页和全局对象页面
            for a in self.css(&doc, "a") {
                if let Some(href) = a.value().attr("href") {
                    if href.starts_with(&self.path_prefix) {
                        let name = a.text().collect::<String>().trim().to_string();
                        if !name.is_empty() {
                            entries.push((name, href.to_string(), "entry".to_string()));
                        }
                    }
                }
            }
        } else if slug == "Reference" || slug.contains("Reference/") {
            // 处理引用页面
            let prefix = "/en-US/docs/Web/JavaScript/Reference/";

            // 1. Global Objects
            for a in self.css(&doc, "a[href^='/en-US/docs/Web/JavaScript/Reference/Global_Objects/']") {
                if let Some(href) = a.value().attr("href") {
                    let name = a.text().collect::<String>().trim().to_string();
                    if !name.is_empty() {
                        entries.push((name, href.to_string(), "Object".to_string()));
                    }
                }
            }

            // 2. Statements
            for a in self.css(&doc, "a[href^='/en-US/docs/Web/JavaScript/Reference/Statements/']") {
                if let Some(href) = a.value().attr("href") {
                    let name = a.text().collect::<String>().trim().to_string();
                    if !name.is_empty() {
                        entries.push((name, href.to_string(), "Statement".to_string()));
                    }
                }
            }

            // 3. Operators
            for a in self.css(&doc, "a[href^='/en-US/docs/Web/JavaScript/Reference/Operators/']") {
                if let Some(href) = a.value().attr("href") {
                    let name = a.text().collect::<String>().trim().to_string();
                    if !name.is_empty() {
                        entries.push((name, href.to_string(), "Operator".to_string()));
                    }
                }
            }

            // 4. Functions
            for a in self.css(&doc, "a[href^='/en-US/docs/Web/JavaScript/Reference/Functions/']") {
                if let Some(href) = a.value().attr("href") {
                    let name = a.text().collect::<String>().trim().to_string();
                    if !name.is_empty() {
                        entries.push((name, href.to_string(), "Function".to_string()));
                    }
                }
            }
        } else {
            // 处理单个条目页面
            let name = self.get_name(&doc, slug);
            let entry_type = self.get_type(&doc);
            entries.push((name, context.current_url.clone(), entry_type));
        }

        entries
    }
}
