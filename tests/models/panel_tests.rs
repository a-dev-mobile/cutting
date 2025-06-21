

#[cfg(test)]
mod panel_tests {
    use cutlist_optimizer_cli::models::panel::Panel;



    #[test]
    fn test_panel_default() {
        let panel = Panel::default();
        assert_eq!(panel.id, 0);
        assert_eq!(panel.width, None);
        assert_eq!(panel.height, None);
        assert_eq!(panel.count, 0);
        assert_eq!(panel.material, "DEFAULT");
        assert!(!panel.enabled);
        assert_eq!(panel.orientation, 0);
        assert_eq!(panel.label, None);
        assert_eq!(panel.edge, None);
    }

    #[test]
    fn test_panel_new() {
        let panel = Panel::new();
        assert_eq!(panel, Panel::default());
    }

    #[test]
    fn test_panel_with_id() {
        let panel = Panel::new().with_id(42);
        assert_eq!(panel.id, 42);
        assert_eq!(panel.material, "DEFAULT");
    }

    #[test]
    fn test_panel_builder_pattern() {
        let panel = Panel::new()
            .with_id(1)
            .with_width("100.5".to_string())
            .with_height("200.0".to_string())
            .with_count(5)
            .with_material("Wood".to_string())
            .with_enabled(true)
            .with_orientation(90)
            .with_label("Test Panel".to_string());

        assert_eq!(panel.id, 1);
        assert_eq!(panel.width, Some("100.5".to_string()));
        assert_eq!(panel.height, Some("200.0".to_string()));
        assert_eq!(panel.count, 5);
        assert_eq!(panel.material, "Wood");
        assert!(panel.enabled);
        assert_eq!(panel.orientation, 90);
        assert_eq!(panel.label, Some("Test Panel".to_string()));
    }

    #[test]
    fn test_panel_set_material() {
        let mut panel = Panel::new();
        
        // Test setting material with Some value
        panel.set_material(Some("Oak".to_string()));
        assert_eq!(panel.material, "Oak");
        
        // Test setting material with None (should not change)
        panel.set_material(None);
        assert_eq!(panel.material, "Oak");
    }

    #[test]
    fn test_panel_is_valid_disabled() {
        let panel = Panel::new()
            .with_enabled(false)
            .with_count(5)
            .with_width("100.0".to_string())
            .with_height("200.0".to_string());

        assert_eq!(panel.is_valid().unwrap(), false);
    }

    #[test]
    fn test_panel_is_valid_zero_count() {
        let panel = Panel::new()
            .with_enabled(true)
            .with_count(0)
            .with_width("100.0".to_string())
            .with_height("200.0".to_string());

        assert_eq!(panel.is_valid().unwrap(), false);
    }

    #[test]
    fn test_panel_is_valid_no_width() {
        let panel = Panel::new()
            .with_enabled(true)
            .with_count(5)
            .with_height("200.0".to_string());

        assert_eq!(panel.is_valid().unwrap(), false);
    }

    #[test]
    fn test_panel_is_valid_no_height() {
        let panel = Panel::new()
            .with_enabled(true)
            .with_count(5)
            .with_width("100.0".to_string());

        assert_eq!(panel.is_valid().unwrap(), false);
    }

    #[test]
    fn test_panel_is_valid_invalid_width() {
        let panel = Panel::new()
            .with_enabled(true)
            .with_count(5)
            .with_width("invalid".to_string())
            .with_height("200.0".to_string());

        assert!(panel.is_valid().is_err());
    }

    #[test]
    fn test_panel_is_valid_zero_width() {
        let panel = Panel::new()
            .with_enabled(true)
            .with_count(5)
            .with_width("0.0".to_string())
            .with_height("200.0".to_string());

        assert_eq!(panel.is_valid().unwrap(), false);
    }

    #[test]
    fn test_panel_is_valid_negative_height() {
        let panel = Panel::new()
            .with_enabled(true)
            .with_count(5)
            .with_width("100.0".to_string())
            .with_height("-50.0".to_string());

        assert_eq!(panel.is_valid().unwrap(), false);
    }

