// src/docs/babel/tests.rs

// Allow dead_code for test utilities that might not be used in all test functions
#![allow(dead_code)]

use scraper::Html;
use std::fs;
use std::path::PathBuf;

use crate::core::scraper::filter::{Filter, FilterContext};
use crate::docs::babel::clean::BabelCleanHtmlFilter;
use crate::docs::babel::entries::BabelEntriesFilter;
// If BabelScraper itself is tested for filter application:
// use crate::docs::babel::BabelScraper; 

// Helper function to load HTML from test data files
fn load_test_html(filename: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test_docs/babel_test_data");
    path.push(filename);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read test HTML file {:?}: {}", path, e))
}

#[test]
fn test_babel_clean_html_filter_parser_page() {
    let html_content = load_test_html("parser_page.html");
    let document = Html::parse_document(&html_content);

    let filter = BabelCleanHtmlFilter::new();
    let cleaned_document = filter.process(document).expect("Filter processing failed");
    let cleaned_html = cleaned_document.root_element().inner_html(); // Get HTML of content within .theme-doc-markdown or full

    // Assertions for parser_page.html:
    // 1. Main content selector ".theme-doc-markdown" is applied (implicitly, as filter processes its content or whole doc)
    //    The filter itself extracts .theme-doc-markdown, so cleaned_html is its content.

    // 2. Unnecessary elements removed
    assert!(!cleaned_html.contains("<nav class=\"toc\">"), "TOC should be removed");
    assert!(!cleaned_html.contains("Table of Contents Here"), "TOC content should be removed");
    assert!(!cleaned_html.contains("<div class=\"nav-footer\">"), "Nav footer should be removed");
    assert!(!cleaned_html.contains("Footer Nav"), "Nav footer content should be removed");
    assert!(!cleaned_html.contains("<div class=\"fixedHeaderContainer\">"), "FixedHeaderContainer should be removed");

    // 3. Code blocks processed
    //    <pre class="language-javascript"> -> <pre data-language="javascript"><code>...</code></pre>
    assert!(cleaned_html.contains("<pre data-language=\"javascript\"><code data-language=\"javascript\">"), "Pre block should have data-language and inner code tag");
    assert!(cleaned_html.contains("const babelParser = require(&quot;@babel/parser&quot;);"), "Code content should be present and HTML escaped");
    assert!(!cleaned_html.contains("token-line"), "Class 'token-line' should be removed from pre content's transformation");
    assert!(!cleaned_html.contains("class=\"language-javascript\""), "Original class on pre should be removed/transformed");
    
    // 4. Attributes class and style removed (check on a sample element)
    //    <p style="color: red;" class="some-class">Styled paragraph.</p> -> <p>Styled paragraph.</p>
    assert!(cleaned_html.contains("<p>Styled paragraph.</p>"), "Style and class should be stripped from paragraph");
    assert!(!cleaned_html.contains("style="), "No style attributes should remain generally");
    assert!(!cleaned_html.contains("class="), "No class attributes should remain generally (except those added by processing like data-language)");
}

#[test]
fn test_babel_clean_html_filter_usage_page() {
    let html_content = load_test_html("usage_page.html");
    let document = Html::parse_document(&html_content);

    let filter = BabelCleanHtmlFilter::new();
    let cleaned_document = filter.process(document).expect("Filter processing failed");
    let cleaned_html = cleaned_document.root_element().inner_html();

    // Assertions for usage_page.html:
    assert!(!cleaned_html.contains("<div class=\"codeBlockTitle_x_ju\">"), "codeBlockTitle_x_ju should be removed");
    assert!(!cleaned_html.contains("Example with Title"), "codeBlockTitle content should be removed");
    assert!(!cleaned_html.contains("<div class=\"docs-prevnext\">"), "docs-prevnext should be removed");
    assert!(!cleaned_html.contains("<div class=\"toc-headings\">"), "toc-headings should be removed outside .theme-doc-markdown");
    
    // Check code block processing for bash
    assert!(cleaned_html.contains("<pre data-language=\"bash\"><code data-language=\"bash\">"), "Bash Pre block processed");
    assert!(cleaned_html.contains("# A bash example\nnpm install --save-dev @babel/core @babel/cli"), "Bash code content preserved");

    // Check attribute removal from <strong> and <code>
    assert!(cleaned_html.contains("<strong>strong emphasis</strong>"), "Strong tag content, no attributes");
    assert!(cleaned_html.contains("<code>inline code</code>"), "Inline code tag content, no attributes");
    assert!(!cleaned_html.contains("class=\"important-text\""), "Class on strong tag removed");
    assert!(!cleaned_html.contains("class=\"language-text\""), "Class on inline code tag removed");

}

