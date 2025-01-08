use super::{datetime_parts::DateTimeParts, parse::build_notification_msg};
use crate::{Ctx, config::Config, scraper::request::make_request};
use chrono::Local;
use serenity::all::CreateAttachment;
use std::{thread::sleep, time::Duration};

pub async fn timer(
    ctx: Ctx,
    config: &Config,
    target_time: DateTimeParts,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let target_time = target_time
            .get_target_time()
            .single()
            .ok_or("no single time")?;
        dbg!("timer loop start");

        let dif = target_time.signed_duration_since(Local::now());
        dbg!(&dif);
        if dif.abs() == dif {
            sleep(dif.to_std().unwrap());
        } else if dif.abs().to_std().unwrap() >= Duration::from_secs(60) {
            sleep(Duration::from_hours(1));
            continue;
        }
        dbg!("starting");
        crate::bot::send_msgs::send_msg(
            ctx.clone(),
            "Starting scrape now",
            None,
            None,
            &config.destination,
        )
        .await?;

        dbg!("fetching");
        let msg = build_notification_msg(&make_request(config.clone()).await?).await?;

        crate::bot::send_msgs::send_msg(
            ctx.clone(),
            Local::now().to_string(),
            Some(vec![CreateAttachment::bytes(
                msg.bytes().collect::<Vec<u8>>(),
                &format!("{}.txt", Local::now().to_string()),
            )]),
            None,
            &config.destination,
        )
        .await?;
        sleep(Duration::from_secs(60));
    }
}
