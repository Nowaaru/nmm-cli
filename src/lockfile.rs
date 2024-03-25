use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Serialize)]
struct ModLock {
    mod_id: i32,
    file_id: i32,

    url: String,

    sha: String,
    md5: String,
}

#[derive(Debug, Serialize)]
struct LockProvider {
    name: String,
    mods: std::vec::Vec<ModLock>,
}

#[derive(Debug, Serialize)]
struct Lockfile {
    // time since epoch (in milliseconds)
    // when the lockfile was updated
    revision: usize,
    providers: HashMap<String, LockProvider>,
}

impl Lockfile {
    fn new() -> Self {
        Self {
            revision: 0,
            providers: [].into(),
        }
    }

    fn from_file() -> Self {
        todo!();
    }
}
