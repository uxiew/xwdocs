use eyre::Result;
use scraper::{Html, Selector};
use std::any::Any;

use crate::core::scraper::filter::{Filter, FilterContext};
use phf::phf_map;

// NOTE: Using phf crate requires adding `phf = { version = "0.11", features = ["macros"] }` to Cargo.toml
// and potentially `phf_codegen` to build-dependencies if not using the macros feature directly.

static ENTRIES: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "Usage" => &["Options", "Plugins", "Config Files", "Compiler assumptions", "@babel/cli", "@babel/polyfill", "@babel/plugin-transform-runtime", "@babel/register"],
    "Presets" => &["@babel/preset"],
    "Tooling" => &["@babel/parser", "@babel/core", "@babel/generator", "@babel/code-frame", "@babel/helper", "@babel/runtime", "@babel/template", "@babel/traverse", "@babel/types", "@babel/standalone"],
};

const DEFAULT_TYPE: &str = "Guide";
const PLUGIN_TYPE: &str = "Other Plugins";

#[derive(Debug, Default, Clone)]
pub struct BabelEntriesFilter;

impl BabelEntriesFilter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Filter for BabelEntriesFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // This filter does not modify the HTML content itself.
        // Entry extraction happens in `get_entries`.
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }

    fn get_entries(&self, html_str: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let document = Html::parse_document(html_str);
        let mut entries_vec = Vec::new();

        // 1. Extract Entry Name (from <h1>)
        let h1_selector = Selector::parse("h1").expect("Invalid h1 selector"); // Should not fail for "h1"
        let name = if let Some(h1_element) = document.select(&h1_selector).next() {
            h1_element.text().collect::<String>().trim().to_string()
        } else {
            // If no h1, cannot determine a name, so return no entries.
            // Alternative: use context.title or other fallback if appropriate.
            return entries_vec; 
        };

        // If name is empty after trimming, it's not a valid entry.
        if name.is_empty() {
            return entries_vec;
        }

        // 2. Determine Entry Type
        let mut entry_type: Option<String> = None;

        // Check against ENTRIES map
        for (category, prefixes) in ENTRIES.into_iter() {
            if prefixes.iter().any(|prefix| name.starts_with(prefix)) {
                entry_type = Some(category.to_string());
                break;
            }
        }

        // If not found, check subpath for "babel-plugin"
        if entry_type.is_none() {
            // context.current_path is the relative path of the file/page being processed.
            // This serves as the 'subpath'.
            if context.current_path.contains("babel-plugin") {
                entry_type = Some(PLUGIN_TYPE.to_string());
            }
        }
        
        // Assign default type if still not determined
        let final_entry_type = entry_type.unwrap_or_else(|| DEFAULT_TYPE.to_string());

        // The 'path' for the entry is typically the path of the current document.
        // The FilterContext provides `current_path`.
        let path = context.current_path.clone(); 

        entries_vec.push((name, path, final_entry_type));
        
        entries_vec
    }

    // Standard trait methods for dynamic dispatch and type introspection
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        // As FilterContext is not taken mutably in get_entries, 
        // and this filter itself has no state,
        // a mutable reference to self might not be strictly necessary for this filter's logic.
        // However, the trait requires it.
        self
    }
}
