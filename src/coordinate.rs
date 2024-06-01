use core::fmt;

use crate::{
    angle::{self, DegreeMinutesSeconds},
    hemisphere,
};

/// Specifies the **north–south position** of a point on the Earth's surface.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Latitude {
    pub hemisphere: hemisphere::Latitude,
    pub angle: angle::Angle,
}

/// Specifies the **east–west position** of a point on the Earth's surface.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Longitude {
    pub hemisphere: hemisphere::Longitude,
    pub angle: angle::Angle,
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.hemisphere,
            DegreeMinutesSeconds::from(self.angle)
        )
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.hemisphere,
            DegreeMinutesSeconds::from(self.angle)
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Coordinate {
    latitude: Latitude,
    longitude: Longitude,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.latitude, self.longitude)
    }
}
