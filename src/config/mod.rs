use self::destination::Destination;
use crate::bot::context::DisCtx;
use shared_singleton::*;

pub mod destination;
pub mod state;
pub mod vault;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub bot_token: String,
    pub destination: destination::Destination,
    pub cookie_value: String,
}

impl_singleton_arc!(Config, Config::load());

impl Config {
    pub fn load() -> Config {
        let data = std::fs::read_to_string("/etc/bean_archive_stats_scraper/config.toml")
            .expect("failed to read config at '/etc/bean_archive_stats_scraper/config.toml'");

        match toml::from_str(&data) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "error in config file '/etc/bean_archive_stats_scraper/config.toml'\n{}",
                    e
                );
                panic!()
            }
        }
    }
}

#[derive(Debug)]
pub struct Context {
    pub dis_ctx: DisCtx,
    pub config: Config,
}

impl Context {
    pub fn get_error_ch(&self) -> &Destination {
        &self.config.destination
    }

    pub fn get_dest(&self) -> &Destination {
        &self.config.destination
    }
}
