use chrono::{Datelike, Timelike};
use crate::{angle::Degrees, coordinate};

/// Calculates the culmination time as specified by <https://gml.noaa.gov/grad/solcalc/solareqns.PDF>
/// 
/// # Example
/// 
/// ```
/// # use sextant::{culmination, coordinate, angle, angle::Angle};
/// # use chrono::TimeZone;
/// let lng = coordinate::Longitude::new(Angle::from(angle::Degrees(0.0)));
/// let date = chrono::NaiveDate::from_ymd_opt(2024, 10, 6).unwrap();
/// assert_eq!(culmination(lng, date), chrono::NaiveDate::from_ymd_opt(2024, 10, 6).unwrap().and_hms_opt(11, 47, 38).unwrap());
/// ```
pub fn culmination(lng: coordinate::Longitude, date: chrono::NaiveDate) -> chrono::NaiveDateTime {
	let fractional_year_y = ((2.0 * std::f64::consts::PI) / 365.0) * (date.ordinal() as f64 - 1.0);
	let equation_of_time_in_minutes = 229.18 * (0.000075 + 0.001868 * fractional_year_y.cos() - 0.032077 * fractional_year_y.sin() - 0.014615 * (2.0*fractional_year_y).cos() - 0.040849 * (2.0*fractional_year_y).sin());
	let solar_noon_in_minutes = 720.0 - 4.0 * Degrees::from(lng.raw_angle).0 - equation_of_time_in_minutes;
	let solar_noon_hour_fraction = solar_noon_in_minutes / 60.0;
	let solar_noon_minute_fraction = solar_noon_hour_fraction.fract() * 60.0;
	let solar_noon_seconds_fraction = solar_noon_minute_fraction.fract() * 60.0;
	
	date.and_hms_opt(solar_noon_hour_fraction as u32, solar_noon_minute_fraction as u32, solar_noon_seconds_fraction as u32).unwrap()
}