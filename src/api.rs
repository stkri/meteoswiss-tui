use std::fmt::Display;

use chrono::Utc;

/// Build the url for the api access based on the requested parameter and date.
pub fn url(param: Parameter, date: chrono::DateTime<Utc>) -> String {
    let date = date.date_naive().format("%Y%m%d");
    format!(
        "https://data.geo.admin.ch/api/stac/v0.9/collections/ch.meteoschweiz.ogd-local-forecasting/items/{date:08}-ch/assets/vnut12.lssw.{date:08}0100.{}.csv",
        param.api_key(),
    )
}

/// Build the url for the api access based on the requested parameter and date.
pub fn url_today(param: Parameter) -> String {
    url(param, Utc::now())
}

/// Meteoroligical Data Parameters provided by the MeteoSwiss API
///
/// TP is short for 10th Percentile
/// NP is short for 90th Percentile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Parameter {
    MeanWindDirectionHourly,
    MeanWindSpeedScalarHourly,
    MaxGustPeakHourly,
    TPMeanWindSpeedHourly,
    TPMaxGustPeakHourly,
    NPMeanWindSpeedHourly,
    NPMaxGustPeakHourly,
    MeanGlobalRadiationHourly,
    // Meteoswiss Icon not provided to comply with copyright
    HighCloudCover,
    LowCloudCover,
    MediumCloudCover,
    MeanDiffuseRadiationHourly,
    ProbabilityPrecipitationThreeHours,
    TotalPrecipitationThreeHours,
    TotalPrecipitationHourly,
    TPTotalPrecipitationHourly,
    NPTotalPrecipitationHourly,
    TotalSunshineDurationHourly,
    MeanAirTemperatureHourly,
    TPMeanAirTemperatureHourly,
    NPMeanAirTemperatureHourly,
    ZeroDegreeLevelHourly,
    // MeteoSwiss Icon not provided to comply with copyright
    TotalPrecipitationDailyUTC,
    TotalPrecipitationDailyLocal,
    TPTotalPrecipitationDailyLocal,
    NPTotalPrecipitationDailyLocal,
    MinAirTemperatureDailyUTC,
    MaxAirTemperatureDailyUTC,
    MinAirTemperatureDailyLocal,
    MaxAirTemperatureDailyLocal,
}

impl Parameter {
    fn api_key(self) -> &'static str {
        use Parameter as P;
        match self {
            P::MeanWindDirectionHourly => "dkl010h0",
            P::MeanWindSpeedScalarHourly => "fu3010h0",
            P::MaxGustPeakHourly => "fu3010h1",
            P::TPMeanWindSpeedHourly => "fu3q10h0",
            P::TPMaxGustPeakHourly => "fu3q10h1",
            P::NPMeanWindSpeedHourly => "fu3q90h0",
            P::NPMaxGustPeakHourly => "fu3q90h1",
            P::MeanGlobalRadiationHourly => "gre000h0",
            P::HighCloudCover => "nprohihs",
            P::LowCloudCover => "nprolohs",
            P::MediumCloudCover => "npromths",
            P::MeanDiffuseRadiationHourly => "ods000h0",
            P::ProbabilityPrecipitationThreeHours => "rp0003i0",
            P::TotalPrecipitationThreeHours => "rre003i0",
            P::TotalPrecipitationHourly => "rre150h0",
            P::TPTotalPrecipitationHourly => "rreq10h0",
            P::NPTotalPrecipitationHourly => "rreq90h0",
            P::TotalSunshineDurationHourly => "sre000h0",
            P::MeanAirTemperatureHourly => "tre200h0",
            P::TPMeanAirTemperatureHourly => "treq10h0",
            P::NPMeanAirTemperatureHourly => "treq90h0",
            P::ZeroDegreeLevelHourly => "zprfr0hs",
            P::TotalPrecipitationDailyUTC => "rka150d0",
            P::TotalPrecipitationDailyLocal => "rka150p0",
            P::TPTotalPrecipitationDailyLocal => "rreq10p0",
            P::NPTotalPrecipitationDailyLocal => "rreq90p0",
            P::MinAirTemperatureDailyUTC => "tre200dn",
            P::MaxAirTemperatureDailyUTC => "tre200dx",
            P::MinAirTemperatureDailyLocal => "tre200pn",
            P::MaxAirTemperatureDailyLocal => "tre200px",
        }
    }
}
