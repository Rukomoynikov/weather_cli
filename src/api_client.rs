use crate::utils::config::read_config_value;
use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::error::Error;
use url::Url;

pub struct APIClient {
    pub(crate) api_key: String,
    pub(crate) client: Client,
}

#[async_trait]
pub trait Get {
    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T, Box<dyn Error>>;
}

impl APIClient {
    pub fn new() -> Self {
        Self {
            api_key: read_config_value("api_key"),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Get for APIClient {
    async fn get<T: DeserializeOwned>(&self, url: String) -> Result<T, Box<dyn Error>> {
        let url = match Url::parse(&url) {
            Ok(url) => url,
            Err(err) => {
                return Err(Box::new(err));
            }
        };

        let url = Url::parse_with_params(url.as_str(), &[("appid", &self.api_key)])?;

        let response = self.client.get(url).send().await?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            eprintln!("Wrong API key");
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        if response.status() != reqwest::StatusCode::OK {
            return Err(Box::new(response.error_for_status().unwrap_err()));
        }

        let data = response.json::<T>().await?;

        Ok(data)
    }
}
