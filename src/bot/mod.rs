use self::context::CacheAndHttp;
use crate::config::Context;
use serenity::all::{Client, GatewayIntents};
use std::sync::Arc;

pub mod context;
pub mod send_msgs;

pub async fn bot_start(config: crate::config::Config) -> (Arc<crate::config::Context>, Client) {
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(config.bot_token.clone(), intents)
        .await
        .expect("Error creating client");
    let dis_ctx = CacheAndHttp::new(client.http.clone(), client.cache.clone());

    // let get_reminders_cmd = CreateCommand::new("get_reminders").fun

    (
        Arc::new(Context {
            dis_ctx: Arc::new(dis_ctx),
            config,
        }),
        client,
    )
}
