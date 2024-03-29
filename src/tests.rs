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
        use crate::nix;
        use crate::provider::ModProvider;

        let file_id = 7364;
        let mod_id = 112;
        let game_domain_name = "monsterhunterworld";

        let apikey = std::env::var("NEXUS_API_KEY").unwrap_or("".into());
        println!("{apikey}");
        assert!(apikey != "");

        let provider =
            crate::nexus::NexusProvider::new(std::env::var("NEXUS_API_KEY").ok());

        let provider_request = provider.fetch::<TestResponse>(
            format!(
                "/v1/games/{}/mods/{}/files/{}/download_link.json",
                game_domain_name, mod_id, file_id
            ),
            &[].into(),
        );

        println!(
            "{:#?}",
            provider_request.map_or_else(
                |e| panic!("{:#?}", e),
                |f| nix::prefetch_url(f.data[1].URI.clone(), None)
            )
        );
    }

    fn test_lockfile() {
        use crate::lockfile;
    }
}
