use crate::utils::config::read_config;
use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;
use url::Url;

pub struct APIClient {
    pub(crate) api_key: String,
    pub(crate) client: Client,
}

#[async_trait]
pub trait Get {
    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T>;
}

impl APIClient {
    pub fn new() -> Self {
        let config = read_config();

        Self {
            api_key: config.api_key.unwrap_or("".to_string()),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Get for APIClient {
    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        let url = Url::parse(&url)?;

        let url = Url::parse_with_params(url.as_str(), &[("appid", &self.api_key)])?;

        let response = self.client.get(url).send().await?;

        // if response.status() == reqwest::StatusCode::UNAUTHORIZED {
        //     eprintln!("Wrong API key");
        //     return response.error_for_status().map_err(|e| e.into())
        // }
        //
        // if response.status() != reqwest::StatusCode::OK {
        //     return Err(Box::new(response.error_for_status().unwrap_err()));
        // }

        let data = response.json::<T>().await?;

        Ok(data)
    }
}
