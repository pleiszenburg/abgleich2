use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ZpoolCfg {

    pub root: String,
    pub host: String,

}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {

    pub source: ZpoolCfg,
    pub target: ZpoolCfg,

    pub include_root: bool,
    pub keep_snapshots: u64,
    pub keep_backlog: bool,
    pub always_changed: bool,
    pub written_threshold: Option<u64>,
    pub check_diff: bool,
    pub suffix: String,
    pub digits: u64,

    pub ignore: Vec<String>,

}

impl Settings {

    pub fn from_configfile(path: &str) -> Self {

        let raw_attempt = Config::builder()
            // .add_source(File::with_name("default.yaml"))
            .add_source(File::with_name(path).required(false))
            .add_source(Environment::with_prefix("ABGLEICH"))
            .build();

        match raw_attempt {
            Ok(raw) => {
                let config_attempt: Result<_, ConfigError> = raw.try_deserialize();
                match config_attempt {
                    Ok(config) => {
                        config
                    }
                    Err(error) => {
                        panic!("failed to deserialize config\n{}", error)
                    }
                }
            },
            Err(error) => {
                panic!("failed to build config\n{}", error)
            }
        }

    }

}
