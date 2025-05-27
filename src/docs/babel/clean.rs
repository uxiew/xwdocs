use eyre::Result;
use scraper::{Html, Selector, ElementRef};
use regex::Regex;

use crate::core::scraper::filter::Filter; // Assuming Filter trait's path

#[derive(Debug, Default, Clone)] // Added Clone for potential future use if needed by Filter trait context
pub struct BabelCleanHtmlFilter;

impl BabelCleanHtmlFilter {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Filter for BabelCleanHtmlFilter {
    fn process(&self, document: Html) -> Result<Html> {
        // 1. Select main content
        let main_content_selector = Selector::parse(".theme-doc-markdown")
            .map_err(|e| eyre::eyre!("Failed to parse .theme-doc-markdown selector: {}", e))?;
        
        let mut html_string = if let Some(main_content_node) = document.select(&main_content_selector).next() {
            main_content_node.html()
        } else {
            // If .theme-doc-markdown is not found, process (and potentially clean) the whole document's HTML.
            // Or, depending on requirements, one might return an error or an empty document.
            // For this implementation, we'll use the original document's full HTML.
            document.root_element().html()
        };

        // 2. Remove unnecessary elements (using string manipulation with regex for simplicity here)
        // More robust parsing might be needed for complex cases.
        let selectors_to_remove_patterns = [
            // Simple class/element removals. Complex selectors like ">" are harder with simple regex.
            r#"(?s)<div\s[^>]*class="[^"]*fixedHeaderContainer[^"]*"[^>]*>.*?</div>"#,
            r#"(?s)<div\s[^>]*class="[^"]*toc[^"]*"[^>]*>.*?</div>"#, // Matches .toc
            r#"(?s)<div\s[^>]*class="[^"]*toc-headings[^"]*"[^>]*>.*?</div>"#, // Matches .toc-headings
            r#"(?s)<nav\s[^>]*class="[^"]*nav-footer[^"]*"[^>]*>.*?</nav>"#, // Matches .nav-footer (assuming it's a nav)
            // For .postHeader > a, it's complex. A simple regex might remove all <a> within .postHeader.
            // A proper parser-based approach would be better. This is a simplification.
            // Example: r#"(?s)(<div\s[^>]*class="[^"]*postHeader[^"]*"[^>]*>)\s*<a[^>]*>.*?</a>"#, (and then keep group 1)
            r#"(?s)<div\s[^>]*class="[^"]*docs-prevnext[^"]*"[^>]*>.*?</div>"#,
            r#"(?s)<div\s[^>]*class="[^"]*codeBlockTitle_x_ju[^"]*"[^>]*>.*?</div>"#,
        ];

        for pattern_str in &selectors_to_remove_patterns {
            let regex = Regex::new(pattern_str)
                .map_err(|e| eyre::eyre!("Failed to compile regex pattern '{}': {}", pattern_str, e))?;
            html_string = regex.replace_all(&html_string, "").into_owned();
        }
        
        // Special handling for .postHeader > a (conceptual, needs proper parsing or very careful regex)
        // This regex is a placeholder for a more complex operation:
        // It aims to remove direct child 'a' tags of '.postHeader'.
        // A simple regex for this is difficult and error-prone.
        // For now, we'll skip robustly implementing this specific complex selector via regex.

        // 3. Process code blocks
        // This requires parsing the current html_string to find <pre> elements,
        // modifying them, and then serializing back or carefully replacing parts of the string.
        // This is a complex operation to do with string manipulation alone if not careful.
        // Let's try a regex-based approach for <pre> blocks.
        // This will be a multi-stage regex process.
        
        // Regex to find <pre class="language-...">...</pre>
        let pre_block_regex = Regex::new(r#"(?s)<pre\s*(?:class="[^"]*language-([^"\s]+)[^"]*")?[^>]*>(.*?)</pre>"#)
            .map_err(|e| eyre::eyre!("Failed to compile pre_block_regex: {}", e))?;
        
        html_string = pre_block_regex.replace_all(&html_string, |caps: &regex::Captures| {
            let lang_attr = caps.get(1).map_or("".to_string(), |m| format!(r#"data-language="{}""#, m.as_str()));
            let original_pre_content = caps.get(2).map_or("", |m| m.as_str());

            // Parse the inner content of <pre> to find .token-line
            let pre_content_doc = Html::parse_fragment(original_pre_content);
            let token_line_selector = Selector::parse(".token-line").unwrap(); // Safe unwrap for known selector
            
            let new_inner_content = pre_content_doc
                .select(&token_line_selector)
                .map(|token_line_el| token_line_el.text().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n"); // Join lines with newline

            format!("<pre {}><code {}>{}</code></pre>", lang_attr, lang_attr, html_escape::encode_text(&new_inner_content))
        }).into_owned();


        // 4. Remove class and style attributes from all elements
        // Regex to remove class attribute
        let class_attr_regex = Regex::new(r#"\s*class="[^"]*""#)
            .map_err(|e| eyre::eyre!("Failed to compile class_attr_regex: {}", e))?;
        html_string = class_attr_regex.replace_all(&html_string, "").into_owned();

        // Regex to remove style attribute
        let style_attr_regex = Regex::new(r#"\s*style="[^"]*""#)
            .map_err(|e| eyre::eyre!("Failed to compile style_attr_regex: {}", e))?;
        html_string = style_attr_regex.replace_all(&html_string, "").into_owned();

        // Re-parse the modified HTML string into an Html document
        let final_document = Html::parse_fragment(&html_string);
        Ok(final_document)
    }
}

// Note: The Filter trait might require other methods like `box_clone` or `as_any`
// depending on its full definition in `crate::core::scraper::filter::Filter`.
// If so, those would need to be implemented as well.
// For example:
// impl Clone for Box<dyn Filter> {
//     fn clone(&self) -> Box<dyn Filter> {
//         self.box_clone()
//     }
// }
// This assumes the trait definition requires `Clone` and provides `box_clone`.
// The actual implementation details for those would depend on the trait definition.
// Without the exact trait definition, this part is speculative.
// The core logic is in `process`.
```
