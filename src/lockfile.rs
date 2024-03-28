use std::{
    collections::HashMap,
    path::{self, Path, PathBuf},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::provider::Limits;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModLock<S: Into<String> + Clone = String> {
    mod_id: i32,
    file_id: i32,

    pub sha: S,
    pub store_path: S,
}

impl<S> ModLock<S>
where
    S: Into<String> + Clone,
{
    pub fn new(mod_id: i32, file_id: i32, sha: S, store_path: S) -> Self {
        Self {
            file_id,
            mod_id,
            sha,
            store_path,
        }
    }
}

impl Clone for ModLock {
    fn clone(&self) -> Self {
        return Self {
            mod_id: self.mod_id,
            file_id: self.file_id,

            sha: self.sha.clone(),
            store_path: self.store_path.clone(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockProvider<L = Limits>
where
    L: Into<Limits>,
{
    pub name: String,
    /*
       {
        mod-id = {
            file-id = {
                mod_id
                file_id

                url
                sha
            }
        }
       }
    */
    pub mods: HashMap<String, HashMap<String, ModLock>>,
    pub limits: L,
}
impl<L> LockProvider<L>
where
    L: Into<Limits>,
{
    pub fn insert_modlock(&mut self, modlock: ModLock) -> Result<(), ()> {
        let mod_id = modlock.mod_id.to_string();
        let file_id = modlock.file_id.to_string();

        let mod_files = self
            .mods
            .entry(mod_id)
            .or_insert(HashMap::new())
            .entry(file_id)
            .or_insert(modlock.clone());

        *mod_files = modlock;

        println!("fuck me man: {:#?}", self.mods);
        Ok(())
    }

    pub fn remove_modlock_by_fileid<S: Into<String> + Clone>(
        &mut self,
        modid: S,
        fileid: S,
    ) -> Result<(), ()> {
        let files = self.mods.get_mut(&modid.into()).unwrap();
        let mut item_to_remove: String = "".into();
        // for (fileid, ModLock { sha, .. }) in files {
        //     if hash.clone().into() == *sha {
        //         item_to_remove = fileid.clone();
        //     }
        // }

        files.remove(&fileid.into()).ok_or(()).map(|_| ())
    }
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

    pub fn add_mod<S: Into<String> + Clone>(
        &mut self,
        provider_name: S,
        modlock: ModLock,
    ) -> Result<(), ()> {
        self.providers
            .entry(provider_name.into())
            .or_default()
            .insert_modlock(modlock)
    }

    pub fn remove_fileid<S: Into<String> + Clone>(
        &mut self,
        modid: S,
        fileid: S,
    ) -> Result<(), ()> {
        self.providers
            .get_mut("nexus")
            .ok_or(())?
            .remove_modlock_by_fileid(modid, fileid)
    }

    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        std::fs::read_to_string(path).map(|what| {
            serde_json::from_str(&what)
                .expect(&format!("could not turn file into string:\n{}", &what))
        })
    }

    pub fn from_cwd() -> Option<Self> {
        // why did i interchange these? :thinking:"
        if let Ok(cwd) = std::env::current_dir() {
            let new_lock = cwd.join("./nmm.lock");
            // println!("test: {:?}", new_lock);
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

    pub fn write(&self, to: Option<&Path>) -> Result<(), std::io::Error> {
        let current_dir = std::env::current_dir().expect("current directory was not found");
        let path = to.unwrap_or_else(|| &current_dir);
        let to_write = serde_json::to_string_pretty(self)?;

        println!("{}", to_write);

        std::fs::write(
            if path.is_dir() {
                path.join("./nmm.lock")
            } else {
                path.to_owned()
            },
            to_write,
        )
    }
}
