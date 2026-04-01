#[derive(Debug, serde::Deserialize, Clone, Copy)]
pub struct WeatherDataEntry {
    pub location: u32,
    pub location_type: u8,
    pub date: u64,
    pub value: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Location {
    Plz(u16),
}

impl Into<u32> for Location {
    fn into(self) -> u32 {
        match self {
            Location::Plz(plz) => plz as u32 * 100,
        }
    }
}
