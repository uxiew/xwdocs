//! 解析器模块
//!
//! 参考原始 Ruby 项目中的 parser.rb 实现
//! 提供 HTML 和 文档解析的功能

use crate::core::error::{Error, Result};
use html5ever::driver::ParseOpts;
use html5ever::tendril::{TendrilSink, StrTendril}; // Added StrTendril
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, parse_fragment, QualName, Namespace, LocalName}; // Ensured all are from html5ever
use html5ever::namespaces::HTML_NS; // For the HTML namespace constant
// Removed: use markup5ever::{Namespace, Atom}; 
use markup5ever_arcdom::{ArcDom, Handle, NodeData}; // This is for ArcDom, not core types for QualName
use regex::Regex;
use std::default::Default;
use std::str;
// use std::sync::Arc; // Removed unused import

/// HTML 解析器结构体
// #[derive(Debug)] // Removed derive(Debug)
pub struct Parser {
    /// 内部解析选项
    parse_opts: ParseOpts,
}

// Manual implementation of Debug for Parser
impl std::fmt::Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Parser")
         // .field("parse_opts", &self.parse_opts) // This would fail
         .field("parse_opts", &"<ParseOpts (not Debug)>") // Placeholder
         .finish()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    /// 创建新的解析器实例
    pub fn new() -> Self {
        let mut parse_opts = ParseOpts::default();
        parse_opts.tree_builder = TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        };

        Self { parse_opts }
    }

    /// 解析 HTML 文档
    pub fn parse(&self, html: &str) -> Result<ArcDom> {
        let dom = parse_document(ArcDom::default(), self.parse_opts.clone())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .map_err(|e| Error::ParseError(format!("HTML parsing error: {}", e)))?;

        Ok(dom)
    }

    /// 解析 HTML 片段
    pub fn parse_fragment(&self, html: &str, context_node_str: &str) -> Result<ArcDom> {
        // Construct QualName for the context node, assuming HTML namespace
        let local_name = LocalName::from(context_node_str); // Atom::from can take &str
        // HTML_NS is &'static Namespace, QualName::new expects Namespace by value.
        let context_qual_name = QualName::new(None, *HTML_NS, local_name); // This construction is correct as per instructions
        
        let dom = parse_fragment(
            ArcDom::default(),
            self.parse_opts.clone(),
            context_qual_name, // Pass the constructed QualName
            Vec::new(),
        )
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .map_err(|e| Error::ParseError(format!("HTML fragment parsing error: {}", e)))?;

        Ok(dom)
    }

    /// 清理 HTML
    pub fn clean_html(&self, html: &str) -> String {
        // 移除 doctype
        let re = Regex::new(r"<!DOCTYPE[^>]*>").unwrap();
        let html = re.replace_all(html, "");

        // 替换相对路径中的点号
        let mut result = html.to_string();
        for entity in &["&#46;", ".", "&#x2E;"] {
            result = result.replace(entity, ".");
        }

        result
    }
}

/// DOM 节点工具扩展
pub trait NodeExt {
    /// 获取节点的文本内容
    fn text_content(&self) -> String;
    
    /// 获取属性值
    fn attr(&self, name: &str) -> Option<String>;
}

impl NodeExt for Handle {
    fn text_content(&self) -> String {
        let node = self;
        let mut result = String::new();
        
        match &node.data {
            NodeData::Text { contents } => {
                result.push_str(&contents.borrow());
            }
            NodeData::Element { .. } => {
                for child in node.children.borrow().iter() {
                    result.push_str(&child.text_content());
                }
            }
            _ => {}
        }
        
        result
    }
    
    fn attr(&self, name: &str) -> Option<String> {
        if let NodeData::Element { attrs, .. } = &self.data {
            let attrs = attrs.borrow();
            for attr in attrs.iter() {
                if attr.name.local.as_ref() == name {
                    return Some(attr.value.to_string());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let parser = Parser::new();
        let html = "<html><body><div>Test</div></body></html>";
        let dom = parser.parse(html).unwrap();
        
        let document_node = dom.document;
        let html_node = document_node.children.borrow()[0].clone();
        let body_node = html_node.children.borrow()[1].clone();
        let div_node = body_node.children.borrow()[0].clone();
        
        assert_eq!(div_node.text_content(), "Test");
    }

    #[test]
    fn test_clean_html() {
        let parser = Parser::new();
        let html = "<!DOCTYPE html><html><body>Test</body></html>";
        let cleaned = parser.clean_html(html);
        
        assert_eq!(cleaned, "<html><body>Test</body></html>");
    }
}
