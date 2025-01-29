use self::destination::Destination;
use crate::{bot::context::DisCtx, scraper::datetime_parts::DateTimeParts};
use shared_singleton::*;

pub mod destination;
pub mod state;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub bot_token: String,
    pub url: String,
    pub destination: destination::Destination,
    pub cookie_value: String,
    pub time: DateTimeParts,
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
}
