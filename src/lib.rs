#![doc = include_str!("../README.md")]
//! sextant-calculator calculates the geographic position (latitude, longitude) on the Earth's surface using a sextant measurement.
//! 
//! # Example
//! 
//! ```
//! # use sextant::angle::{Angle, DegreeMinutesSeconds, Degrees};
//! # use sextant::SextantMeasurement;
//! # use sextant::coordinate;
//! # use chrono::{FixedOffset, NaiveDate};
//! let culmination_time = {
//! 	let hour = 60 * 60;
//! 	let timezone = FixedOffset::east_opt(2 * hour).unwrap(); // CET
//! 	NaiveDate::from_ymd_opt(2024, 8, 17).unwrap()
//! 		.and_hms_opt(13, 30, 22).unwrap()
//! 		.and_local_timezone(timezone).unwrap()
//! };
//!
//! let culmination_time_prime_meridian = {
//! 	let hour = 60 * 60;
//! 	let timezone = FixedOffset::east_opt(1 * hour).unwrap();
//! 	culmination_time.date_naive() // Same date as measurement
//! 		.and_hms_opt(13, 4, 45).unwrap()
//! 		.and_local_timezone(timezone).unwrap()
//! };
//!
//! let sextant_measurement = SextantMeasurement {
//! 	culmination_time,
//! 	elevation: Angle::from(DegreeMinutesSeconds { degrees: 53, minutes: 46, seconds: 0.0 }),
//! 	index_error: Angle::from(DegreeMinutesSeconds { degrees: 0, minutes: 0, seconds: 0.0 }),
//! 	culmination_time_prime_meridian,
//! };
//! 
//! let expected_latitude = coordinate::Latitude::new(Angle::from(Degrees(49.014)));
//! let actual_latitude = sextant_measurement.calculate_latitude();
//! let latitude_difference = actual_latitude.raw_angle - expected_latitude.raw_angle;
//! assert!(Degrees::from(latitude_difference).0.abs() < 1.0, "expected={:?} actual={:?}", expected_latitude.raw_angle, actual_latitude.raw_angle);
//! 
//! let expected_longitude = coordinate::Longitude::new(Angle::from(Degrees(8.404)));
//! let actual_longitude = sextant_measurement.calculate_longitude();
//! let longitude_difference = actual_longitude.raw_angle - expected_longitude.raw_angle;
//! assert!(Degrees::from(longitude_difference).0.abs() < 0.5, "expected={:?} actual={:?}", expected_longitude.raw_angle, actual_longitude.raw_angle);
//! ```

pub mod angle;
pub mod coordinate;
pub mod hemisphere;
mod sextant;
mod sun_declination;

pub use sun_declination::calculate as calculate_sun_declination;
pub use sextant::SextantMeasurement;
