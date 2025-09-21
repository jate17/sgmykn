use reqwest;
use scraper::{Html, Selector};
use std::collections::HashMap;
use crate::crawler::utility;

pub async fn crawl_page(url_c: &str, wheres: &mut HashMap<i32, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> { // <-- nome giusto + parametro
    let resp = reqwest::get(url_c).await?;
    let base_url = resp.url().clone();
    let body = resp.text().await?;

    let seed_host = base_url.host_str().ok_or("URL senza host")?;
    let allowed_base_domain = utility::extract_base_domain(seed_host)
        .unwrap_or_else(|| seed_host.to_string());

    let doc = Html::parse_document(&body);
    let sel_a = Selector::parse("a").unwrap();

    for a in doc.select(&sel_a) {
        if let Some(href) = a.value().attr("href") {
            if href.starts_with('#') || href.starts_with("mailto:")
                || href.starts_with("tel:") || href.starts_with("javascript:") {
                continue;
            }
            if let Ok(abs) = base_url.join(href) {
                let cls = utility::check_url(abs.as_str(), &allowed_base_domain)?;
                match cls {
                    1 => utility::append_to_crawl(abs.to_string(), 0, wheres),
                    2 => utility::append_to_crawl(abs.to_string(), 1, wheres),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
