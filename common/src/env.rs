use secstr::SecStr;
use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize)]
pub struct Environment {
    pub development: bool,
    #[serde(deserialize_with = "string_to_secstr")]
    pub master_key: SecStr,
    pub postgres_url: String,
    pub postgres_pool_size: u32,
    pub redis_url: String,
    pub resolver_host: String,
    pub authn_base_url: String,
}

pub fn string_to_secstr<'de, D>(deserializer: D) -> Result<SecStr, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Ok(SecStr::new(s.into_bytes()))
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
