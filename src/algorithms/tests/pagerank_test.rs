//! Integration tests for PageRank algorithm

use std::collections::HashMap;

#[cfg(test)]
mod pagerank_tests {
    use super::*;
    use crawler::algorithms::{LinkGraph, PageRankCalculator};

    /// Test PageRank calculation on a simple graph
    #[test]
    fn test_pagerank_simple_graph() {
        // Create a simple graph: A -> B -> C -> A (cycle)
        let mut graph = LinkGraph::new();

        // Add nodes
        graph.nodes = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ];

        // Add edges: A->B, B->C, C->A
        graph.outbound.insert("A".to_string(), vec!["B".to_string()]);
        graph.outbound.insert("B".to_string(), vec!["C".to_string()]);
        graph.outbound.insert("C".to_string(), vec!["A".to_string()]);

        graph.inbound.insert("A".to_string(), vec!["C".to_string()]);
        graph.inbound.insert("B".to_string(), vec!["A".to_string()]);
        graph.inbound.insert("C".to_string(), vec!["B".to_string()]);

        // Calculate PageRank
        let calculator = PageRankCalculator::new();
        let ranks = calculator.calculate(&graph);

        // Verify results
        assert_eq!(ranks.len(), 3);

        // In a symmetric cycle, all nodes should have equal PageRank
        let rank_a = ranks.get("A").unwrap();
        let rank_b = ranks.get("B").unwrap();
        let rank_c = ranks.get("C").unwrap();

        assert!((rank_a - rank_b).abs() < 0.0001);
        assert!((rank_b - rank_c).abs() < 0.0001);

        // Sum of all ranks should be 1.0
        let sum: f64 = ranks.values().sum();
        assert!((sum - 1.0).abs() < 0.0001);

        println!("✅ Simple cycle test passed: A={:.6}, B={:.6}, C={:.6}", rank_a, rank_b, rank_c);
    }

    /// Test PageRank with a hub node (one node pointed to by many)
    #[test]
    fn test_pagerank_hub_node() {
        let mut graph = LinkGraph::new();

        // Hub node: A, B, C all point to D
        graph.nodes = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
        ];

        // All point to D (hub)
        graph.outbound.insert("A".to_string(), vec!["D".to_string()]);
        graph.outbound.insert("B".to_string(), vec!["D".to_string()]);
        graph.outbound.insert("C".to_string(), vec!["D".to_string()]);
        graph.outbound.insert("D".to_string(), vec![]);

        graph.inbound.insert("D".to_string(), vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ]);
        graph.inbound.insert("A".to_string(), vec![]);
        graph.inbound.insert("B".to_string(), vec![]);
        graph.inbound.insert("C".to_string(), vec![]);

        let calculator = PageRankCalculator::new();
        let ranks = calculator.calculate(&graph);

        let rank_d = ranks.get("D").unwrap();
        let rank_a = ranks.get("A").unwrap();

        // Hub node D should have highest PageRank
        assert!(rank_d > rank_a);
        assert!(rank_d > &0.5); // Should be significantly higher

        println!("✅ Hub node test passed: D={:.6} (hub), A={:.6}", rank_d, rank_a);
    }

    /// Test PageRank normalization (sum = 1.0)
    #[test]
    fn test_pagerank_normalization() {
        let mut graph = LinkGraph::new();

        // Create a more complex graph
        graph.nodes = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
        ];

        // Random edges
        graph.outbound.insert("1".to_string(), vec!["2".to_string(), "3".to_string()]);
        graph.outbound.insert("2".to_string(), vec!["3".to_string(), "4".to_string()]);
        graph.outbound.insert("3".to_string(), vec!["1".to_string()]);
        graph.outbound.insert("4".to_string(), vec!["5".to_string()]);
        graph.outbound.insert("5".to_string(), vec!["1".to_string()]);

        // Build inbound
        for (source, targets) in &graph.outbound {
            for target in targets {
                graph.inbound.entry(target.clone())
                    .or_insert_with(Vec::new)
                    .push(source.clone());
            }
        }

        let calculator = PageRankCalculator::new();
        let ranks = calculator.calculate(&graph);

        // Verify normalization: sum should be 1.0
        let sum: f64 = ranks.values().sum();
        assert!((sum - 1.0).abs() < 0.0001, "PageRank sum must be 1.0, got {}", sum);

        // Verify all ranks are positive
        for (node, rank) in &ranks {
            assert!(*rank > 0.0, "Node {} has non-positive rank: {}", node, rank);
        }

        println!("✅ Normalization test passed: sum={:.6}", sum);
    }

    /// Test PageRank convergence
    #[test]
    fn test_pagerank_convergence() {
        let mut graph = LinkGraph::new();

        // Simple two-node graph
        graph.nodes = vec!["A".to_string(), "B".to_string()];
        graph.outbound.insert("A".to_string(), vec!["B".to_string()]);
        graph.outbound.insert("B".to_string(), vec!["A".to_string()]);
        graph.inbound.insert("A".to_string(), vec!["B".to_string()]);
        graph.inbound.insert("B".to_string(), vec!["A".to_string()]);

        let calculator = PageRankCalculator::new();
        let ranks = calculator.calculate(&graph);

        // Should converge (no panic, returns results)
        assert_eq!(ranks.len(), 2);

        println!("✅ Convergence test passed");
    }

    /// Test graph with no edges (all dangling nodes)
    #[test]
    fn test_pagerank_no_edges() {
        let mut graph = LinkGraph::new();

        graph.nodes = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        // No edges - all dangling

        let calculator = PageRankCalculator::new();
        let ranks = calculator.calculate(&graph);

        // All nodes should have equal rank (1/N)
        let expected_rank = 1.0 / 3.0;
        for (_node, rank) in &ranks {
            assert!((rank - expected_rank).abs() < 0.0001);
        }

        println!("✅ No edges test passed: all ranks = {:.6}", expected_rank);
    }

    /// Test get_top_pages method
    #[test]
    fn test_get_top_pages() {
        let mut ranks = HashMap::new();
        ranks.insert("Page1".to_string(), 0.5);
        ranks.insert("Page2".to_string(), 0.3);
        ranks.insert("Page3".to_string(), 0.15);
        ranks.insert("Page4".to_string(), 0.05);

        let calculator = PageRankCalculator::new();
        let top_pages = calculator.get_top_pages(&ranks, 2);

        assert_eq!(top_pages.len(), 2);
        assert_eq!(top_pages[0].0, "Page1");
        assert_eq!(top_pages[1].0, "Page2");
        assert_eq!(top_pages[0].1, 0.5);
        assert_eq!(top_pages[1].1, 0.3);

        println!("✅ Top pages test passed");
    }

    /// Test empty graph
    #[test]
    fn test_pagerank_empty_graph() {
        let graph = LinkGraph::new();

        let calculator = PageRankCalculator::new();
        let ranks = calculator.calculate(&graph);

        assert_eq!(ranks.len(), 0);

        println!("✅ Empty graph test passed");
    }
}
