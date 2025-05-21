//! 参考 Ruby 版本 devdocs-original/lib/docs/filters/babel/entries.rb 设计
//! 简化版 Babel 条目过滤器实现

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use lazy_static::lazy_static;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::any::Any;

// 定义常量 ENTRIES，对应原Ruby版本的常量定义
lazy_static! {
    static ref ENTRIES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();

        map.insert(
            "Usage",
            vec![
                "Options",
                "Plugins",
                "Config Files",
                "Compiler assumptions",
                "@babel/cli",
                "@babel/polyfill",
                "@babel/plugin-transform-runtime",
                "@babel/register",
            ],
        );

        map.insert("Presets", vec!["@babel/preset"]);

        map.insert(
            "Tooling",
            vec![
                "@babel/parser",
                "@babel/core",
                "@babel/generator",
                "@babel/code-frame",
                "@babel/helper",
                "@babel/runtime",
                "@babel/template",
                "@babel/traverse",
                "@babel/types",
                "@babel/standalone",
            ],
        );

        map
    };
}

/// Babel 文档条目过滤器
/// 参考 Ruby 原版 babel/entries.rb 实现
pub struct BabelEntriesFilter {}

impl BabelEntriesFilter {
    /// 创建新的 Babel 条目过滤器
    pub fn new() -> Self {
        Self {}
    }

    /// 获取文档名称 - 对应Ruby原版的get_name方法
    fn get_name(&self, document: &Html) -> String {
        if let Ok(selector) = Selector::parse("h1") {
            if let Some(heading) = document.select(&selector).next() {
                return heading.text().collect::<Vec<_>>().join("");
            }
        }
        String::new()
    }

    /// 获取文档类型 - 对应Ruby原版的get_type方法
    fn get_type(&self, name: &str, subpath: &str) -> Option<String> {
        // 使用常量而不是临时变量，更接近原版实现
        for (type_name, values) in ENTRIES.iter() {
            if values.iter().any(|val| name.starts_with(val)) {
                return Some((*type_name).to_string());
            }
        }

        // 检查是否为插件
        if subpath.contains("babel-plugin") {
            return Some("Other Plugins".to_string());
        }

        None
    }
}

impl Filter for BabelEntriesFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // 条目过滤器不修改HTML内容
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self::new())
    }

    fn get_entries(&self, html: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let document = Html::parse_document(html);
        let name = self.get_name(&document);

        // 获取类型
        let entry_type = self
            .get_type(&name, &context.current_path)
            .unwrap_or_else(|| "Miscellaneous".to_string());

        // 创建条目 - 简单地返回当前页面作为唯一条目
        vec![(name, context.current_path.clone(), entry_type)]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
