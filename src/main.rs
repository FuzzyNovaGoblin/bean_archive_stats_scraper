use parse::tbd;
use request::fake_request;
use scraper::Selector;

mod config;
mod request;
mod parse;
mod bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let s = tbd(&fake_request().await?).await?;
    println!("{}", s);

    Ok(())
}
