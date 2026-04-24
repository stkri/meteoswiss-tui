use color_eyre::eyre::WrapErr;
use color_eyre::{Result, Section};
use encoding_rs::WINDOWS_1252;
use std::sync::LazyLock;
use weather_data::WeatherDataEntry;

mod location;
mod weather_data;

pub static API_CLI: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let param = weather_data::Parameter::TotalSunshineDurationHourly;
    let res = API_CLI
        .get(weather_data::url_today(param))
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

    let location = location::Location(1312);

    let zh_entry = csv_reader
        .deserialize::<WeatherDataEntry>()
        .map(|e| e.unwrap())
        .filter(|e| e.location == location.get_api_id())
        .next()
        .unwrap();

    println!(
        "Maximum ... for {}: {} {} ",
        location.get_name().await?,
        zh_entry.value,
        param.associated_unit()
    );
    Ok(())
}
