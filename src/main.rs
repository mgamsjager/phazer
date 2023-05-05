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

fn main() {
    let (tx, rx): (Sender<Channel>, Receiver<Channel>) = mpsc::channel();
    let (item_tx, item_rx): (Sender<Item>, Receiver<Item>) = mpsc::channel();
    let feeds = vec![
        "http://feeds.feedburner.com/tweakers/nieuws",
        "https://feeds.nos.nl/nosnieuwsalgemeen",
        "https://fd.nl/?rss",
        "https://fd.nl/beurs?rss",
    ];

    for feed in feeds {
        create_feed_tx_thread(tx.clone(), feed);
    }

    // Diff thread.
    thread::spawn(move || {
        let mut cached_items: HashMap<String, Item> = HashMap::new();
        for channel in rx {
            let mut sorted_items = channel.items.clone();
            sorted_items.sort_by(sort);

            for item in &sorted_items[sorted_items.len() - 5..] {
                if let Some(guid) = item.guid.as_ref() {
                    let key = &guid.value;
                    if let Vacant(entry) = cached_items.entry(String::from(key)) {
                        entry.insert(item.to_owned());
                        item_tx.clone().send(item.to_owned()).unwrap();
                    }
                } else {
                    eprintln!("No Guid found for item {}", item.link.to_owned().unwrap());
                }
            }
        }
    });

    std::process::Command::new("clear").status().unwrap();

    for item in item_rx {
        let date = &item.pub_date.as_ref().unwrap();
        let parsed_date = DateTime::parse_from_rfc2822(date)
            .unwrap_or_default()
            .format("%d-%m-%Y %H:%M");
        println!(
            "{date} | {gui} | {title} |",
            date = Style::new().bold().paint(parsed_date.to_string()),
            gui = item
                .guid
                .as_ref()
                .unwrap()
                .value
                .as_str()
                .get(..20)
                .unwrap_or_default(),
            title = Style::new().bold().paint(item.title.as_ref().unwrap())
        );
    }
}
