use chrono::Utc;

/// Build the url for the api access based on the requested parameter and today's date.
pub fn url() -> String {
    let date = Utc::now().date_naive().format("%Y%m%d");
    format!(
        "https://data.geo.admin.ch/api/stac/v0.9/collections/ch.meteoschweiz.ogd-local-forecasting/items/{date:08}-ch/assets/vnut12.lssw.{date:08}0100.dkl010h0.csv"
    )
}


pub enum Parameter {
    
}