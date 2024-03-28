use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
};

pub fn check_store_url(path: PathBuf) -> bool {
    std::process::Command::new("nix store")
        .arg("ls")
        .arg(path)
        .output()
        .expect("failed to find store path")
        .status
        .success()
}

#[derive(Debug)]
pub struct Prefetched {
    pub sha: String,
    pub store_path: String,
}

impl Clone for Prefetched {
    fn clone(&self) -> Self {
        Self {
            sha: self.sha.clone(),
            store_path: self.store_path.clone(),
        }
    }
}

pub fn prefetch_url(url: String) -> Option<Prefetched> {
    let k = String::from_utf8(
        std::process::Command::new("nix-prefetch-url")
            .arg("--print-path")
            .arg(url.replace(" ", "%20"))
            .arg("--name")
            .arg("nmm-cli-result")
            .output()
            .expect("failed to add file to store")
            .stdout,
    )
    .expect("failed to convert vec<utf8> to string");
    let out = k.trim().split("\n").collect::<Vec<&str>>();
    if out.len() < 2 {
        None
    } else {
        Some(Prefetched {
            sha: out[0].to_string(),
            store_path: out[1].to_string(),
        })
    }
}
