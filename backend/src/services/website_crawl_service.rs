use std::time::Duration;

use serde::Deserialize;
use spider::{hashbrown::HashSet, page::Page, website::Website, CaseInsensitiveString};
use sqlx::{Pool, Transaction};
use tokio::task;
use url::Url;

use crate::{
    db::{
        domain_link_repository, domain_repository, new_transaction,
        website_page_repository::{self},
        DB,
    },
    models::domain::Domain,
    util::search_error::SearchResult,
};

pub async fn crawl_websites(db_pool: &Pool<DB>) -> SearchResult<()> {
    let mut transaction = new_transaction(db_pool).await?;

    insert_initial_websites(&mut transaction).await?;

    transaction.commit().await?;

    crawl(db_pool).await?;

    Ok(())
}

async fn insert_initial_websites(transaction: &mut Transaction<'_, DB>) -> SearchResult<()> {
    let top_websites = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("./datasets/test.csv")
        .expect("CSV open not worko, mucho sado")
        .deserialize()
        .map(|v| {
            let w: TopWebsite = v.expect("Failed to parse website");
            w.domain
        })
        .collect::<Vec<String>>();

    println!("Parsed {} websites", top_websites.len());

    for site in top_websites.into_iter() {
        if domain_repository::find_by_domain(transaction, site.clone())
            .await?
            .is_some()
        {
            continue;
        }

        println!("Inserting domain to TODO {site}");
        domain_repository::insert_domain_to_index(transaction, site).await?;
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

async fn crawl(db_pool: &Pool<DB>) -> SearchResult<()> {
    let mut transaction = new_transaction(db_pool).await?;

    let to_index = domain_repository::find_non_indexed(&mut transaction).await?;

    transaction.commit().await?;

    for domain in to_index.into_iter() {
        let db_clone = db_pool.clone();
        task::spawn(crawl_domain(db_clone, domain.clone()));
    }

    Ok(())
}

async fn crawl_domain(db_pool: Pool<DB>, domain: Domain) {
    match crawl_next(&db_pool, &domain).await {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to crawl domain {}, got err {e}", domain.domain);
        }
    }
}

async fn crawl_next(db_pool: &Pool<DB>, to_index: &Domain) -> SearchResult<()> {
    let url = parse_url(&to_index.domain)?;
    println!("Processing {url}");

    let mut website: Website = Website::new(url.as_str());
    website
        .with_respect_robots_txt(true)
        .with_subdomains(false)
        .with_tld(true)
        .with_user_agent(Some("search-rs/0.1.0".into()))
        .with_delay(250)
        .with_request_timeout(Some(Duration::from_secs(15)));

    println!("\tBegin crawl...");
    website.crawl().await;
    println!("\tBegin scrape...");
    website.scrape().await;

    let mut transaction = new_transaction(db_pool).await?;

    let domain = domain_repository::set_domain_indexed(&mut transaction, &to_index).await?;

    if let Some(pages) = website.get_pages() {
        handle_website_pages(&mut transaction, pages, &url, &domain).await?;
    } else {
        println!("\tNo pages found :(")
    }

    let links = website.get_links();
    handle_website_link(&mut transaction, links, &url, &domain).await?;

    transaction.commit().await?;

    Ok(())
}

async fn handle_website_pages(
    transaction: &mut Transaction<'_, DB>,
    pages: &Box<Vec<Page>>,
    url: &Url,
    domain: &Domain,
) -> SearchResult<()> {
    println!("\tFound {} pages for url", pages.len());
    for page in pages.iter() {
        println!("\t\tFOUND PAGE {} for website {url}", page.get_url());

        let page_url = Url::parse(page.get_url())?;

        if website_page_repository::find_website_page_by_url(transaction, page_url.to_string())
            .await?
            .is_some()
        {
            // The URL already exists
            continue;
        }

        let title = extract_page_title(&page);

        website_page_repository::insert_website_page(
            transaction,
            &domain,
            title,
            page_url.to_string(),
        )
        .await?;
    }

    Ok(())
}

async fn handle_website_link(
    transaction: &mut Transaction<'_, DB>,
    links: &HashSet<CaseInsensitiveString>,
    url: &Url,
    domain: &Domain,
) -> SearchResult<()> {
    println!("\t    Found {} links", links.len());
    for link in links {
        let link_url = parse_url(&link.inner().to_string())?;
        println!(
            "\t\t Processing url {link_url} for domain {}",
            domain.domain
        );
        if link_url == *url {
            // Skipping existing url.
            return Ok(());
        }

        let child_domain = link_url
            .domain()
            .expect("Failed to get domain from URL")
            .to_string();

        let d = match domain_repository::find_by_domain(transaction, child_domain.clone()).await? {
            // A domain already exists matching this, ensure to insert a link between it and the parent
            Some(d) => d,
            // No domain exists, add it and connect it with the parent
            None => {
                domain_repository::insert_domain_to_index(transaction, child_domain.clone()).await?
            }
        };

        if d == *domain {
            // Don't insert links to ourselves.
            return Ok(());
        }
        domain_link_repository::insert_domain_link(transaction, &domain, &d).await?;
    }
    Ok(())
}

fn extract_page_title(page: &Page) -> Option<String> {
    let html = page.get_html();
    let (_, rest) = html.split_once("<head>")?;
    let (head, _) = rest.split_once("</head>")?;
    let (_, title_rest) = head.split_once("<title>")?;
    let (title, _) = title_rest.split_once("</title>")?;

    Some(title.to_string())
}

fn parse_url(base: &str) -> SearchResult<Url> {
    match Url::parse(base) {
        Ok(u) => return Ok(u),
        Err(_) => Ok(Url::parse(&format!("https://{base}"))?),
    }
}
