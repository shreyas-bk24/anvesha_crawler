//! Integration test with database

#[cfg(test)]
mod integration_tests {
    use crate::algorithms::{LinkGraph, PageRankCalculator};
    use crate::storage::database::{Database, DatabaseConfig};
    use crate::storage::repository::PageRepository;
    
    #[tokio::test]
    async fn test_pagerank_with_database() {
        // Connect to test database
        let db_config = DatabaseConfig {
            database_url: "postgresql://crawler_user:crawler_pass@localhost:5432/crawler_db".to_string(),
            max_connections: 5,
            enable_wal_mode: false,
            enable_foreign_keys: true,
        };

        let pool = Database::connect(&db_config).await.unwrap();
        let repository = PageRepository::new(pool);

        // Build graph from database
        let graph = LinkGraph::from_repository(&repository).await.unwrap();

        println!("📊 Graph loaded: {} nodes, {} edges",
                 graph.node_count(),
                 graph.outbounds.values().map(|v| v.len()).sum::<usize>());

        // Calculate PageRank
        let calculator = PageRankCalculator::new();
        let ranks = calculator.calculate(&graph);

        // Verify
        assert!(ranks.len() > 0);

        let sum: f64 = ranks.values().sum();
        assert!((sum - 1.0).abs() < 0.001);

        // Get top 5
        let top_5 = calculator.get_top_pages(&ranks, 5);
        println!("\n🏆 Top 5 Pages:");
        for (i, (url, rank)) in top_5.iter().enumerate() {
            println!("  {}. {:.6} - {}", i+1, rank, url);
        }

        println!("\n✅ Database integration test passed");
    }
}
