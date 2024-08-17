//! Hemisphere of latitudes (i.e. northern or southern) and longitudes (i.e. eastern or western).

use std::fmt;

/// Northern or southern hemisphere.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Latitude {
    North,
    South,
}

/// Eastern or western hemisphere.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Longitude {
    East,
    West,
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Latitude::North => "North",
                Latitude::South => "South",
            }
        )
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Longitude::East => "East",
                Longitude::West => "West",
            }
        )
    }
}
