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

mod tests {
    

    
    

    #[test]
    fn test_nexus() {
        let file_id = 7364;
        let mod_id = 112;
        let game_domain_name = "monsterhunterworld";

        let apikey = std::env::var("NEXUS_API_KEY").unwrap_or("".into());
        println!("{apikey}");
        assert!(apikey != "");

        let provider =
            nexus::NexusProvider::new(std::env::var("NEXUS_API_KEY").unwrap_or_default());
        let t = HashMap::<String, String>::new();

        let provider_request = provider.fetch::<TestResponse>(
            format!(
                "/v1/games/{}/mods/{}/files/{}/download_link.json",
                game_domain_name, mod_id, file_id
            ),
            &t,
        );

        println!(
            "{:#?}",
            provider_request.map_or_else(|e| panic!("{:#?}", e), |f| f)
        );
    }
}