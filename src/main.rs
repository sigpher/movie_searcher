use config::{Config, File};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = url_get().await?;
    let jokes: Jokes = serde_json::from_str(&content)?;
    if jokes.code == 200 {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        let count = RE.captures(&jokes.msg).unwrap();
        println!("{} jokes found.", count.get(0).unwrap().as_str());
        for (index, joke) in jokes.data.iter().enumerate() {
            println!("{:02}:{}", index + 1, joke);
        }
    }
    Ok(())
}

async fn url_get() -> Result<String, Box<dyn Error>> {
    let settings = Config::builder()
        .add_source(File::with_name("Setting.toml"))
        .build()?;
    let url = format!(
        "{}{}",
        settings.get_string("web.base_url")?,
        settings.get_int("web.num")?
    );
    let client = reqwest::Client::new();
    let resp = client.get(&url).send().await?;
    let content = resp.text().await?;
    Ok(content)
}

#[derive(Debug, Serialize, Deserialize)]
struct Jokes {
    msg: String,
    code: u32,
    data: Vec<String>,
}
