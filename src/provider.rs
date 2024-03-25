use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub trait ModProvider {
    fn fetch<T>(
        &self,
        endpoint: String,
        query_params: &std::collections::HashMap<String, String>,
    ) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned;
}
