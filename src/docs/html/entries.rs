//! HTML文档条目过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use scraper::{Html, Selector, Element};
use regex::Regex;
use std::any::Any;

const ADDITIONAL_ENTRIES: &[(&str, &[&str])] = &[
    ("Element/Heading_Elements", &["h1", "h2", "h3", "h4", "h5", "h6"])
];

/// HTML文档条目过滤器
pub struct HtmlEntriesFilter;

impl HtmlEntriesFilter {
    /// 创建新的条目过滤器
    pub fn new() -> Self {
        HtmlEntriesFilter
    }

    fn get_name(&self, _doc: &Html, slug: &str) -> String { // doc changed to _doc
        let mut name = slug.replace('_', " ")
            .replace('/', ".")
            .trim()
            .to_string();

        name = name.replace("Element.", "").to_lowercase();
        if name.starts_with("Global attributes.") {
            name = name.replace("Global attributes.", "");
            name.push_str(" (attribute)");
        }
        if let Some(captures) = Regex::new(r"input\.([-\w]+)").unwrap().captures(&name) {
            if let Some(input_type) = captures.get(1) {
                name = format!("input type=\"{}\"", input_type.as_str());
            }
        }
        name
    }

    fn get_type(&self, doc: &Html, slug: &str) -> Option<String> {
        if slug.contains("CORS") || slug.contains("Using") {
            return Some("Miscellaneous".to_string());
        }

        if let Ok(selector) = Selector::parse(".deprecated, .non-standard, .obsolete") {
            if doc.select(&selector).next().is_some() {
                return Some("Obsolete".to_string());
            }
        }

        if slug.starts_with("Global_attr") {
            Some("Attributes".to_string())
        } else if slug.starts_with("Element/") {
            Some("Elements".to_string())
        } else {
            Some("Miscellaneous".to_string())
        }
    }

    fn include_default_entry(&self, slug: &str, doc: &Html) -> bool {
        if slug == "Element/Heading_Elements" {
            return false;
        }

        if let Ok(selector) = Selector::parse(".overheadIndicator, .blockIndicator") {
            if let Some(node) = doc.select(&selector).next() {
                let content = node.text().collect::<String>();
                if content.contains("not on a standards track") {
                    return false;
                }
            }
        }
        true
    }

    fn additional_entries(&self, doc: &Html, slug: &str) -> Vec<(String, String, String)> {
        // 检查预定义的额外条目
        for (entry_slug, elements) in ADDITIONAL_ENTRIES {
            if *entry_slug == slug {
                return elements.iter()
                    .map(|&tag| (tag.to_string(), tag.to_string(), "Elements".to_string()))
                    .collect();
            }
        }

        if slug == "Attributes" {
            let mut entries = Vec::new();
            if let Ok(selector) = Selector::parse(".standard-table td:first-child") {
                for node in doc.select(&selector) {
                    let next_content = node.next_sibling_element().map_or("".to_string(), |e| e.text().collect());
                    if next_content.contains("Global attribute") {
                        continue;
                    }

                    let mut name = if let Some(code) = node.select(&Selector::parse("code").unwrap()).next() {
                        code.text().collect::<String>().trim().to_string()
                    } else {
                        node.text().collect::<String>().trim().to_string()
                    };
                    name.push_str(" (attribute)");
                    let id = name.to_lowercase().replace(' ', "-");
                    entries.push((name, id, "Attributes".to_string()));
                }
            }
            entries
        } else if slug == "Link_types" {
            let mut entries = Vec::new();
            if let Ok(selector) = Selector::parse(".standard-table td:first-child > code") {
                for node in doc.select(&selector) {
                    let name = format!("rel: {}", node.text().collect::<String>().trim());
                    let id = name.to_lowercase().replace(' ', "-");
                    entries.push((name, id, "Attributes".to_string()));
                }
            }
            entries
        } else {
            Vec::new()
        }
    }

    fn build_entry(&self, name: String, fragment: Option<String>, entry_type: Option<String>, context: &FilterContext) -> (String, String, String) {
        let path = if let Some(frag) = fragment {
            if frag.contains('#') {
                frag
            } else {
                format!("{}#{}", context.current_path, frag)
            }
        } else {
            context.current_path.clone()
        };

        (name, path, entry_type.unwrap_or_else(|| "Element".to_string()))
    }
}

impl Filter for HtmlEntriesFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> { // context changed to _context
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(HtmlEntriesFilter::new())
    }

    fn get_entries(&self, html: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let slug = &context.current_path;
        let _is_root = slug.is_empty() || slug == "/" || slug == &context.root_path; // is_root changed to _is_root

        let mut entries = Vec::new();
        let doc = Html::parse_document(html);

        if self.include_default_entry(slug, &doc) {
            let name = self.get_name(&doc, slug);
            if let Some(entry_type) = self.get_type(&doc, slug) {
                entries.push((name, slug.to_string(), entry_type));
            }
        }

        entries.extend(self.additional_entries(&doc, slug));
        entries
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
