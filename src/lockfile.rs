use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::provider::Limits;

#[derive(Debug, Serialize, Deserialize)]
struct ModLock {
    mod_id: i32,
    file_id: i32,

    url: String,
    sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockProvider<L = Limits>
where
    L: Into<Limits>,
{
    name: String,
    mods: std::vec::Vec<ModLock>,
    limits: L,
}

impl Default for LockProvider {
    fn default() -> Self {
        Self {
            name: "".into(),
            mods: [].into(),
            limits: Limits::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile<L = Limits>
where
    L: Into<Limits>,
{
    // time since epoch (in milliseconds)
    // when the lockfile was updated
    revision: usize,
    providers: HashMap<String, LockProvider<L>>,
}

impl Lockfile {
    pub fn new() -> Self {
        Self {
            revision: 0,
            providers: HashMap::from([("nexus".into(), LockProvider::default())]),
        }
    }

    pub fn set_provider(
        &mut self,
        provider_name: String,
        provider: LockProvider,
    ) -> Result<LockProvider, ()> {
        if let Some((k, _)) = &self.providers.get_key_value(&provider_name) {
            self.providers.insert(k.to_string(), provider).ok_or(())
        } else {
            Err(())
        }
    }

    pub fn get_provider(&self, provider_name: String) -> Option<&LockProvider> {
        self.providers.get(provider_name.as_str())
    }

    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        std::fs::read_to_string(path)
            .map(|what| serde_json::from_str(&what).expect("could not turn file into string"))
    }

    pub fn from_pwd() -> Option<Self> {
        // why did i interchange these? :thinking:"
        if let Ok(cwd) = std::env::current_dir() {
            let new_lock = cwd.join("/nmm.lock");
            match new_lock.try_exists() {
                Ok(exists) => {
                    if exists {
                        if let Ok(lockfile) = Self::from_file(&new_lock) {
                            Some(lockfile)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn write(&self, to: &Path) -> Result<(), std::io::Error> {
        std::fs::write(
            if to.is_dir() {
                to.join("./nmm.lock")
            } else {
                to.to_owned()
            },
            serde_json::to_string_pretty(self)?,
        )
    }
}
