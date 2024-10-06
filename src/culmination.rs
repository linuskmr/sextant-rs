use chrono::Datelike;
use crate::{angle::{Angle, Degrees}, coordinate};

/// Calculates the culmination time as specified by <https://gml.noaa.gov/grad/solcalc/solareqns.PDF>
/// 
/// # Example
/// 
/// ```
/// # use sextant::{culmination, coordinate, angle, angle::Angle};
/// # use chrono::TimeZone;
/// let lng = coordinate::Longitude::new(Angle::from(angle::Degrees(0.0)));
/// let date = chrono::NaiveDate::from_ymd_opt(2024, 10, 6).unwrap();
/// assert_eq!(culmination(lng, date), chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(chrono::NaiveDate::from_ymd_opt(2024, 10, 6).unwrap().and_hms_opt(11, 47, 38).unwrap(), chrono::Utc));
/// ```
pub fn culmination(lng: coordinate::Longitude, date: chrono::NaiveDate) -> chrono::DateTime<chrono::Utc> {
	let fractional_year_y = ((2.0 * std::f64::consts::PI) / 365.0) * (date.ordinal() as f64 - 1.0);
	let equation_of_time_in_minutes = 229.18 * (0.000075 + 0.001868 * fractional_year_y.cos() - 0.032077 * fractional_year_y.sin() - 0.014615 * (2.0*fractional_year_y).cos() - 0.040849 * (2.0*fractional_year_y).sin());
	let solar_noon_in_minutes = 720.0 - 4.0 * Degrees::from(lng.raw_angle).0 - equation_of_time_in_minutes;

	let solar_noon_hour_fraction = solar_noon_in_minutes / 60.0;
	let solar_noon_minute_fraction = solar_noon_hour_fraction.fract() * 60.0;
	let solar_noon_seconds_fraction = solar_noon_minute_fraction.fract() * 60.0;
	
	let culmination_time = chrono::NaiveTime::from_hms_opt(solar_noon_hour_fraction as u32, solar_noon_minute_fraction as u32, solar_noon_seconds_fraction as u32).unwrap();
	let culmination_datetime = chrono::NaiveDateTime::new(date, culmination_time);
	chrono::DateTime::from_naive_utc_and_offset(culmination_datetime, chrono::Utc)
}


/// Calculates the culmination for the prime meridian by forwarding the date to the `culmination` function.
pub fn reference_culmination(date: chrono::NaiveDate) -> chrono::DateTime<chrono::Utc> {
	let reference_longitude = coordinate::Longitude::new(Angle::from(Degrees(0.0)));
	culmination(reference_longitude, date)
}