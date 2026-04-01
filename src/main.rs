use std::sync::Arc;

use color_eyre::eyre::WrapErr;
use color_eyre::{Result, Section};
mod api;
mod weather_data;
use encoding_rs::WINDOWS_1252;
use weather_data::WeatherDataEntry;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = reqwest::Client::new();
    let param = api::Parameter::MaxAirTemperatureDailyLocal;
    let res = cli
        .get(api::url_today(param))
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

    let location = weather_data::Location::Plz(8001);

    let zh_entry = csv_reader
        .deserialize::<WeatherDataEntry>()
        .map(|e| e.unwrap())
        .filter(|e| e.location == location.into())
        .next()
        .unwrap();

    println!("Maximum temp for Zurich: {} °C", zh_entry.value);
    Ok(())
}
