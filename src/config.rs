use anyhow::anyhow;
use shuttle_secrets::SecretStore;

#[derive(Clone)]
pub struct Config {
    pub secret_1: String,
    pub secret_2: String,
    pub not_a_secret: String,
}

impl TryFrom<SecretStore> for Config {
    type Error = anyhow::Error;

    fn try_from(value: SecretStore) -> Result<Self, Self::Error> {
        fn get_secret(secret_store: &SecretStore, key: &str) -> Result<String, anyhow::Error> {
            secret_store
                .get(key)
                .ok_or_else(|| anyhow!("Missing secret {}", key))
        }
        Ok(Config {
            secret_1: get_secret(&value, "SECRET_1")?,
            secret_2: get_secret(&value, "SECRET_2")?,
            not_a_secret: get_secret(&value, "NOT_A_SECRET")?,
        })
    }
}
