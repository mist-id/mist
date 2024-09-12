use secstr::SecStr;
use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize)]
pub struct Environment {
    #[serde(deserialize_with = "string_to_secstr")]
    pub master_key: SecStr,
    pub authn_base_url: String,
    #[serde(default = "default_postgres_url")]
    pub postgres_url: String,
    #[serde(default = "default_postgres_pool_size")]
    pub postgres_pool_size: u32,
    #[serde(default = "default_redis_url")]
    pub redis_url: String,
    #[serde(default = "default_resolver_host")]
    pub resolver_host: String,
    #[serde(default)]
    pub development: bool,
}

pub fn string_to_secstr<'de, D>(deserializer: D) -> Result<SecStr, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(SecStr::new(s.into_bytes()))
}

fn default_postgres_url() -> String {
    "postgres://casper@localhost/mist".into()
}

fn default_postgres_pool_size() -> u32 {
    10
}

fn default_redis_url() -> String {
    "redis://localhost".into()
}

fn default_resolver_host() -> String {
    "http://localhost:9003/1.0/identifiers".into()
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            development: Default::default(),
            master_key: SecStr::new(vec![]),
            postgres_url: Default::default(),
            postgres_pool_size: Default::default(),
            redis_url: Default::default(),
            resolver_host: Default::default(),
            authn_base_url: Default::default(),
        }
    }
}
