use std::{collections::HashMap, error};
pub async fn fetch(
    url: String,
    query: Option<HashMap<String, String>>,
) -> Result<reqwest::Response, Box<dyn error::Error>> {
    let response = reqwest::Client::new()
        .get(url)
        .query(&query.unwrap_or_default())
        .send()
        .await?;
    Ok(response)
}
