use anyhow::Result;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = reqwest::Client::new();
    let param = api::Parameter::MaxAirTemperatureDailyLocal;
    let res = cli.get(api::url(param)).send().await?;

    println!("{res:#?}");
    Ok(())
}
