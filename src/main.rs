use crate::location::Location;
use crate::weather_data::{Parameter, get_entries};
use color_eyre::Result;
use std::sync::LazyLock;

mod location;
mod weather_data;

pub static API_CLI: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let locations = vec![Location(6318), Location(8001), Location(1312)];

    let param = Parameter::MaxAirTemperatureDailyLocal;
    let entries = get_entries(param, &locations).await.unwrap();

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
    Ok(())
}
