use std::sync::Arc;

use color_eyre::eyre::WrapErr;
use color_eyre::{Result, Section};
mod api;
mod weather_data;
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

    println!("{res:#?}");

    let mut body = res.text().await.wrap_err("Failed to get body")?;

    let mut body_lines_iter = body.as_str().lines();

    let _ = body_lines_iter.next();

    let new_csv = format!("location;location_type;date;value;{}", body_lines_iter.collect::<String>()).replace("\n", "\n");

    let mut csv_reader =
        csv::ReaderBuilder::new().delimiter(b';').from_reader(new_csv.as_bytes());

    for entry in csv_reader.deserialize::<WeatherDataEntry>(){
        let entry_vals = entry.wrap_err("Failed to parse CSV data")?;
        println!("Max Temp for location {id}: {t} °C", id = entry_vals.location, t = entry_vals.value);
    }

    Ok(())
}
