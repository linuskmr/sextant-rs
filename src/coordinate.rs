//! Latitude, Longitude, and Coordinate types.

use core::fmt;

use crate::{
    angle::{self, Angle, DegreeMinutesSeconds},
    hemisphere,
};

/// *North–south* position of a point on the Earth's surface.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Latitude {
    /// The raw angle is positive for the [northern](hemisphere::Latitude::North) and negative for the [southern](hemisphere::Latitude::South) hemisphere.
    pub raw_angle: angle::Angle,
}

/// *East–west* position of a point on the Earth's surface.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Longitude {
    /// The raw angle is positive for the [eastern](hemisphere::Longitude::East) and negative for the [western](hemisphere::Longitude::West) hemisphere.
    pub raw_angle: angle::Angle,
}

/// *Geographic position* of a point on the Earth's surface using [`Latitude`] and [`Longitude`].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Coordinate {
    /// North–south position
    latitude: Latitude,
    /// East–west position
    longitude: Longitude,
}


impl Latitude {
    pub fn new(raw_angle: angle::Angle) -> Self {
        Self { raw_angle }
    }

    /// Returns the (always positive) angle of the latitude in the corresponding [`hemisphere`](Self::hemisphere).
    pub fn angle(&self) -> angle::Angle {
        if angle::Degrees::from(self.raw_angle).0 < 0.0 {
            Angle::from(angle::Degrees(angle::Degrees::from(self.raw_angle).0.abs()))
        } else {
            self.angle()
        }
    }

    /// Returns whether the latitude is in the [northern](hemisphere::Latitude::North) or the [southern](hemisphere::Latitude::South) hemisphere.
    /// 
    /// [Northern hemisphere](hemisphere::Latitude::North) for positive [`Self::raw_angle`] and [southern hemisphere](hemisphere::Latitude::South) for negative values.
    pub fn hemisphere(&self) -> hemisphere::Latitude {
        if angle::Degrees::from(self.raw_angle).0 < 0.0 {
            hemisphere::Latitude::South
        } else {
            hemisphere::Latitude::North
        }
    }
}

impl Longitude {
    pub fn new(raw_angle: angle::Angle) -> Self {
        Self { raw_angle }
    }

    /// Returns the (always positive) angle of the longitude in the corresponding [`hemisphere`](Self::hemisphere).
    pub fn angle(&self) -> angle::Angle {
        if angle::Degrees::from(self.raw_angle).0 < 0.0 {
            Angle::from(angle::Degrees(angle::Degrees::from(self.raw_angle).0.abs()))
        } else {
            self.raw_angle
        }
    }

    /// Returns whether the longitude is in the [eastern](hemisphere::Longitude::East) or the [western](hemisphere::Longitude::West) hemisphere.
    ///
    /// [Western hemisphere](hemisphere::Longitude::West) for positive [`Self::raw_angle`] and [eastern hemisphere](hemisphere::Longitude::East) for negative values.
    pub fn hemisphere(&self) -> hemisphere::Longitude {
        if angle::Degrees::from(self.raw_angle).0 < 0.0 {
            hemisphere::Longitude::West
        } else {
            hemisphere::Longitude::East
        }
    }
}

impl fmt::Display for Latitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.hemisphere(),
            DegreeMinutesSeconds::from(self.angle())
        )
    }
}

impl fmt::Display for Longitude {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}",
            self.hemisphere(),
            DegreeMinutesSeconds::from(self.angle())
        )
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.latitude, self.longitude)
    }
}
