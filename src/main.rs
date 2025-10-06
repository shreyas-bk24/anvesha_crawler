use clap::Parser;
use crawler::{init, CrawlerConfig, WebCrawler};
use tracing::info;
use crawler::storage::database::{Database, DatabaseConfig};
use crawler::storage::repository::PageRepository;

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
async fn main() -> crawler::Result<()> {
    let args = Args::parse();

    // Initialize logging and metrics
    init().await?;

    // Load configuration
    let config = CrawlerConfig::from_file(&args.config)?;
    info!("Loaded configuration from: {}", args.config);

    match args.command {
        Some(Commands::Crawl { seed_urls, save_to_db, max_pages }) => {
            let mut crawler_config = config;

            // Update seed URLs if provided
            if !seed_urls.is_empty() {
                crawler_config.crawler.seed_urls = seed_urls;
            }

            // Update max pages if provided
            crawler_config.crawler.max_pages = max_pages;

            // 🔥 SIMPLE: Initialize database if save_to_db is true
            let repository = if save_to_db {
                info!("🗄️ Database storage enabled - initializing PostgreSQL database");

                let db_config = DatabaseConfig {
                    database_url: crawler_config.storage.database_url.clone(),
                    max_connections: crawler_config.storage.max_connections,
                    enable_wal_mode: false,
                    enable_foreign_keys: true,
                };

                // Connect and migrate database
                let pool = Database::connect(&db_config).await?;
                Database::migrate(&pool).await?;
                info!("✅ Database initialized and migrations completed");

                Some(PageRepository::new(pool))
            } else {
                info!("📝 Running crawler without database storage");
                None
            };

            // 🔥 SIMPLE: Just create crawler normally
            let crawler = WebCrawler::new(crawler_config).await?;

            // 🔥 SIMPLE: Pass repository to the crawl method
            crawler.start_crawling_with_repository(repository).await?;
        }
        Some(Commands::Api { port }) => {
            println!("API server not implemented yet. Port: {}", port);
        }
        Some(Commands::Stats) => {
            println!("Crawler Statistics:");
        }
        None => {
            let crawler = WebCrawler::new(config).await?;
            crawler.start_crawling().await?;
        }
    }

    Ok(())
}
