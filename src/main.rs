#![feature(duration_constructors)]

use bot::bot_start;
use config::Config;
use scraper::timer;
use std::{sync::Arc, thread::sleep, time::Duration};
use tokio::spawn;

mod bot;
mod config;
mod scraper {
    pub mod parse;
    pub mod request;
    pub mod timer;
    pub mod datetime_parts;
}

type Ctx = Arc<config::Context>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load();
    let (ctx, mut client) = bot_start(config.clone()).await;
    let _client_handle = spawn(async move { client.start().await });
    sleep(Duration::from_secs(1));

    // let s = build_notification_msg(&fake_request().await?).await?;
    // println!("{}", s);



    loop {
        if let Err(e) =
            timer::timer(ctx.clone(), &config, scraper::datetime_parts::DateTimeParts::new(23, 30)).await
        {
            spawn(bot::send_msgs::report_rust_error(
                ctx.clone(),
                e.to_string(),
            ));
        }
    }
}
