use config::{Config, File};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Config::builder()
        .add_source(File::with_name("Setting.toml"))
        .build()?;
    let url = "https://www.yingyinwu.com/search-%E5%A4%A9%E4%B8%8B.htm";
    let threadlist = scraper::Selector::parse("ul.threadlist").unwrap();

    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;
    let content = resp.text().await?;
    let document = scraper::Html::parse_document(&content);
    let threadlist = scraper::Selector::parse("ul.threadlist").unwrap();
    let titles = document.select(&threadlist).map(|x| x.inner_html());

    titles.for_each(|item| println!("{}", item));

    Ok(())
}
