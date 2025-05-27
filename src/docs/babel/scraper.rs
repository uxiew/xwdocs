use async_trait::async_trait;
use eyre::Result;
use regex::RegexSet;

use crate::core::doc::Doc;
use crate::core::scraper::filter::Filter; // Added for filters()
use crate::docs::babel::clean::BabelCleanHtmlFilter; // Added for filters()
use crate::docs::babel::entries::BabelEntriesFilter; // Added for filters()
// Assuming UrlScraper is the base scraper used elsewhere, similar to CssScraper
// If not, these configurations would need to be handled directly in BabelScraper or via the Scraper trait.
use crate::core::scraper::{Scraper, UrlScraper}; // Assuming UrlScraper exists and is relevant
use crate::core::types::StackItem;

#[derive(Debug, Clone)]
pub struct BabelScraper {
    // Assuming UrlScraper handles common scraper functionalities like storing base_url, options, etc.
    // If UrlScraper is not the right base, these fields would be part of BabelScraper directly.
    scraper: UrlScraper,
    skip_patterns: RegexSet,
}

impl BabelScraper {
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://babeljs.io/docs/";
        let mut scraper = UrlScraper::new("Babel", version, base_url, output_path);

        // 1. Set base_url - handled by UrlScraper::new

        // 2. Set trailing_slash
        scraper.options_mut().set_trailing_slash(true); // Assuming this method exists

        // 3. Define skip_patterns
        let skip_patterns = RegexSet::new(&[
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
        ])
        .unwrap();

        // 4. Set skip_link logic - This will be implemented in should_scrape_url or similar
        // For now, we store the patterns. The actual logic will be in a method.

        // 5. Set attribution
        let attribution = r#"
&copy; 2014-present Sebastian McKenzie<br>
Licensed under the MIT License.
        "#
        .to_string();
        scraper.options_mut().set_attribution(attribution); // Assuming this method exists

        // Example: Add initial paths if necessary (based on CssScraper)
        // scraper = scraper.with_initial_paths(vec!["/".to_string()]);

        Self {
            scraper,
            skip_patterns,
        }
    }

    // Helper method for skip_link logic, assuming it's called by a method from Scraper trait
    // or within the scrape method.
    fn should_skip_url(&self, url_str: &str) -> bool {
        if url_str.starts_with("https://babeljs.io/docs/en/") {
            return true; // Skip this URL
        }
        // Check against skip_patterns
        if self.skip_patterns.is_match(url_str) {
            return true; // Skip this URL
        }
        false // Do not skip
    }
}

#[async_trait]
impl Scraper for BabelScraper {
    // Assuming Config and Item types are handled by UrlScraper or are generic
    type Config = <UrlScraper as Scraper>::Config;
    type Item = Doc; // Or <UrlScraper as Scraper>::Item if Doc is generic to UrlScraper

    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn config(&self) -> &Self::Config {
        self.scraper.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.scraper.config_mut()
    }

    fn stack(&self) -> &Vec<StackItem> {
        self.scraper.stack()
    }

    fn stack_mut(&mut self) -> &mut Vec<StackItem> {
        self.scraper.stack_mut()
    }

    async fn scrape(&mut self) -> Result<Vec<Self::Item>> {
        // The actual scraping logic will need to use self.should_skip_url(url)
        // For example, when iterating through links:
        // let links = find_all_links_on_page(current_page_content);
        // for link in links {
        //     if !self.should_skip_url(&link) {
        //         self.stack_mut().push(StackItem::Url(link));
        //     }
        // }
        // This is a placeholder for the actual scraping logic.
        // It would likely call self.scraper.scrape() or a similar method if UrlScraper handles the core loop.
        // Or, if BabelScraper implements its own loop, it would fetch pages, parse them,
        // extract links, filter them using should_skip_url, and extract content.
        unimplemented!("Actual scraping logic needs to be implemented, using should_skip_url and skip_patterns.")
    }

    fn filters(&self) -> Result<Vec<Box<dyn Filter>>> {
        Ok(vec![
            Box::new(BabelCleanHtmlFilter::default()),
            Box::new(BabelEntriesFilter::default()),
        ])
    }

    fn get_latest_version(&self, _opts: Option<crate::core::config::Options>) -> Result<String> {
        // Call the utility function to get the latest release from GitHub.
        // Assuming the path to the utility function is crate::core::utils::github::get_latest_github_release
        // and it does not require the `opts` parameter for this specific functionality.
        crate::core::utils::github::get_latest_github_release("babel", "babel")
    }

    // Potentially, a method like this would be part of the Scraper trait
    // or called by the scraping loop.
    // fn should_scrape_url(&self, url: &str) -> bool {
    //     !self.should_skip_url(url)
    // }
}
