use reqwest::{Client, Method, Url, header::HeaderValue};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};

// const COOKIE_VALUE: &'static str = include_str!("cookie_value");
const COOKIE_VALUE: &'static str = include_str!("cookie_value.in");

pub async fn make_request() -> Result<String, Box<dyn std::error::Error>> {
    let mut req = reqwest::Request::new(
        Method::GET,
        // Url::parse("https://archiveofourown.org/users/FuzzyNovaGoblin/stats")?,
        Url::parse("https://archiveofourown.org/users/witch_of_history/stats")?,
    );
    req.headers_mut()
        .append("Cookie", HeaderValue::from_static(COOKIE_VALUE));

    let resp = Client::new().execute(req).await?;

    Ok(resp.text().await?)
}

pub async fn fake_request() -> Result<String, Box<dyn std::error::Error>> {
    let mut f = File::open("/home/fuzzy/Downloads/example.html")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    Ok(buf.into())
}
