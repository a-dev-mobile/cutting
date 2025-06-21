use serde::{Deserialize, Serialize};

/// Priority criteria for optimization algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationPriority {
    MostTiles,
    LeastWastedArea,
    LeastNbrCuts,
    MostHvDiscrepancy,
    BiggestUnusedTileArea,
    SmallestCenterOfMassDistToOrigin,
    LeastNbrMosaics,
    LeastNbrUnusedTiles,
    MostUnusedPanelArea,
}

impl std::fmt::Display for OptimizationPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::MostTiles => "MOST_TILES",
            Self::LeastWastedArea => "LEAST_WASTED_AREA",
            Self::LeastNbrCuts => "LEAST_NBR_CUTS",
            Self::MostHvDiscrepancy => "MOST_HV_DISCREPANCY",
            Self::BiggestUnusedTileArea => "BIGGEST_UNUSED_TILE_AREA",
            Self::SmallestCenterOfMassDistToOrigin => "SMALLEST_CENTER_OF_MASS_DIST_TO_ORIGIN",
            Self::LeastNbrMosaics => "LEAST_NBR_MOSAICS",
            Self::LeastNbrUnusedTiles => "LEAST_NBR_UNUSED_TILES",
            Self::MostUnusedPanelArea => "MOST_UNUSED_PANEL_AREA",
        };
        write!(f, "{}", text)
    }
}

impl Default for OptimizationPriority {
    fn default() -> Self {
        Self::LeastWastedArea
    }
}
