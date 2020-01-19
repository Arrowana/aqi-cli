extern crate serde;

use super::parsing;

#[derive(serde::Deserialize)]
pub struct SensorData {
    pub location: Location,
    pub sensordatavalues: Vec<SensorDataValue>,
}

#[derive(Clone, serde::Deserialize)]
pub struct SensorDataValue {
    #[serde(deserialize_with = "parsing::string_as_f32")]
    pub value: f32,
    pub value_type: String,
}

#[derive(serde::Deserialize)]
pub struct Location {
    #[serde(deserialize_with = "parsing::string_as_f32")]
    pub latitude: f32,
    #[serde(deserialize_with = "parsing::string_as_f32")]
    pub longitude: f32,
}

#[derive(serde::Deserialize)]
pub struct FreeGeoIPLocation {
    pub latitude: f32,
    pub longitude: f32,
}