#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct WeatherDataEntry {
    pub location: u32,
    pub location_type: u8,
    pub date: u64,
    pub value: f64,
}

