use rss::Channel;
use simple_error::SimpleError;
use std::error::Error;
use std::io::{BufRead, BufReader};

async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("http://feeds.feedburner.com/tweakers/nieuws")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(c) = example_feed().await {
        for i in c.items {
            println!(
                "Date: {0} \t {1} \n\t {2}\n {3}",
                i.pub_date.unwrap(),
                i.title.unwrap(),
                i.description.unwrap(),
                i.link.unwrap(),
            )
        }
    }
    Ok(())
}
