use std::ops;
use std::fmt;

/// Specifies an angle in either degrees or radians.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Angle {
    Degrees(Degrees),
    Radians(Radians),
}

// Arithmetic operations for Angle

impl ops::Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Angle::Degrees(lhs), Angle::Degrees(rhs)) => Angle::Degrees(Degrees(lhs.0 + rhs.0)),
            (Angle::Radians(lhs), Angle::Radians(rhs)) => Angle::Radians(Radians(lhs.0 + rhs.0)),
            (Angle::Degrees(lhs), Angle::Radians(rhs)) => {
                Angle::Degrees(Degrees(lhs.0 + Degrees::from(rhs).0))
            }
            (Angle::Radians(lhs), Angle::Degrees(rhs)) => {
                Angle::Radians(Radians(lhs.0 + Radians::from(rhs).0))
            }
        }
    }
}

impl ops::Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Angle::Degrees(lhs), Angle::Degrees(rhs)) => Angle::Degrees(Degrees(lhs.0 - rhs.0)),
            (Angle::Radians(lhs), Angle::Radians(rhs)) => Angle::Radians(Radians(lhs.0 - rhs.0)),
            (Angle::Degrees(lhs), Angle::Radians(rhs)) => {
                Angle::Degrees(Degrees(lhs.0 - Degrees::from(rhs).0))
            }
            (Angle::Radians(lhs), Angle::Degrees(rhs)) => {
                Angle::Radians(Radians(lhs.0 - Radians::from(rhs).0))
            }
        }
    }
}

/// ```
/// # use sextant::angle::{Angle, Degrees};
/// Angle::from(Degrees(90.0));
/// ```
impl From<Degrees> for Angle {
    fn from(degrees: Degrees) -> Self {
        Self::Degrees(degrees)
    }
}

/// ```
/// # use sextant::angle::{Angle, Radians};
/// Angle::from(Radians(std::f64::consts::PI));
/// ```
impl From<Radians> for Angle {
    fn from(radians: Radians) -> Self {
        Self::Radians(radians)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radians(pub f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Degrees(pub f64);

// Arithmetic operations for Radians and Degrees

impl ops::Add for Degrees {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::Sub for Radians {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl ops::Sub for Degrees {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl ops::Add for Radians {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// ```
/// # use sextant::angle::{Angle, Degrees};
/// let degrees = Degrees(90.0);
/// let angle = Angle::from(degrees);
/// assert_eq!(Degrees::from(angle), degrees);
/// ```
impl From<Angle> for Degrees {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degrees(degrees) => degrees,
            Angle::Radians(radians) => Degrees::from(radians),
        }
    }
}

/// ```
/// # use sextant::angle::{Angle, Radians};
/// let radians = Radians(90.0);
/// let angle = Angle::from(radians);
/// assert_eq!(Radians::from(angle), radians);
/// ```
impl From<Angle> for Radians {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Degrees(degrees) => Radians::from(degrees),
            Angle::Radians(radians) => radians,
        }
    }
}

/// ```
/// # use sextant::angle::{Degrees, Radians};
/// let radians = Radians::from(Degrees(90.0));
/// assert_eq!(radians, Radians(std::f64::consts::PI / 2.0));
/// ```
impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        Self(radians.0 * (180.0 / std::f64::consts::PI))
    }
}

/// ```
/// # use sextant::angle::{Degrees, Radians};
/// let degrees = Degrees::from(Radians(std::f64::consts::PI));
/// assert_eq!(degrees, Degrees(180.0));
/// ```
impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        Self(degrees.0 * (std::f64::consts::PI / 180.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DegreeMinutesSeconds {
    pub degrees: i32,
    pub minutes: i32,
    pub seconds: f64,
}

/// ```
/// # use sextant::angle::{Angle, Degrees, DegreeMinutesSeconds};
/// let angle = Angle::from(Degrees(90.0));
/// let expected_dms = DegreeMinutesSeconds { degrees: 90, minutes: 0, seconds: 0.0 };
/// assert_eq!(DegreeMinutesSeconds::from(angle), expected_dms);
/// ```
impl From<Angle> for DegreeMinutesSeconds {
    fn from(angle: Angle) -> Self {
        let degrees_float = Degrees::from(angle).0;

        let degrees = degrees_float.floor();
        let minutes = (degrees_float - degrees).floor();
        let seconds = degrees_float - degrees - minutes;

        Self {
            degrees: degrees as i32,
            minutes: (minutes * 60.0) as i32,
            seconds: seconds * 3600.0,
        }
    }
}

/// ```
/// # use sextant::angle::{Angle, DegreeMinutesSeconds, Degrees};
/// let dms = DegreeMinutesSeconds { degrees: 90, minutes: 0, seconds: 0.0 };
/// assert_eq!(Angle::from(dms), Angle::from(Degrees(90.0)));
/// ```
impl From<DegreeMinutesSeconds> for Angle {
    fn from(dms: DegreeMinutesSeconds) -> Self {
        let degrees = dms.degrees as f64 + dms.minutes as f64 / 60.0 + dms.seconds / 3600.0;
        Self::Degrees(Degrees(degrees))
    }
}

/// ```
/// # use sextant::angle::DegreeMinutesSeconds;
/// let dms = DegreeMinutesSeconds { degrees: 47, minutes: 12, seconds: 3.1 };
/// assert_eq!(format!("{}", dms), "47°12'03\"");
/// ```
impl fmt::Display for DegreeMinutesSeconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}°{:02}'{:02.0}\"",
            self.degrees, self.minutes, self.seconds
        )
    }
}
