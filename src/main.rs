mod crawler;

use crate::crawler::crawl::crawl_page;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    crawl_page("https://lpg.unibs.it/FI-PPING/").await
}

