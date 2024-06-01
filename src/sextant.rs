use crate::{
    angle::{Angle, Degrees},
    coordinate, hemisphere,
    sun_declination,
};

pub struct Sextant<Tz: chrono::TimeZone> {
    /// Date and time of measurement.
    pub datetime: chrono::DateTime<Tz>,
    /// Measured angle of the sun above the horizon.
    pub elevation: Angle,
    /// Error of the sextant.
    pub index_error: Angle,
    /// Time of culmination at the reference prime meridian (aka Greenwich).
    pub culmination_time_prime_meridian: chrono::DateTime<Tz>,
}

impl<Tz: chrono::TimeZone> Sextant<Tz> {
    /// ```
    /// # use sextant_rs::{angle::{Angle, Degrees, DegreeMinutesSeconds}, hemisphere::Latitude, coordinate};
    /// # use chrono::{TimeZone, FixedOffset, NaiveDate};
    /// # use sextant_rs::sextant::Sextant;
    /// let hour = 3600; // https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html#method.and_local_timezone
    /// let measurement_timezone = FixedOffset::east_opt(1 * hour).unwrap(); // CET
    /// let datetime = NaiveDate::from_ymd_opt(2022, 2, 12).unwrap()
    /// 	.and_hms_opt(12, 32, 0).unwrap()
    /// 	.and_local_timezone(measurement_timezone).unwrap();
    ///
    /// let primeMeridianTimezone = FixedOffset::east_opt(0).unwrap(); // British Winter Time/UTC. TODO: Maybe an example in the summer would be more interesting
    /// let culminationTimePrimeMeridian = NaiveDate::from_ymd_opt(2022, 2, 12).unwrap()
    /// 	.and_hms_opt(12, 14, 0).unwrap()
    /// 	.and_local_timezone(primeMeridianTimezone).unwrap();
    ///
    /// let sextant = Sextant {
    /// 	datetime,
    /// 	elevation: Angle::from(DegreeMinutesSeconds { degrees: 25, minutes: 8, seconds: 59.0 }),
    /// 	index_error: Angle::from(DegreeMinutesSeconds { degrees: 1, minutes: 42, seconds: 0.0 }),
    /// 	culminationTimePrimeMeridian,
    /// };
    /// let expected_latitude = 51.651263890248416;
    /// assert!(Degrees::from(sextant.calculate_latitude().angle).0 - expected_latitude < 2.0);
    /// ```
    pub fn calculate_latitude(&self) -> coordinate::Latitude {
        let corrected_elevation = self.elevation - self.index_error;
        let sun_declination = sun_declination::calculate(self.datetime.with_timezone(&chrono::Utc));
        let celestial_equator_height = corrected_elevation - sun_declination;
        dbg!(
            &corrected_elevation,
            Degrees::from(sun_declination),
            &celestial_equator_height
        );
        coordinate::Latitude {
            hemisphere: hemisphere::Latitude::North, // TODO: Could also be south. We just don't know it.
            angle: Angle::from(Degrees(90.0)) - celestial_equator_height,
        }
    }
}
