//! Test program to run the Babel scraper

use xwdoc::core::scraper::base::Scraper;
use xwdoc::docs::babel::BabelScraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating Babel scraper...");
    // Create output path for the documentation
    let output_path = "./docs/babel12";
    let mut scraper = BabelScraper::new(output_path, "7");

    println!("Running Babel scraper...");
    scraper.run().await?;

    println!("Babel scraper completed successfully!");
    Ok(())
}
