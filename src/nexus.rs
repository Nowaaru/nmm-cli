use crate::provider::{Limits, ModProvider};
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct NexusLimits {
    daily: Limits,
    hourly: Limits,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct NexusServer {
    pub name: String,
    pub short_name: String,
    pub URI: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct NexusDownloadEndpoints<T = NexusServer> {
    data: std::vec::Vec<T>,
}

impl<T> Into<std::vec::Vec<T>> for NexusDownloadEndpoints<T> {
    fn into(self) -> std::vec::Vec<T> {
        self.data
    }
}

impl NexusDownloadEndpoints<NexusServer> {
    pub fn get_server<S: Into<String> + Copy>(&self, server_name: S) -> Option<String> {
        let servers = &self.data;

        for NexusServer {
            short_name, URI, ..
        } in servers
        {
            if (short_name.contains(&server_name.into())) {
                return Some(URI.to_owned());
            }
        }
        None
    }

    pub fn get_server_or<S: Into<String> + Copy, F: Fn(&std::vec::Vec<NexusServer>) -> String>(
        &self,
        server_name: S,
        closure: F,
    ) -> String {
        if let Some(url) = self.get_server(server_name) {
            return url;
        } else {
            return closure(&self.data);
        }
    }
}

pub trait NexusEndpoints<T = i32, U = T> {
    fn make_download_links(
        &self,
        game_domain_name: String,
        mod_id: T,
        file_id: U,
    ) -> Result<NexusDownloadEndpoints, ()>
    where
        T: Into<i32> + std::fmt::Display,
        U: Into<i32> + std::fmt::Display;
}

pub struct NexusProvider {
    api_base_url: String,
    api_key: String,

    client: reqwest::blocking::Client,
    limits: NexusLimits,
}

impl<T, U> NexusEndpoints<T, U> for NexusProvider
where
    T: Into<i32> + std::fmt::Display,
    U: Into<i32> + std::fmt::Display,
{
    fn make_download_links(
        &self,
        game_domain_name: String,
        mod_id: T,
        file_id: U,
    ) -> Result<NexusDownloadEndpoints, ()> {
        // self.fetch(endpoint, query_params)
        let provider_request = self.fetch::<NexusDownloadEndpoints>(
            format!(
                "/v1/games/{}/mods/{}/files/{}/download_link.json",
                game_domain_name, mod_id, file_id
            ),
            &[].into(),
        );

        let as_vec = provider_request.map_err(|e| panic!("{:#?}", e));

        println!("{:#?}", as_vec);
        return as_vec;
    }
}

impl NexusProvider {
    pub fn new(api_key: Option<String>) -> Self {
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

            api_key: api_key.map_or(std::env::var("NEXUS_API_KEY").unwrap_or("".into()), |v| v),
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
        let http_item = self
            .client
            .request(Method::GET, out_url)
            .header("apikey", &self.api_key)
            .query(query_params)
            .build()?;

        let response = self.client.execute(http_item)?;
        response.error_for_status().map(|res| res.json::<T>())? // what
    }
}
