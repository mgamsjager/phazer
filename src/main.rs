use ansi_term::Style;
use rss::Channel;
use std::error::Error;
use std::io::BufReader;
use std::{thread, time};

async fn get_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://feeds.nos.nl/nosnieuwsalgemeen")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        std::process::Command::new("clear").status().unwrap();
        if let Ok(c) = get_feed().await {
            for i in c.items {
                println!(
                    "{0} \t {1} ",
                    Style::new().bold().paint(i.pub_date.unwrap()),
                    Style::new().bold().paint(i.title.unwrap()),
                    // i.description.unwrap().as_str(),
                    // i.link.unwrap(),
                )
            }
        }
        println!("\nLast run: {}", chrono::offset::Local::now().to_rfc2822());
        thread::sleep(time::Duration::from_secs(60 * 10));
    }
}
