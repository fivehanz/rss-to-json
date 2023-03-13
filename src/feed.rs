use std::{error::Error, result::Result};
use rss::Channel;
use quickxml_to_serde::{xml_string_to_json, Config};


pub async fn feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    
    Ok(channel)
}

pub async fn feed_json_string(url: &str) -> String {
    let feed = feed(url).await;
    
    xml_string_to_json(
        feed.unwrap().to_string(), 
        &Config::new_with_defaults()
    )
    .unwrap()
    .to_string()
}