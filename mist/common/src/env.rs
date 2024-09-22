use secstr::SecVec;
use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize)]
pub struct Environment {
    #[serde(deserialize_with = "string_to_secstr")]
    pub master_key: SecVec<u8>,
    pub authn_url: String,
    #[serde(default = "default_postgres_url")]
    pub postgres_url: String,
    #[serde(default = "default_postgres_pool_size")]
    pub postgres_pool_size: u32,
    #[serde(default = "default_redis_url")]
    pub redis_url: String,
    #[serde(default = "default_nats_url")]
    pub nats_url: String,
    #[serde(default = "default_resolver_url")]
    pub resolver_url: String,
    #[serde(default)]
    pub development: bool,
}

pub fn string_to_secstr<'de, D>(deserializer: D) -> Result<SecVec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(SecVec::from(s.into_bytes()))
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

fn default_nats_url() -> String {
    "nats://localhost:4222".into()
}

fn default_resolver_url() -> String {
    "http://localhost:9050/1.0/identifiers".into()
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            master_key: SecVec::new(vec![]),
            authn_url: Default::default(),
            postgres_url: Default::default(),
            postgres_pool_size: Default::default(),
            redis_url: Default::default(),
            nats_url: Default::default(),
            resolver_url: Default::default(),
            development: Default::default(),
        }
    }
}
