use std::fmt;

/// Specifies the **north–south position** of a point on the Earth's surface.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Latitude {
    North,
    South,
}

/// Specifies the **east-west position** of a point on the Earth's surface.
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
