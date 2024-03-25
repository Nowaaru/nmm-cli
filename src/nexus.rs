use std::collections::HashMap;

use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::provider::ModProvider;

struct Limits {
    limit: i32,
    remaining: i32,
    reset: usize,
}

struct NexusLimits {
    daily: Limits,
    hourly: Limits,
}

struct NexusServer {
    name: String,
    short_name: String,
    uri: String,
}

trait NexusEndpoints<T = i32, U = T> {
    fn make_download_link(
        &self,
        game_domain_name: String,
        id: T,
        mod_id: U,
    ) -> std::vec::Vec<NexusServer>
    where
        T: Into<i32>;
}

pub struct NexusProvider {
    api_base_url: String,
    api_key: String,

    client: reqwest::blocking::Client,
    limits: NexusLimits,
}

impl<T, U> NexusEndpoints<T, U> for NexusProvider {
    fn make_download_link(
        &self,
        game_domain_name: String,
        id: T,
        mod_id: U,
    ) -> std::vec::Vec<NexusServer>
    where
        T: Into<i32>,
    {
        // self.fetch(endpoint, query_params)
        todo!();
    }
}

impl NexusProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            limits: NexusLimits {
                daily: Limits {
                    limit: 2500,
                    remaining: 2500,
                    reset: 0,
                },
                hourly: Limits {
                    limit: 2500,
                    remaining: 2500,
                    reset: 0,
                },
            },

            api_key,
            api_base_url: "api.nexusmods.com".into(),
        }
    }
}

impl ModProvider for NexusProvider {
    fn fetch<T>(
        &self,
        endpoint: String,
        query_params: &std::collections::HashMap<String, String>,
    ) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let out_url = format!("https://{}{}", &self.api_base_url, endpoint);
        println!("{}", out_url);
        let http_item = self
            .client
            .request(Method::GET, out_url)
            .header("apikey", &self.api_key)
            .query(query_params)
            .build()?;

        let response = self.client.execute(http_item)?;
        match response.error_for_status() {
            Ok(res) => res.json::<T>(),

            Err(a) => Err(a),
        }
    }
}
