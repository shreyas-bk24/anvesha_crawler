use clap::Parser;
use crawler::{init, CrawlerConfig, WebCrawler};
use tracing::info;

#[derive(Parser)]
#[command(name = "search-crawler")]
#[command(about = "A modular web crawler for search engines")]
struct Args {
    #[arg(short, long, default_value = "config/default.toml")]
    config: String,

    #[arg(long)]
    dry_run: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Crawl {
        #[arg(short, long)]
        seed_urls: Vec<String>,
        #[arg(long)]
        save_to_db: bool,
        #[arg(long, default_value = "10")]
        max_pages: usize,
    },
    Api {
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },
    Stats,
}

#[tokio::main]
async fn main() -> crawler::Result<()> { // Fixed: search_engine_crawler -> crawler
    let args = Args::parse();

    // Initialize logging and metrics
    init().await?;

    // Load configuration
    let config = CrawlerConfig::from_file(&args.config)?;
    info!("Loaded configuration from: {}", args.config);

    match args.command {
        Some(Commands::Crawl { seed_urls, .. }) => {
            let mut crawler_config = config;
            if !seed_urls.is_empty() {
                crawler_config.crawler.seed_urls = seed_urls; // Fixed: Added .crawler field
            }

            let crawler = WebCrawler::new(crawler_config).await?;
            crawler.start_crawling().await?;
        }
        Some(Commands::Api { port }) => {
            // Remove this for now since API module doesn't exist yet
            println!("API server not implemented yet. Port: {}", port);
            // let api_server = crawler::api::create_server(config, port).await?;
            // api_server.serve().await?;
        }
        Some(Commands::Stats) => {
            // Show crawler statistics
            println!("Crawler Statistics:");
            // Implementation here
        }
        None => {
            // Default: start crawling
            let crawler = WebCrawler::new(config).await?;
            crawler.start_crawling().await?;
        }
    }

    Ok(())
}
