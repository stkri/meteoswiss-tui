use crate::location::Location;
use chrono::{DateTime, TimeZone, Utc};
use color_eyre::eyre::{ContextCompat, WrapErr};
use color_eyre::{Result, Section};
use encoding_rs::WINDOWS_1252;
use std::env;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

/// Build the url for the api access based on the requested parameter and date.
pub fn url(param: Parameter, date: DateTime<Utc>) -> String {
    let hour = date.format("%H");
    let date = date.format("%Y%m%d");
    format!(
        "https://data.geo.admin.ch/ch.meteoschweiz.ogd-local-forecasting/{date}-ch/vnut12.lssw.{date}{hour}00.{param}.csv",
        param = param.api_key(),
    )
}

/// Build the url for the api access based on the requested parameter and date.
pub fn url_today(param: Parameter) -> String {
    url(param, Utc::now() - chrono::TimeDelta::hours(2))
}

/// Meteoroligical Data Parameters provided by the MeteoSwiss API
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parameter {
    MeanWindDirectionHourly,
    MeanWindSpeedScalarHourly,
    MaxGustPeakHourly,
    MeanGlobalRadiationHourly,
    HighCloudCover,
    LowCloudCover,
    MediumCloudCover,
    MeanDiffuseRadiationHourly,
    ProbabilityPrecipitationThreeHours,
    TotalPrecipitationThreeHours,
    TotalPrecipitationHourly,
    TotalSunshineDurationHourly,
    MeanAirTemperatureHourly,
    ZeroDegreeLevelHourly,
    TotalPrecipitationDailyLocal,
    MinAirTemperatureDailyLocal,
    MaxAirTemperatureDailyLocal,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Parameter as P;
        let label = match &self {
            P::MeanWindDirectionHourly => "hourly mean wind direction",
            P::MeanWindSpeedScalarHourly => "hourly mean windspeed",
            P::MaxGustPeakHourly => "hourly maximum gust peak",
            P::MeanGlobalRadiationHourly => "hourly mean global radiation",
            P::HighCloudCover => "high cloud cover",
            P::LowCloudCover => "low cloud cover",
            P::MediumCloudCover => "medium cloud cover",
            P::MeanDiffuseRadiationHourly => "hourly mean diffuse radiation",
            P::ProbabilityPrecipitationThreeHours => {
                "probability of precipitation in the next three hours"
            }
            P::TotalPrecipitationThreeHours => "total precipitation in the next three hours",
            P::TotalPrecipitationHourly => "total precipitation in the next hour",
            P::TotalSunshineDurationHourly => "total sunshine duration in the next hour",
            P::MeanAirTemperatureHourly => "hourly mean air temperature",
            P::ZeroDegreeLevelHourly => "hourly zero degree level",
            P::TotalPrecipitationDailyLocal => "total precipition today",
            P::MinAirTemperatureDailyLocal => "minimum air temperature today",
            P::MaxAirTemperatureDailyLocal => "maximum air temperature today",
        };
        f.write_str(label)?;
        Ok(())
    }
}

