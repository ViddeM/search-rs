#![forbid(unsafe_code)]

use config::Config;
use serde::Deserialize;
use spider::website::Website;
use std::time::Duration;
#[macro_use]
extern crate rocket;

mod config;

#[launch]
async fn rocket() -> _ {
    let config = Config::new().expect("Failed to load config");

    scrape().await;

    rocket::build()
}

#[derive(Debug, Deserialize)]
struct TopWebsite {
    #[serde(alias = "Rank")]
    rank: String,
    #[serde(alias = "Domain")]
    domain: String,
    #[serde(alias = "Open Page Rank")]
    open_page_rank: String,
}

async fn scrape() {
    let mut top_websites = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./datasets/top_thousand_domains.csv")
        .expect("CSV open not worko, mucho sado")
        .deserialize()
        .map(|v| {
            let w: TopWebsite = v.expect("Failed to parse website");
            format!("https://{}", w.domain)
        })
        .collect::<Vec<String>>();
    top_websites.reverse();
    println!("Parsed {} websites", top_websites.len());
    let mut handled: Vec<String> = Vec::new();

    while top_websites.is_empty() == false {
        let domain = top_websites.pop().unwrap();
        println!("Processing {domain}  --- {} remaining", top_websites.len());

        let mut website: Website = Website::new(&domain);
        website
            .with_respect_robots_txt(true)
            .with_subdomains(true)
            .with_tld(true)
            .with_user_agent(Some("search-rs/0.1.0".into()))
            .with_delay(250)
            .with_request_timeout(Some(Duration::from_secs(15)));

        website.crawl().await;

        handled.push(domain);

        let links = website.get_links();

        println!("\tFound {} links", links.len());
        for link in links {
            let url = link.inner().to_string();
            if handled.contains(&url) == false && top_websites.contains(&url) == false {
                top_websites.push(url);
            }
        }
    }

    println!("In total, went through {} sites!", handled.len());
}
