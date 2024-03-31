use crate::lockfile;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Display;

use crate::lockfile::ModLock;
pub trait ModProvider {
    fn fetch<T>(
        &self,
        endpoint: String,
        query_params: &std::collections::HashMap<String, String>,
    ) -> Result<T, reqwest::Error>
    where
        T: DeserializeOwned;

    fn download(
        &self,
        game_id: std::string::String,
        mod_id: i32,
        file_id: i32,
        lockfile: &mut lockfile::Lockfile,
    ) -> Result<(), ()>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limits {
    pub limit: i32,
    pub remaining: i32,
    pub reset: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            limit: 2500,
            remaining: 2500,
            reset: usize::MAX,
        }
    }
}
