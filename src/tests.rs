mod tests {
    use serde::{Deserialize, Serialize};

    #[cfg(test)]
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(transparent)]
    struct TestResponse {
        data: std::vec::Vec<TestResponseItem>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TestResponseItem {
        name: String,
        short_name: String,
        URI: String,
    }

    #[test]
    fn test_nexus() {
        use crate::provider::ModProvider;

        let file_id = 7364;
        let mod_id = 112;
        let game_domain_name = "monsterhunterworld";

        let apikey = std::env::var("NEXUS_API_KEY").unwrap_or("".into());
        println!("{apikey}");
        assert!(apikey != "");

        let provider =
            crate::nexus::NexusProvider::new(std::env::var("NEXUS_API_KEY").unwrap_or_default());
        let t = std::collections::HashMap::<String, String>::new();

        let provider_request = provider.fetch::<TestResponse>(
            format!(
                "/v1/games/{}/mods/{}/files/{}/download_link.json",
                game_domain_name, mod_id, file_id
            ),
            &t,
        );

        println!(
            "{:#?}",
            provider_request.map_or_else(
                |e| panic!("{:#?}", e),
                |f| {
                    let k = String::from_utf8(
                        std::process::Command::new("nix-prefetch-url")
                            .arg("--print-path")
                            .arg(f.data[1].URI.clone().replace(" ", "%20"))
                            .arg("--name")
                            .arg("nmm-cli-result")
                            .output()
                            .expect("failed to add file to store")
                            .stdout,
                    )
                    .expect("failed to convert vec!<utf8>to string");
                    k.trim().split("\n").collect::<Vec<&str>>()[1].to_string()
                }
            )
        );
    }
}
