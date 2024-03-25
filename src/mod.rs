use crate::provider::ModProvider;

trait RemoteMod<T>
where
    T: ModProvider,
{
    async fn fetch(&self) -> Result<Mod<T>, reqwest::Error>;
    fn get_client(&self) -> reqwest::Client;
}

pub struct Mod<T>
where
    T: ModProvider,
{
    name: String,
    description: String,
    version: String,
    author: String,
    id: String,

    provider: T,
}
