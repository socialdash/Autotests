use std::env;

use config_crate::{Config as RawConfig, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub gateway_microservice: GatewayMicroservice,
    pub users_microservice: Microservice,
    pub stores_microservice: Microservice,
    pub saga_microservice: SagaMicroservice,
    pub orders_microservice: Microservice,
    pub billing_microservice: Microservice,
    pub warehouses_microservice: Microservice,
    pub notifications_microservice: Microservice,
    pub delivery_microservice: Microservice,
    pub test_environment: Option<RunMode>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Microservice {
    pub database_url: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayMicroservice {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SagaMicroservice {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RunMode {
    Local {
        graphql_url: String,
    },
    Cluster {
        graphql_url: String,
        test_tools_url: String,
    },
}

impl Config {
    /// Creates config from base.toml, which are overwritten by <env>.toml, where env is one of dev,
    /// k8s, nightly. After that it could be overwritten by env variables like STQ_FUNCTIONAL_TESTS
    /// (this will override `url` field in config).
    pub fn new() -> Result<Self, ConfigError> {
        // Optional file specific for environment
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        Config::with_env(env)
    }

    pub fn with_env(env: impl Into<String>) -> Result<Self, ConfigError> {
        let mut s = RawConfig::new();

        s.merge(File::with_name("config/base"))?;
        s.merge(File::with_name(&format!("config/{}", env.into())).required(false))?;
        s.merge(Environment::with_prefix("STQ_FUNCTIONAL_TESTS"))?;
        s.try_into()
    }
}
