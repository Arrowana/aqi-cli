extern crate serde;

// https://stackoverflow.com/questions/44836327/is-there-is-a-simpler-way-to-convert-a-type-upon-deserialization
pub fn string_as_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_str(F32Visitor).or(Ok(Default::default()))
}

struct F32Visitor;

impl<'de> serde::de::Visitor<'de> for F32Visitor {
    type Value = f32;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representation of a f32")
    }
    fn visit_str<E>(self, value: &str) -> Result<f32, E>
    where
        E: serde::de::Error,
    {
        value.parse::<f32>().or(Ok(0.0))
    }
}