use ansi_term::Style;
use chrono::DateTime;
use rss::{Channel, Item};
use std::cmp::Ordering;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

mod config;
use config::read_config;

#[derive(Debug, PartialEq, Clone)]
struct CustomFeederItem {
    channel_owner: String,
    item: Item,
}

fn get_feed(feed_url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::blocking::get(feed_url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn create_feed_tx_thread(tx: Sender<Channel>, feed_url: &str) {
    let string = String::from(feed_url);

    thread::spawn(move || loop {
        if let Ok(c) = get_feed(&string) {
            tx.send(c).expect("Failed to put on transmitter");
            thread::sleep(time::Duration::from_secs(60 * 1));
        }
    });
}
fn sort(a: &Item, b: &Item) -> Ordering {
    let aa = DateTime::parse_from_rfc2822(a.pub_date.as_ref().unwrap()).unwrap_or_default();
    let bb = DateTime::parse_from_rfc2822(b.pub_date.as_ref().unwrap()).unwrap_or_default();
    aa.cmp(&bb)
}

fn output_feeder_item(cf_item: &CustomFeederItem) {
    let date = cf_item.item.pub_date.as_ref().unwrap();
    let parsed_date = DateTime::parse_from_rfc2822(date)
        .unwrap_or_default()
        .format("%d-%m-%Y %H:%M");

    println!(
        "{date} | {owner: <15} | {title}",
        date = Style::new().bold().paint(parsed_date.to_string()),
        owner = cf_item.channel_owner,
        title = Style::new()
            .bold()
            .paint(cf_item.item.title.as_ref().unwrap())
    );
}

fn differentiator(rx: Receiver<Channel>, item_tx: Sender<CustomFeederItem>) {
    let mut cached_items: HashMap<String, Item> = HashMap::new();
    for channel in rx {
        let mut sorted_items = channel.items.clone();
        sorted_items.sort_by(sort);

        for item in &sorted_items[sorted_items.len() - 5..] {
            if let Some(guid) = item.guid.as_ref() {
                let key = &guid.value;
                if let Vacant(entry) = cached_items.entry(String::from(key)) {
                    entry.insert(item.to_owned());
                    let cf_item = CustomFeederItem {
                        channel_owner: channel.title.to_owned(),
                        item: item.clone(),
                    };
                    item_tx
                        .clone()
                        .send(cf_item)
                        .expect("Unable to send feeder item to channel");
                }
            } else {
                eprintln!("No Guid found for item {}", item.link.to_owned().unwrap());
            }
        }
    }
}

fn main() {
    let (tx, rx): (Sender<Channel>, Receiver<Channel>) = mpsc::channel();
    let (item_tx, item_rx): (Sender<CustomFeederItem>, Receiver<CustomFeederItem>) =
        mpsc::channel();

    let config = read_config("feeds.toml");

    let feeds = match config {
        Ok(c) => c.feeds.to_vec(),
        Err(e) => {
            eprintln!("Error parsing config file, {}", e);
            std::process::exit(1);
        }
    };

    for feed in feeds {
        let feed: &str = match feed.as_str() {
            None => {
                eprintln!("Invalid feed url");
                std::process::exit(1);
            }
            Some(f) => f,
        };
        create_feed_tx_thread(tx.clone(), feed);
    }

    thread::spawn(move || differentiator(rx, item_tx));

    std::process::Command::new("clear").status().unwrap();

    for c_f_item in item_rx {
        output_feeder_item(&c_f_item);
    }
}
