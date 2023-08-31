use std::time::Duration;

use serde::Deserialize;
use spider::{page::Page, website::Website};
use sqlx::{Pool, Transaction};

use crate::{
    db::{
        indexed_website_repository::insert_indexed_website, new_transaction,
        website_to_index_repository, DB,
    },
    util::search_error::SearchResult,
};

pub async fn crawl_websites(db_pool: &Pool<DB>) -> SearchResult<()> {
    let mut transaction = new_transaction(db_pool).await?;

    insert_initial_websites(&mut transaction).await?;

    crawl(&mut transaction).await?;

    transaction.commit().await?;

    Ok(())
}

async fn insert_initial_websites(transaction: &mut Transaction<'_, DB>) -> SearchResult<()> {
    let top_websites = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./datasets/top_thousand_domains.csv")
        .expect("CSV open not worko, mucho sado")
        .deserialize()
        .map(|v| {
            let w: TopWebsite = v.expect("Failed to parse website");
            w.domain
        })
        .collect::<Vec<String>>();

    println!("Parsed {} websites", top_websites.len());

    for site in top_websites.into_iter() {
        website_to_index_repository::insert_website_to_index(transaction, site).await?;
    }

    Ok(())
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

async fn crawl(transaction: &mut Transaction<'_, DB>) -> SearchResult<()> {
    loop {
        let to_index = match website_to_index_repository::get_oldest(transaction).await? {
            Some(w) => w,
            None => return Ok(()),
        };

        let url = format!("https://{}", to_index.domain);
        println!("Processing {url}");

        let mut website: Website = Website::new(&url);
        website
            .with_respect_robots_txt(true)
            .with_subdomains(true)
            .with_tld(true)
            .with_user_agent(Some("search-rs/0.1.0".into()))
            .with_delay(250)
            .with_request_timeout(Some(Duration::from_secs(15)));

        website.crawl().await;
        website.scrape().await;

        for page in website
            .get_pages()
            .expect("Failed to retrieve pages after scrape")
            .into_iter()
        {
            let url = strip_protocol(page.get_url());
            let title = extract_page_title(&page);

            let indexed_website = insert_indexed_website(transaction, url, title).await?;
        }

        let links = website.get_links();

        println!("\tFound {} links", links.len());
        for link in links {
            let url = link.inner().to_string();
            if handled.contains(&url) == false && top_websites.contains(&url) == false {
                top_websites.push(url);
            }
        }
    }
}

fn strip_protocol(url: &str) -> String {
    let without_protocol = if url.starts_with("https://") {
        url.strip_prefix("https://").unwrap()
    } else if url.starts_with("http") {
        url.strip_prefix("http://").unwrap()
    } else {
        println!("Unknown url prefix {url}");
        url
    };

    if without_protocol.starts_with("www.") {
        without_protocol.strip_prefix("www.").unwrap()
    } else {
        without_protocol
    }
    .to_string()
}

fn extract_page_title(page: &Page) -> Option<String> {
    let html = page.get_html();
    let (_, rest) = html.split_once("<head>")?;
    let (head, _) = rest.split_once("</head>")?;
    let (_, title_rest) = head.split_once("<title>")?;
    let (title, _) = title_rest.split_once("</title>")?;

    Some(title.to_string())
}
