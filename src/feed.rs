use quickxml_to_serde::{xml_string_to_json, Config};
use rss::Channel;
use std::{error::Error, result::Result};

/// Fetches the content from the given URL and returns the parsed RSS channel.
///
/// # Arguments
///
/// * `url` - The URL of the RSS feed.
///
/// # Returns
///
/// The parsed `Channel` struct representing the RSS channel.
pub async fn feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    // Fetch the content from the URL
    let content = reqwest::get(url).await?.bytes().await?;

    // Parse the content into a Channel struct
    let channel: Channel = Channel::read_from(&content[..])?;

    Ok(channel)
}

/// Fetches the XML feed from the specified URL and converts it to a JSON string.
///
/// # Arguments
///
/// * `url` - The URL of the XML feed.
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - The JSON string representation of the XML feed, or an error if the conversion fails.
pub async fn feed_json_string(url: &str) -> Result<String, Box<dyn Error>> {
    // Fetch the XML feed from the specified URL
    let feed: Channel = feed(url).await?;

    // Convert the XML feed to a JSON string
    let config: Config = Config::new_with_defaults();
    let json_string: serde_json::Value = xml_string_to_json(feed.to_string(), &config)?;

    Ok(json_string.to_string())
}
