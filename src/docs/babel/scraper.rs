use async_trait::async_trait;
// use eyre::Result; // Will be replaced by CoreResult if no other eyre features are used.
                  // If eyre::eyre! or other specific eyre features are used internally, keep it.
                  // For now, assuming only the Result type alias was used for the run signature.
use crate::core::error::Result as CoreResult; // Alias for the project's Result type
use regex::RegexSet;
use std::sync::Arc; 

// Corrected imports based on analysis
use crate::core::scraper::base::Scraper; 
use crate::core::scraper::filter::Filter; // This is unused, will be caught by compiler if so.
use crate::core::scraper::url_scraper::UrlScraper;
use crate::docs::babel::clean::BabelCleanHtmlFilter;
use crate::docs::babel::entries::BabelEntriesFilter;
// use crate::core::doc::Doc; // Doc is a trait, if Item was an associated type, it would be Box<dyn Doc>
// use crate::core::types::StackItem; // StackItem not found in types.rs
// use crate::core::config::Options; // Options not found in config.rs

// UrlScraper does not derive Debug or Clone, so BabelScraper cannot either if it contains UrlScraper directly.
// #[derive(Debug, Clone)] 
pub struct BabelScraper {
    scraper: UrlScraper,
    // skip_patterns are now captured by the skip_link closure in UrlScraper
}

impl BabelScraper {
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://babeljs.io/docs/";
        
        let skip_patterns_arc = Arc::new(RegexSet::new(&[
            r"/usage/",
            r"/configuration/",
            r"/learn/",
            r"/v7-migration/",
            r"/v7-migration-api/",
            r"/editors/",
            r"/presets/",
            r"/caveats/",
            r"/faq/",
            r"/roadmap/",
        ]).unwrap());

        let skip_link_logic = {
            let patterns = Arc::clone(&skip_patterns_arc);
            move |url_str: &str| -> bool {
                if url_str.starts_with("https://babeljs.io/docs/en/") {
                    return true;
                }
                if patterns.is_match(url_str) {
                    return true;
                }
                false
            }
        };

        let attribution_text = r#"
&copy; 2014-present Sebastian McKenzie<br>
Licensed under the MIT License.
        "#;

        let mut url_scraper = UrlScraper::new("Babel", version, base_url, output_path)
            .with_trailing_slash(true)
            .with_attribution(attribution_text)
            .with_skip_link(skip_link_logic);
            // Filters are added directly to UrlScraper
            // .with_initial_paths(vec!["/".to_string()]); // Example if needed

        url_scraper.filters.push(Box::new(BabelCleanHtmlFilter::default()));
        url_scraper.filters.push(Box::new(BabelEntriesFilter::default()));
        
        Self {
            scraper: url_scraper,
        }
    }

    // get_latest_version is not part of the Scraper trait.
    // It should be an inherent method if needed.
    // For now, commenting out due to unresolved dependency on crate::core::utils::github
    // and crate::core::config::Options
    /*
    pub fn get_latest_version(&self, _opts: Option<Options>) -> Result<String> {
        // Correct path to github utility would be needed.
        // crate::core::utils::github::get_latest_github_release("babel", "babel")
        unimplemented!("get_latest_version needs core::utils::github and potentially Options type fixed")
    }
    */
}

#[async_trait]
impl Scraper for BabelScraper {
    // Scraper trait does not have Config or Item associated types.
    // type Config = <UrlScraper as Scraper>::Config; // Incorrect
    // type Item = Box<dyn Doc>; // Incorrect as Item is not an associated type of Scraper

    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        // BabelScraper's `new` takes `version` which is passed to UrlScraper.
        self.scraper.version()
    }

    async fn run(&mut self) -> CoreResult<()> { // Changed return type to CoreResult
        // Delegate to the wrapped UrlScraper's run method.
        // self.scraper.run().await already returns crate::core::error::Result<()>
        self.scraper.run().await
    }

    // Methods like config, config_mut, stack, stack_mut, scrape, filters
    // are not part of the core Scraper trait.
    // They were likely assumed from a different trait definition or were specific to a BaseScraper concept
    // that UrlScraper itself embodies but doesn't expose via these specific methods through the main Scraper trait.
    // UrlScraper's own `run` method handles the scraping loop, fetching, filtering, etc.
}