#[test]
fn test_babel_entries_filter_parser_page() {
    let html_content = load_test_html("parser_page.html");
    // let document = Html::parse_document(&html_content); // Not needed as get_entries takes html_str

    let filter = BabelEntriesFilter::new();
    let mut context = FilterContext::default(); // Create a basic context
    context.current_path = "docs/babel-parser.html".to_string(); // Example path

    let entries = filter.get_entries(&html_content, &context);

    assert_eq!(entries.len(), 1, "Should extract one entry");
    let entry = &entries[0];
    assert_eq!(entry.0, "@babel/parser", "Entry name mismatch");
    assert_eq!(entry.1, "docs/babel-parser.html", "Entry path mismatch");
    assert_eq!(entry.2, "Tooling", "Entry type mismatch for @babel/parser");
}

#[test]
fn test_babel_entries_filter_usage_page() {
    let html_content = load_test_html("usage_page.html");

    let filter = BabelEntriesFilter::new();
    let mut context = FilterContext::default();
    context.current_path = "docs/usage.html".to_string();

    let entries = filter.get_entries(&html_content, &context);

    assert_eq!(entries.len(), 1, "Should extract one entry");
    let entry = &entries[0];
    assert_eq!(entry.0, "Babel Usage Guide", "Entry name mismatch");
    assert_eq!(entry.1, "docs/usage.html", "Entry path mismatch");
    // Based on ENTRIES map, "Babel Usage Guide" doesn't start with a specific prefix from "Usage", "Presets", or "Tooling" categories.
    // It doesn't contain "babel-plugin" in its path. So it should fall to DEFAULT_TYPE.
    assert_eq!(entry.2, "Guide", "Entry type mismatch for general guide");
}

#[test]
fn test_babel_entries_filter_plugin_page_by_path() {
    // Create a minimal HTML structure, as the name and type logic for plugins primarily depends on path
    let html_content = r#"
        <!DOCTYPE html>
        <html><head><title>Some Plugin</title></head>
        <body><div class="theme-doc-markdown"><h1>My Custom Plugin</h1></div></body></html>
    "#;
    // let document = Html::parse_document(&html_content);

    let filter = BabelEntriesFilter::new();
    let mut context = FilterContext::default();
    // This path simulates a babel plugin page
    context.current_path = "docs/plugins/babel-plugin-my-custom.html".to_string(); 

    let entries = filter.get_entries(&html_content, &context);

    assert_eq!(entries.len(), 1, "Should extract one entry for plugin page");
    let entry = &entries[0];
    assert_eq!(entry.0, "My Custom Plugin", "Plugin entry name mismatch");
    assert_eq!(entry.1, "docs/plugins/babel-plugin-my-custom.html", "Plugin entry path mismatch");
    assert_eq!(entry.2, "Other Plugins", "Entry type should be 'Other Plugins' due to path");
}

// Future test:
// #[tokio::test]
// async fn test_babel_scraper_get_latest_version() {
//     // This test would require mocking the network call to GitHub.
//     // For now, it's a placeholder.
//     // let scraper = BabelScraper::new("dummy_output", "dummy_version");
//     // let version = scraper.get_latest_version(None).await;
//     // assert!(version.is_ok());
//     // assert!(!version.unwrap().is_empty());
//     todo!("Implement test for get_latest_version with mocking");
// }
```