impl Parameter {
    /// Returns the 8 character key used by the API for the parameters
    fn api_key(self) -> &'static str {
        use Parameter as P;
        match self {
            P::MeanWindDirectionHourly => "dkl010h0",
            P::MeanWindSpeedScalarHourly => "fu3010h0",
            P::MaxGustPeakHourly => "fu3010h1",
            P::MeanGlobalRadiationHourly => "gre000h0",
            P::HighCloudCover => "nprohihs",
            P::LowCloudCover => "nprolohs",
            P::MediumCloudCover => "npromths",
            P::MeanDiffuseRadiationHourly => "ods000h0",
            P::ProbabilityPrecipitationThreeHours => "rp0003i0",
            P::TotalPrecipitationThreeHours => "rre003i0",
            P::TotalPrecipitationHourly => "rre150h0",
            P::TotalSunshineDurationHourly => "sre000h0",
            P::MeanAirTemperatureHourly => "tre200h0",
            P::ZeroDegreeLevelHourly => "zprfr0hs",
            P::TotalPrecipitationDailyLocal => "rka150p0",
            P::MinAirTemperatureDailyLocal => "tre200pn",
            P::MaxAirTemperatureDailyLocal => "tre200px",
        }
    }

    /// Returns the unit which is associated with a parameter.
    pub fn associated_unit(self) -> &'static str {
        use Parameter as P;
        match self {
            P::MeanWindDirectionHourly => "°",
            P::MeanWindSpeedScalarHourly | P::MaxGustPeakHourly => "km/h",
            P::MeanGlobalRadiationHourly | P::MeanDiffuseRadiationHourly => "W/m²",
            P::ZeroDegreeLevelHourly => "m",
            P::LowCloudCover
            | P::MediumCloudCover
            | P::HighCloudCover
            | P::ProbabilityPrecipitationThreeHours => "%",
            P::TotalPrecipitationHourly
            | P::TotalPrecipitationDailyLocal
            | P::TotalPrecipitationThreeHours => "mm",
            P::TotalSunshineDurationHourly => "min/h",
            P::MeanAirTemperatureHourly
            | P::MaxAirTemperatureDailyLocal
            | P::MinAirTemperatureDailyLocal => "°C",
        }
    }
}

/// A singular forecast datapoint, which includes the value, the location, and the date.
#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct WeatherDataEntry {
    /// The API id for the location.
    pub location: u32,
    _location_type: u8,
    date: u64,
    /// The forecast value
    pub value: f64,
}

impl WeatherDataEntry {
    /// Get the date value for the entry
    pub fn date(&self) -> Result<DateTime<Utc>> {
        let mut date_val = self.date;
        let minute = date_val % 1_00;
        date_val /= 1_00;
        let hour = date_val % 1_00;
        date_val /= 1_00;
        let day = date_val % 1_00;
        date_val /= 1_00;
        let month = date_val % 1_00;
        date_val /= 1_00;
        let year = date_val;
        Utc.with_ymd_and_hms(
            year as i32,
            month as u32,
            day as u32,
            hour as u32,
            minute as u32,
            0,
        )
        .single()
        .context("Failure when parsing date and time from API response")
    }
}

/// Return a list of entries for the give locations.
/// Fails if the location is invalid and/or no connection can be established.
pub async fn get_entries(
    param: Parameter,
    locations: &[Location],
) -> Result<Vec<WeatherDataEntry>> {
    let res = crate::API_CLI
        .get(url_today(param))
        .send()
        .await
        .wrap_err("Failed to access MeteoSwiss API")
        .suggestion("Check your internet connection")?;

    let body = res.text().await.wrap_err("Failed to get body")?;

    let (cow, _, _) = WINDOWS_1252.decode(body.as_bytes());

    let body = cow.into_owned();

    let mut body_lines_iter = body.lines();
    let _ = body_lines_iter.next();
    let body = body_lines_iter
        .map(|s| format!("{s}\n"))
        .collect::<String>();

    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(body.as_bytes());

    let int_locations = locations.iter().map(|l| l.get_api_id()).collect::<Vec<_>>();
    Ok(csv_reader
        .deserialize::<WeatherDataEntry>()
        .map(|e| e.expect("Format of repsonse CSV should be valid"))
        .filter(|e| int_locations.contains(&e.location))
        .collect())
}

/// Create a directory in temp to store CSVs
pub fn init_cache() -> Result<()> {
    // Make sure any caches left over due to ungraceful quits aren't used.
    // We can safely ignore this Result<()>
    // If the function returns Ok(()), the function has successfully deleted the temp file.
    // If it returns Err, the cache didn't exist to begin with.
    let _ = clear_cache();

    let mut dir = env::temp_dir();
    dir.push("meteoswiss-tui");

    fs::DirBuilder::new()
        .recursive(true)
        .create(&dir)
        .wrap_err(format!("Error when creating cache at {}", dir.display()))
}

/// Clear any cache directory in temp
pub fn clear_cache() -> Result<()> {
    let mut dir = env::temp_dir();
    dir.push("meteoswiss-tui");

    fs::remove_dir_all(&dir).wrap_err(format!("Error when clearing cache at {}", dir.display()))
}
