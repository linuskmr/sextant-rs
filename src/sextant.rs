use crate::{
	angle::{Angle, Degrees}, coordinate, reference_culmination, sun_declination
};

/// A measurement of the sun's *angle* above the horizon (i.e. [elevation](Self::elevation)) at the *time* of its highest point (i.e. [culmination time](Self::culmination_time)).
pub struct SextantMeasurement<Tz: chrono::TimeZone> {
	/// Date and time of the measurement (i.e. at culmination).
	pub culmination_time: chrono::DateTime<Tz>,
	/// Measured angle of the sun above the horizon.
	pub elevation: Angle,
	/// Error of the sextant.
	pub index_error: Angle,
}

impl<Tz: chrono::TimeZone> SextantMeasurement<Tz> {
	/// Calculate the latitude of the observer.
	/// 
	/// # Example
	/// 
	/// See the [crate-level example](crate#example).
	pub fn calculate_latitude(&self) -> coordinate::Latitude {
		let corrected_elevation = self.elevation - self.index_error;
		let sun_declination = sun_declination::calculate(self.culmination_time.with_timezone(&chrono::Utc));
		let celestial_equator_height = corrected_elevation - sun_declination;
		dbg!(
			&corrected_elevation,
			Degrees::from(sun_declination),
			&celestial_equator_height
		);
		let latitude = Angle::from(Degrees(90.0)) - celestial_equator_height;
		coordinate::Latitude::new(latitude)
	}

	/// Calculate the longitude of the observer.
	/// 
	/// # Example
	/// 
	/// See the [crate-level example](crate#example).
	pub fn calculate_longitude(&self) -> coordinate::Longitude {
		let culmination_time_prime_meridian = reference_culmination(self.culmination_time.date_naive());
		let sun_peak_delta = culmination_time_prime_meridian - self.culmination_time.to_utc();
		println!("sun_peak_delta={}", sun_peak_delta.num_minutes());
		let sun_peak_delta_seconds = sun_peak_delta.num_seconds();
		let sun_peak_delta_hours = sun_peak_delta_seconds as f64 / 60.0 / 60.0;
		let longitude = sun_peak_delta_hours * (180.0 / 12.0);
		coordinate::Longitude::new(Angle::from(Degrees(longitude)))
	}
}