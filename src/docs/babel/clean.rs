// use eyre::Result; // Keep using eyre for internal errors, but trait signature will use core::error::Result
use crate::core::error::Result as CoreResult; // Alias for the project's Result type
use scraper::{Html, Selector}; 
use regex::Regex;
use std::any::Any; 

// Import the actual Filter trait and FilterContext
use crate::core::scraper::filter::{Filter, FilterContext};

#[derive(Debug, Default, Clone)] // Ensured Clone is derived
pub struct BabelCleanHtmlFilter;

impl BabelCleanHtmlFilter {
    pub fn new() -> Self {
        Default::default()
    }
}

// Reimplementing the Filter trait for BabelCleanHtmlFilter
impl Filter for BabelCleanHtmlFilter {
    fn apply(&self, html_input_str: &str, _context: &mut FilterContext) -> CoreResult<String> { // Changed return type
        // Internal logic can still use eyre::Result and `?` will convert via From impl
        let internal_result: eyre::Result<String> = (|| {
            // Parse the full HTML string first
            let document = Html::parse_document(html_input_str);

            // 1. Select main content
            let main_content_selector = Selector::parse(".theme-doc-markdown")
                .map_err(|e| eyre::eyre!("Failed to parse .theme-doc-markdown selector: {}", e))?;
            
            let mut effective_html_string = if let Some(main_content_node) = document.select(&main_content_selector).next() {
                main_content_node.html() // Work with the HTML of the main content
            } else {
                // If .theme-doc-markdown is not found, process the whole document's HTML string.
                html_input_str.to_string() 
            };

            // The rest of the cleaning logic uses regex on `effective_html_string`
            // This part is preserved from the previous `process` method's logic.

            // 2. Remove unnecessary elements
            let selectors_to_remove_patterns = [
                r#"(?s)<div\s[^>]*class="[^"]*fixedHeaderContainer[^"]*"[^>]*>.*?</div>"#,
                r#"(?s)<div\s[^>]*class="[^"]*toc[^"]*"[^>]*>.*?</div>"#,
                r#"(?s)<div\s[^>]*class="[^"]*toc-headings[^"]*"[^>]*>.*?</div>"#,
                r#"(?s)<nav\s[^>]*class="[^"]*nav-footer[^"]*"[^>]*>.*?</nav>"#,
                r#"(?s)<div\s[^>]*class="[^"]*docs-prevnext[^"]*"[^>]*>.*?</div>"#,
                r#"(?s)<div\s[^>]*class="[^"]*codeBlockTitle_x_ju[^"]*"[^>]*>.*?</div>"#,
            ];

            for pattern_str in &selectors_to_remove_patterns {
                let regex = Regex::new(pattern_str)
                    .map_err(|e| eyre::eyre!("Failed to compile regex pattern '{}': {}", pattern_str, e))?;
                effective_html_string = regex.replace_all(&effective_html_string, "").into_owned();
            }
            
            // 3. Process code blocks
            let pre_block_regex = Regex::new(r#"(?s)<pre\s*(?:class="[^"]*language-([^"\s]+)[^"]*")?[^>]*>(.*?)</pre>"#)
                .map_err(|e| eyre::eyre!("Failed to compile pre_block_regex: {}", e))?;
            
            effective_html_string = pre_block_regex.replace_all(&effective_html_string, |caps: &regex::Captures| {
                let lang_attr = caps.get(1).map_or("".to_string(), |m| format!(r#"data-language="{}""#, m.as_str()));
                let original_pre_content = caps.get(2).map_or("", |m| m.as_str());

                let pre_content_doc = Html::parse_fragment(original_pre_content);
                let token_line_selector = Selector::parse(".token-line").expect("Invalid .token-line selector"); // expect is fine for static, known-good selectors
                
                let new_inner_content = pre_content_doc
                    .select(&token_line_selector)
                    .map(|token_line_el| token_line_el.text().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n");

                // Using html_escape::encode_text to prevent issues if code contains '<' or '&'
                format!("<pre {}><code {}>{}</code></pre>", lang_attr, lang_attr, html_escape::encode_text(&new_inner_content))
            }).into_owned();

            // 4. Remove class and style attributes from all elements
            let class_attr_regex = Regex::new(r#"\s*class="[^"]*""#)
                .map_err(|e| eyre::eyre!("Failed to compile class_attr_regex: {}", e))?;
            effective_html_string = class_attr_regex.replace_all(&effective_html_string, "").into_owned();

            let style_attr_regex = Regex::new(r#"\s*style="[^"]*""#)
                .map_err(|e| eyre::eyre!("Failed to compile style_attr_regex: {}", e))?;
            effective_html_string = style_attr_regex.replace_all(&effective_html_string, "").into_owned();

            Ok(effective_html_string)
        })();
        
        // The `?` operator here will convert eyre::Report to crate::core::error::Error
        // due to the From<eyre::Report> for crate::core::error::Error implementation.
        Ok(internal_result?)
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }

    // Keep only one correct definition of as_any
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        // Consistent with HtmlCleanerFilter and default Filter trait if no internal state needs mutation.
        self as &mut dyn Any
    }
}

// The previous comment block at the end of the file which might have caused "unknown start of token"
// has been removed by this overwrite operation. If the error was due to that, it should now be resolved.
// The `ElementRef` import was commented out as it's not directly used in the final string processing logic.
// If DOM-based cleaning (instead of regex on strings) were re-introduced, it might be needed.
