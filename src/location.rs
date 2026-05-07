use std::collections::BTreeMap;
use tokio::sync::RwLock;

use color_eyre::eyre::{ContextCompat, WrapErr};
use color_eyre::{Result, Section};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location(u32);

pub static LOCATION_CACHE: RwLock<BTreeMap<Location, String>> = RwLock::const_new(BTreeMap::new());

pub const LOCATION_CACHE_LIMIT: usize = 20;

impl Location {
    pub fn new(plz: u32) -> Option<Self> {
        match plz {
            0..1000 => None,
            1000..9700 => Some(Location(plz)),
            _ => None,
        }
    }

    pub fn from_api_id(l: u32) -> Self {
        Location(l / 100)
    }

    pub fn get_api_id(self) -> u32 {
        self.0 * 100
    }

    pub async fn get_name(self) -> Result<String> {
        let cache = LOCATION_CACHE.read().await;

        if let Some(name) = cache.get(&self) {
            return Ok(name.clone());
        };

        drop(cache);

        let mut cache = LOCATION_CACHE.write().await;
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

        cache.insert(self, result.clone());

        if cache.len() > LOCATION_CACHE_LIMIT {
            cache.pop_first();
        }

        Ok(result)
    }
}