    #[test]
    fn test_panel_is_valid_success() {
        let panel = Panel::new()
            .with_enabled(true)
            .with_count(5)
            .with_width("100.5".to_string())
            .with_height("200.0".to_string());

        assert_eq!(panel.is_valid().unwrap(), true);
    }

    #[test]
    fn test_panel_width_as_f64() {
        let panel = Panel::new().with_width("123.45".to_string());
        assert_eq!(panel.width_as_f64().unwrap(), 123.45);

        let panel_no_width = Panel::new();
        assert!(panel_no_width.width_as_f64().is_err());

        let panel_invalid = Panel::new().with_width("invalid".to_string());
        assert!(panel_invalid.width_as_f64().is_err());
    }

    #[test]
    fn test_panel_height_as_f64() {
        let panel = Panel::new().with_height("67.89".to_string());
        assert_eq!(panel.height_as_f64().unwrap(), 67.89);

        let panel_no_height = Panel::new();
        assert!(panel_no_height.height_as_f64().is_err());

        let panel_invalid = Panel::new().with_height("invalid".to_string());
        assert!(panel_invalid.height_as_f64().is_err());
    }

    #[test]
    fn test_panel_area() {
        let panel = Panel::new()
            .with_width("10.0".to_string())
            .with_height("20.0".to_string());
        
        assert_eq!(panel.area().unwrap(), 200.0);

        let panel_no_dimensions = Panel::new();
        assert!(panel_no_dimensions.area().is_err());
    }

    #[test]
    fn test_panel_has_valid_dimensions() {
        let panel_valid = Panel::new()
            .with_width("10.0".to_string())
            .with_height("20.0".to_string());
        assert!(panel_valid.has_valid_dimensions());

        let panel_invalid = Panel::new().with_width("invalid".to_string());
        assert!(!panel_invalid.has_valid_dimensions());

        let panel_no_dimensions = Panel::new();
        assert!(!panel_no_dimensions.has_valid_dimensions());
    }

    #[test]
    fn test_panel_display() {
        let panel = Panel::new()
            .with_width("100.0".to_string())
            .with_height("200.0".to_string())
            .with_count(5)
            .with_enabled(true);

        assert_eq!(format!("{}", panel), "[100.0x200.0]*5");

        let disabled_panel = Panel::new()
            .with_width("50.0".to_string())
            .with_height("75.0".to_string())
            .with_count(3)
            .with_enabled(false);

        assert_eq!(format!("{}", disabled_panel), "[50.0x75.0]*3-disabled");

        let panel_no_dimensions = Panel::new().with_count(2).with_enabled(true);
        assert_eq!(format!("{}", panel_no_dimensions), "[?x?]*2");
    }
}

#[cfg(test)]
mod edge_tests {
    use cutlist_optimizer_cli::models::{edge::Edge, panel::Panel};


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
    fn test_edge_has_any_edge() {
        let empty_edge = Edge::new();
        assert!(!empty_edge.has_any_edge());

        let edge_with_top = Edge {
            top: Some("1mm".to_string()),
            ..Edge::default()
        };
        assert!(edge_with_top.has_any_edge());
    }

    #[test]
    fn test_edge_has_all_edges() {
        let empty_edge = Edge::new();
        assert!(!empty_edge.has_all_edges());

        let partial_edge = Edge {
            top: Some("1mm".to_string()),
            left: Some("1mm".to_string()),
            ..Edge::default()
        };
        assert!(!partial_edge.has_all_edges());

        let complete_edge = Edge::uniform("1mm".to_string());
        assert!(complete_edge.has_all_edges());
    }

    #[test]
    fn test_panel_with_edge() {
        let edge = Edge::uniform("2mm".to_string());
        let panel = Panel::new().with_edge(edge.clone());
        
        assert_eq!(panel.edge, Some(edge));
    }
}
