use serde::de;
use core::time::Duration;

pub async fn parse_json_from_uri<T>(uri: &str) -> Result<T, String>
where T: de::DeserializeOwned {
    let text
        =reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build().unwrap()
        .get(uri)
        .send().await.map_err(|er| { er.to_string() })?
        .error_for_status().map_err(|er| { er.to_string() })?
        .text().await.map_err(|er| { er.to_string() })?;

    serde_json::from_str::<T>(text.as_str()).map_err(|er| { er.to_string() })
}