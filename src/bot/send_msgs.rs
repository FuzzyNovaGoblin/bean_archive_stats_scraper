use crate::{Ctx, config::destination::Destination};
use serenity::builder::{CreateAttachment, CreateEmbed};
use std::convert::Into;

pub async fn send_msg(
    ctx: crate::Ctx,
    msg: impl Into<String>,
    files: Option<Vec<CreateAttachment>>,
    embeds: Option<Vec<CreateEmbed>>,
    dest: &Destination,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = dest.id(ctx.clone());

    let builder = serenity::builder::CreateMessage::new().content(msg);

    let builder = match embeds.clone() {
        Some(embeds) => builder.embeds(embeds),
        None => builder,
    };

    let builder = match files.clone() {
        Some(files) => builder.files(files),
        None => builder,
    };
    id.await.send_message(ctx.dis_ctx.clone(), builder).await?;

    Ok(())
}


pub async fn report_rust_error(ctx: Ctx, err_msg: String) {
    eprintln!("err msg: {}", err_msg);
    send_msg(ctx.clone(), err_msg, None, None, ctx.get_error_ch())
        .await
        .unwrap();
}
