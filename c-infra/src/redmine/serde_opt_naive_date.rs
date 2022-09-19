use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serializer};

const DATE_FORMATTER: &str = "%Y-%m-%d";

pub fn serialize<S>(value: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *value {
        Some(ref value) => serializer.serialize_str(&value.format(DATE_FORMATTER).to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    Ok(match s {
        Some(s) => {
            Some(NaiveDate::parse_from_str(&s, DATE_FORMATTER).map_err(serde::de::Error::custom)?)
        }
        None => None,
    })
}
