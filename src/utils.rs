use std::{collections::HashMap, error};
use serde::de;

pub async fn fetch(
    url: String,
    query: Option<HashMap<&str, &str>>,
) -> Result<reqwest::Response, Box<dyn error::Error>> {
    let response = reqwest::Client::new()
        .get(url)
        .query(&query.unwrap_or_default())
        .send()
        .await?;
    Ok(response)
}

pub async fn deserialize<T: de::DeserializeOwned>(response: reqwest::Response) -> Result<T, Box<(dyn error::Error)>> {
    Ok(response.json::<T>().await?)
}
