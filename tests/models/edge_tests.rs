

#[cfg(test)]
mod edge_tests {
    use cutlist_optimizer_cli::models::edge::Edge;


    #[test]
    fn test_edge_default() {
        let edge = Edge::default();
        assert_eq!(edge.top, None);
        assert_eq!(edge.left, None);
        assert_eq!(edge.bottom, None);
        assert_eq!(edge.right, None);
    }

    #[test]
    fn test_edge_new() {
        let edge = Edge::new();
        assert_eq!(edge, Edge::default());
    }

    #[test]
    fn test_edge_uniform() {
        let edge = Edge::uniform("2mm".to_string());
        assert_eq!(edge.top, Some("2mm".to_string()));
        assert_eq!(edge.left, Some("2mm".to_string()));
        assert_eq!(edge.bottom, Some("2mm".to_string()));
        assert_eq!(edge.right, Some("2mm".to_string()));
    }

    #[test]
    fn test_edge_uniform_empty_string() {
        let edge = Edge::uniform("".to_string());
        assert_eq!(edge.top, Some("".to_string()));
        assert_eq!(edge.left, Some("".to_string()));
        assert_eq!(edge.bottom, Some("".to_string()));
        assert_eq!(edge.right, Some("".to_string()));
    }

    #[test]
    fn test_edge_has_any_edge_empty() {
        let empty_edge = Edge::new();
        assert!(!empty_edge.has_any_edge());
    }

    #[test]
    fn test_edge_has_any_edge_with_top() {
        let edge_with_top = Edge {
            top: Some("1mm".to_string()),
            ..Edge::default()
        };
        assert!(edge_with_top.has_any_edge());
    }

    #[test]
    fn test_edge_has_any_edge_with_left() {
        let edge_with_left = Edge {
            left: Some("1mm".to_string()),
            ..Edge::default()
        };
        assert!(edge_with_left.has_any_edge());
    }

    #[test]
    fn test_edge_has_any_edge_with_bottom() {
        let edge_with_bottom = Edge {
            bottom: Some("1mm".to_string()),
            ..Edge::default()
        };
        assert!(edge_with_bottom.has_any_edge());
    }

    #[test]
    fn test_edge_has_any_edge_with_right() {
        let edge_with_right = Edge {
            right: Some("1mm".to_string()),
            ..Edge::default()
        };
        assert!(edge_with_right.has_any_edge());
    }

    #[test]
    fn test_edge_has_any_edge_with_multiple() {
        let edge_with_multiple = Edge {
            top: Some("1mm".to_string()),
            bottom: Some("2mm".to_string()),
            ..Edge::default()
        };
        assert!(edge_with_multiple.has_any_edge());
    }

    #[test]
    fn test_edge_has_all_edges_empty() {
        let empty_edge = Edge::new();
        assert!(!empty_edge.has_all_edges());
    }

    #[test]
    fn test_edge_has_all_edges_partial() {
        let partial_edge = Edge {
            top: Some("1mm".to_string()),
            left: Some("1mm".to_string()),
            ..Edge::default()
        };
        assert!(!partial_edge.has_all_edges());
    }

    #[test]
    fn test_edge_has_all_edges_complete() {
        let complete_edge = Edge::uniform("1mm".to_string());
        assert!(complete_edge.has_all_edges());
    }

    #[test]
    fn test_edge_has_all_edges_different_values() {
        let edge_different_values = Edge {
            top: Some("1mm".to_string()),
            left: Some("2mm".to_string()),
            bottom: Some("3mm".to_string()),
            right: Some("4mm".to_string()),
        };
        assert!(edge_different_values.has_all_edges());
    }

    #[test]
    fn test_edge_clone() {
        let original = Edge::uniform("5mm".to_string());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_edge_debug() {
        let edge = Edge::uniform("1mm".to_string());
        let debug_str = format!("{:?}", edge);
        assert!(debug_str.contains("Edge"));
        assert!(debug_str.contains("1mm"));
    }

    #[test]
    fn test_edge_partial_eq() {
        let edge1 = Edge::uniform("1mm".to_string());
        let edge2 = Edge::uniform("1mm".to_string());
        let edge3 = Edge::uniform("2mm".to_string());
        
        assert_eq!(edge1, edge2);
        assert_ne!(edge1, edge3);
    }

    #[test]
    fn test_edge_serialization() {
        let edge = Edge::uniform("1.5mm".to_string());
        
        // Test serialization
        let serialized = serde_json::to_string(&edge).expect("Failed to serialize");
        assert!(serialized.contains("1.5mm"));
        
        // Test deserialization
        let deserialized: Edge = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(edge, deserialized);
    }

    #[test]
    fn test_edge_empty_serialization() {
        let edge = Edge::default();
        
        // Test serialization
        let serialized = serde_json::to_string(&edge).expect("Failed to serialize");
        
        // Test deserialization
        let deserialized: Edge = serde_json::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(edge, deserialized);
    }
}
