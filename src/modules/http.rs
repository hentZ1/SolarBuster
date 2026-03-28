use crate::prelude::*;
use std::time::Duration;

pub fn build_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .user_agent("SolarBuster")
        .pool_max_idle_per_host(100)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(5))
        .build()
}

pub async fn measure_noise(url: String, client: Client) -> Option<u64> {
    let fake_url = format!("{}{}", url, "THIS_IS_NOT_A_VALID_URL_ON_PURPOSE");

    let resp = client.get(&fake_url).send().await.ok()?;

    let bytes = resp.bytes().await.ok()?;
    Some(bytes.len() as u64)
}
