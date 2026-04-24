use color_eyre::eyre::ContextCompat;
use color_eyre::eyre::WrapErr;
use color_eyre::{Result, Section};
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct Location(pub u32);

impl Location {
    pub fn get_api_id(self) -> u32 {
        self.0 * 100
    }

    pub async fn get_name(self) -> Result<String> {
        // Because Liechtenstein and Switzerland do not have any
        // locations which share postal codes, we can easily determine
        // the country by checking if it is one of 15 valid postal codes
        // in Liechtenstein. (Not all map to places))
        // This is necessary, as there are different API endpoints for
        // Switzerland and Liechtenstein.
        let country = if self.0 >= 9485 && self.0 < 9500 {
            "li"
        } else {
            "ch"
        };
        let url = format!(
            "https://openplzapi.org/{country}/Localities?postalCode={:04}",
            self.0
        );
        let response = crate::API_CLI
            .get(url)
            .send()
            .await
            .wrap_err("Failed to access OpenPLZ API")
            .suggestion("Check your internet connection")?
            .text()
            .await
            .wrap_err("Failed to get body")?;

        let root: Value =
            serde_json::from_str(&response).wrap_err("Failed to parse location response")?;

        let mut result = root
            .get(0)
            .and_then(|value| value.get("name"))
            .wrap_err("Missing location name field")?
            .to_string();

        result.pop();
        result.remove(0);

        Ok(result)
    }
}
