use anyhow::Result;
mod api;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = reqwest::Client::new();

    let res = cli.get(api::url()).send().await?;

    println!("{res:#?}");
    Ok(())
}
