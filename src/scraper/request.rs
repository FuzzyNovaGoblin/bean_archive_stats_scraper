use crate::config::Config;
use reqwest::{Client, Method, Url, header::HeaderValue};
use std::fs::File;
use std::io::Read;

pub async fn make_request(config: Config) -> Result<String, Box<dyn std::error::Error>> {
    let mut req = reqwest::Request::new(
        Method::GET,
        // Url::parse("https://archiveofourown.org/users/FuzzyNovaGoblin/stats")?,
        Url::parse("https://archiveofourown.org/users/witch_of_history/stats")?,
    );
    req.headers_mut().append("Cookie", HeaderValue::from_str(&config.cookie_value)?);

    let resp = Client::new().execute(req).await?;

    Ok(resp.text().await?)
}

pub async fn _fake_request() -> Result<String, Box<dyn std::error::Error>> {
    let mut f = File::open("/home/fuzzy/Downloads/example.html")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf.into())
}
