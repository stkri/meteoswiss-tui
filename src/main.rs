use crate::location::Location;
use crate::weather_data::{Parameter, get_entries};
use color_eyre::Result;
use color_eyre::eyre::ContextCompat;
use std::sync::LazyLock;

mod location;
mod weather_data;

pub static API_CLI: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    weather_data::init_cache()?;

    let locations = vec![
        Location::new(6318).wrap_err("When making location")?,
        Location::new(8001).wrap_err("When making location")?,
        Location::new(1312).wrap_err("When making location")?,
        Location::new(9485).wrap_err("When making location")?,
    ];

    let param = Parameter::MaxAirTemperatureDailyLocal;
    let entries = get_entries(param, &locations).await?;

    for entry in entries {
        println!(
            "{} for {} @ {}: {} {} ",
            param.to_string(),
            Location::from_api_id(entry.location).get_name().await?,
            entry.date()?.format("%d/%m %H:00"),
            entry.value,
            param.associated_unit()
        );
    }

    weather_data::clear_cache()?;

    Ok(())
}
