#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TileDimensions;
    use crate::error::AppError;

    #[test]
    fn test_new_generator() {
        let tiles_to_fit = vec![
            TileDimensions::new(1, 100, 50),
            TileDimensions::new(2, 80, 60),
        ];
        let stock_tiles = vec![
            TileDimensions::new(10, 200, 100),
            TileDimensions::new(11, 150, 120),
        ];

        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
        assert!(generator.is_ok());
    }

    #[test]
    fn test_empty_tiles_error() {
        let tiles_to_fit = vec![];
        let stock_tiles = vec![TileDimensions::new(10, 200, 100)];

        let result = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
        assert!(matches!(result.unwrap_err(), AppError::NoTilesToFit));
    }

    #[test]
    fn test_empty_stock_error() {
        let tiles_to_fit = vec![TileDimensions::new(1, 100, 50)];
        let stock_tiles = vec![];

        let result = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None);
        assert!(matches!(result.unwrap_err(), AppError::NoStockTiles));
    }

    #[test]
    fn test_required_area_calculation() {
        let tiles_to_fit = vec![
            TileDimensions::new(1, 100, 50), // area = 5000
            TileDimensions::new(2, 80, 60),  // area = 4800
        ];
        let stock_tiles = vec![TileDimensions::new(10, 200, 100)];

        let generator = StockSolutionGenerator::new(tiles_to_fit, stock_tiles, None).unwrap();
        assert_eq!(generator.get_required_area(), 9800);
    }
}
