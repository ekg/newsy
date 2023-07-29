use anyhow::Result;
use std::io::Write;
use clap::Parser;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Simple command line app to download news articles

#[derive(Parser)]
#[command(name = "News getter")]
#[command(author = "Erik Garrison <erik.garrison@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Collect articles from newsapi.org, saving as text", long_about = None)]
struct Cli {
    /// API key for NewsAPI.org
    #[clap(short, long)]
    api_key: String,

    /// Search string
    #[clap(short, long, default_value = "")]
    search: String,

    /// Number of articles to download
    #[clap(short, long, default_value="100")]
    limit: usize,

    /// Output directory
    #[clap(short, long, default_value = "./")]
    output_dir: PathBuf,

    /// Optionally list results
    #[clap(long)]
    list_results: bool,
}

/// Article type metadata from API
#[derive(Deserialize)]
struct Article {
    title: String,
    url: String,
}

/// Article metadata from API
#[derive(Deserialize)]
struct ApiResponse {
    status: String,
    totalResults: usize,
    articles: Vec<Article>,
}

/// Makes request to NewsAPI and returns articles
async fn get_top_headline_articles(api_key: &str, limit: usize) -> Result<Vec<Article>> {
    let client = reqwest::Client::new();
    let resp = client.get("https://newsapi.org/v2/top-headlines")
        .header(USER_AGENT, USER_AGENT_STR)
        .query(&[("country", "us"), ("pageSize", &limit.to_string()), ("apiKey", api_key)])
        .send()
        .await?;
    let api_resp = resp.json::<ApiResponse>().await?;
    Ok(api_resp.articles)
}

/// static user agent variable
static USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0";

/// Requests articles based on search string
async fn get_articles(api_key: &str, limit: usize, search: &str) -> Result<Vec<Article>> {
    let client = reqwest::Client::new();
    let resp = client.get("https://newsapi.org/v2/everything")
        .header(USER_AGENT, USER_AGENT_STR)
        .query(&[("q", search), ("pageSize", &limit.to_string()), ("apiKey", api_key)])
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(resp.articles)   
}

/// Downloads and saves article text 
async fn save_article(article: &Article, out_dir: &Path) -> Result<()> {

    eprintln!("Downloading {}", article.url);

    let client = reqwest::Client::new();
    let body = client.get(&article.url)
        .header(USER_AGENT, USER_AGENT_STR)
        .send()
        .await?
        .text()
        .await?;

    // Parse HTML
    let parsed = Html::parse_document(&body);

    let paragraphs = parsed
        .select(&Selector::parse("p").unwrap())
        .map(|p| p.text().collect::<String>()) 
        .collect::<Vec<_>>()
        .join("\n");

    //eprintln!("{}", paragraphs);

    let filename = format!("{}.txt", article.title.replace(' ', "_"));
    let filepath = out_dir.join(&filename);
    let mut file = File::create(filepath)?;
    file.write_all(paragraphs.as_bytes())?;

    eprintln!("Saved {}", filename);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.output_dir.exists() {
        std::fs::create_dir_all(&args.output_dir)?;
    }

    if args.search.is_empty() {
        let articles = get_top_headline_articles(&args.api_key, args.limit).await?;
        if !args.list_results {
            for article in articles {
                save_article(&article, &args.output_dir).await?;
            }
        } else {
            for article in articles {
                println!("{}\t{}", article.title, article.url);
            }
        }
    } else {
        let articles = get_articles(&args.api_key, args.limit, &args.search).await?;
        if !args.list_results {
            for article in articles {
                save_article(&article, &args.output_dir).await?;
            }
        } else {
            for article in articles {
                println!("{}\t{}", article.title, article.url);
            }
        }
    };

    Ok(())
}
