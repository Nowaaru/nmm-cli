use std::path::{Path, PathBuf};

pub fn check_store_url(path: PathBuf) -> bool {
    std::process::Command::new("nix store")
        .arg("ls")
        .arg(path)
        .output()
        .expect("failed to add file to store")
        .status
        .success()
}

pub fn prefetch_url(url: String) -> String {
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
    .expect("failed to convert vec!<utf8>to string");
    k.trim().split("\n").collect::<Vec<&str>>()[1].to_string()
}
