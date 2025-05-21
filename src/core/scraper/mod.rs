//! 抓取器核心模块
//! 
//! 提供文档抓取的基础功能

pub mod base;
pub mod filter;
pub mod url_scraper;
pub mod fix_redirections;
pub mod rate_limiter;

pub use base::{Scraper, ScraperConfig, BaseScraper};
pub use filter::{Filter, FilterContext};
pub use url_scraper::UrlScraper;
pub use fix_redirections::{FixRedirections, Redirections};
pub use rate_limiter::RateLimiter;
