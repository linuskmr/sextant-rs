use crate::angle::{Angle, Radians};
use chrono::Datelike;

/// Calculates the sun declination for a given date.
///
/// See <https://www.pveducation.org/pvcdrom/properties-of-sunlight/declination-angle>
///
/// # Example
/// ```
/// # use sextant_rs::sun_declination;
/// # use sextant_rs::angle::Degrees;
/// # use chrono::TimeZone;
/// let date = chrono::Utc.with_ymd_and_hms(2022, 02, 12, 0, 0, 0).unwrap();
/// assert_eq!(Degrees::from(sun_declination::calculate(date)), Degrees(-14.348736109751588));
/// ```
pub fn calculate(date: chrono::DateTime<chrono::Utc>) -> Angle {
    let day_of_year = date.ordinal();

    // An alternative formula from https://www.pveducation.org/pvcdrom/properties-of-sunlight/declination-angle
    // would look like `radToDeg(Math.asin(Math.sin(degToRad(-23.45)) * Math.cos(degToRad((360 / 365) * (days + 10))))`
    let declination = f64::to_radians(-23.45)
        * f64::cos(f64::to_radians(360.0 / 365.0) * (day_of_year + 10) as f64);
    Angle::from(Radians(declination))
}
