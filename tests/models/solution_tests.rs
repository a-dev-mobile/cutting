#[cfg(test)]
mod tests {
    use cutlist_optimizer_cli::{
        models::{Mosaic, Solution, TileDimensions},
        Orientation,
    };

    fn create_test_tile_dimensions() -> TileDimensions {
        TileDimensions {
            id: 1,
            width: 100,
            height: 200,
            label: Some("Test Panel".to_string()),
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        }
    }

    #[test]
    fn test_solution_new() {
        let solution = Solution::new();

        assert!(solution.get_id() > 0);
        assert!(solution.get_timestamp() > 0);
        assert_eq!(solution.get_nbr_mosaics(), 0);
        assert_eq!(solution.get_no_fit_panels().len(), 0);
        assert_eq!(solution.get_unused_stock_panels().len(), 0);
        assert!(solution.get_aux_info().is_none());
        assert!(solution.get_creator_thread_group().is_none());
    }

    #[test]
    fn test_solution_from_tile_dimensions() {
        let tile_dims = create_test_tile_dimensions();
        let solution = Solution::from_tile_dimensions(&tile_dims);

        assert_eq!(solution.get_nbr_mosaics(), 1);
        assert_eq!(solution.get_total_area(), 20000); // 100 * 200
        assert!(solution.get_material().is_some());
    }

    #[test]
    fn test_solution_from_solution() {
        let tile_dims = create_test_tile_dimensions();
        let original = Solution::from_tile_dimensions(&tile_dims);
        let copied = Solution::from_solution(&original);

        // Should have different IDs but same content
        assert_ne!(original.get_id(), copied.get_id());
        assert_eq!(original.get_nbr_mosaics(), copied.get_nbr_mosaics());
        assert_eq!(original.get_total_area(), copied.get_total_area());
    }

    #[test]
    fn test_solution_add_mosaic() {
        let mut solution = Solution::new();
        let tile_dims = create_test_tile_dimensions();
        let mosaic = Mosaic::from_tile_dimensions(&tile_dims);

        solution.add_mosaic(mosaic);

        assert_eq!(solution.get_nbr_mosaics(), 1);
        assert_eq!(solution.get_total_area(), 20000);
    }

    #[test]
    fn test_solution_add_multiple_mosaics() {
        let mut solution = Solution::new();
        let tile_dims1 = TileDimensions {
            id: 1,
            width: 100,
            height: 200,
            label: None,
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };
        let tile_dims2 = TileDimensions {
            id: 2,
            width: 150,
            height: 100,
            label: None,
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };

        let mosaics = vec![
            Mosaic::from_tile_dimensions(&tile_dims1),
            Mosaic::from_tile_dimensions(&tile_dims2),
        ];

        solution.add_all_mosaics(mosaics);

        assert_eq!(solution.get_nbr_mosaics(), 2);
        assert_eq!(solution.get_total_area(), 35000); // 20000 + 15000
    }

    #[test]
    fn test_solution_no_fit_panels() {
        let mut solution = Solution::new();
        let tile_dims = create_test_tile_dimensions();

        solution.add_all_no_fit_panels(vec![tile_dims.clone()]);

        assert_eq!(solution.get_no_fit_panels().len(), 1);
        assert_eq!(solution.get_no_fit_panels()[0], tile_dims);
    }

    #[test]
    fn test_solution_aux_info() {
        let mut solution = Solution::new();

        assert!(solution.get_aux_info().is_none());

        solution.set_aux_info(Some("Test info".to_string()));
        assert_eq!(solution.get_aux_info(), Some("Test info"));

        solution.set_aux_info(None);
        assert!(solution.get_aux_info().is_none());
    }

    #[test]
    fn test_solution_creator_thread_group() {
        let mut solution = Solution::new();

        assert!(solution.get_creator_thread_group().is_none());

        solution.set_creator_thread_group(Some("Thread-1".to_string()));
        assert_eq!(solution.get_creator_thread_group(), Some("Thread-1"));

        solution.set_creator_thread_group(None);
        assert!(solution.get_creator_thread_group().is_none());
    }

    #[test]
    fn test_solution_efficiency() {
        let tile_dims = create_test_tile_dimensions();
        let solution = Solution::from_tile_dimensions(&tile_dims);

        // New solution should have 0% efficiency (no final tiles yet)
        assert_eq!(solution.get_used_area_ratio(), 0.0);
        assert_eq!(solution.get_used_area(), 0);
        assert_eq!(solution.get_unused_area(), 20000);
    }

    #[test]
    fn test_solution_bases_as_string() {
        let mut solution = Solution::new();
        let tile_dims1 = TileDimensions {
            id: 1,
            width: 100,
            height: 200,
            label: None,
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };
        let tile_dims2 = TileDimensions {
            id: 2,
            width: 150,
            height: 100,
            label: None,
            material: "Wood".to_string(),
            orientation: Orientation::Any,
            is_rotated: false,
        };

        solution.add_mosaic(Mosaic::from_tile_dimensions(&tile_dims1));
        solution.add_mosaic(Mosaic::from_tile_dimensions(&tile_dims2));

        let bases_string = solution.get_bases_as_string();
        assert!(bases_string.contains("[100x200]"));
        assert!(bases_string.contains("[150x100]"));
    }

    #[test]
    fn test_solution_is_empty() {
        let solution = Solution::new();
        assert!(solution.is_empty());

        let tile_dims = create_test_tile_dimensions();
        let solution_with_mosaic = Solution::from_tile_dimensions(&tile_dims);
        assert!(!solution_with_mosaic.is_empty());
    }

    #[test]
    fn test_solution_summary() {
        let tile_dims = create_test_tile_dimensions();
        let solution = Solution::from_tile_dimensions(&tile_dims);

        let summary = solution.get_summary();
        assert!(summary.contains("Solution #"));
        assert!(summary.contains("1 mosaics"));
        assert!(summary.contains("0 no-fit panels"));
        assert!(summary.contains("0 unused stock panels"));
        assert!(summary.contains("efficiency"));
    }

    #[test]
    fn test_solution_default() {
        let solution = Solution::default();
        assert_eq!(solution.get_nbr_mosaics(), 0);
        assert!(solution.is_empty());
    }
}
