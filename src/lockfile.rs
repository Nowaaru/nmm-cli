use std::{collections::HashMap, error::Error, path::Path};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::provider::Limits;

#[derive(Debug, Serialize, Deserialize)]
struct ModLock {
    mod_id: i32,
    file_id: i32,

    url: String,

    sha: String,
    md5: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockProvider<L>
where
    L: Into<Limits>,
{
    name: String,
    mods: std::vec::Vec<ModLock>,
    limits: L,
}

#[derive(Debug, Serialize, Deserialize)]
struct Lockfile<L = Limits>
where
    L: Into<Limits>,
{
    // time since epoch (in milliseconds)
    // when the lockfile was updated
    revision: usize,
    providers: HashMap<String, LockProvider<L>>,
}

impl Lockfile {
    fn new() -> Self {
        Self {
            revision: 0,
            providers: [].into(),
        }
    }

    fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        std::fs::read_to_string(path)
            .map(|what| serde_json::from_str(&what).expect("could not turn file into string"))
    }
}
