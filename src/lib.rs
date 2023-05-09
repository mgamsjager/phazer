use ansi_term::Style;
use chrono::DateTime;
use rss::{Channel, Item};
use std::cmp::Ordering;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

mod cli;
use cli::SETTINGS;

pub mod custom_feeder_item;
pub use custom_feeder_item::CustomFeederItem;

pub fn get_feed(feed_url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::blocking::get(feed_url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub fn create_feed_tx_thread(tx: Sender<Channel>, feed_url: &str) {
    let string = String::from(feed_url);

    thread::spawn(move || loop {
        if let Ok(c) = get_feed(&string) {
            tx.send(c).expect("Failed to put on transmitter");

            unsafe {
                thread::sleep(time::Duration::from_secs(60 * SETTINGS.interval));
            };
        }
    });
}
pub fn sort(a: &Item, b: &Item) -> Ordering {
    let aa = DateTime::parse_from_rfc2822(a.pub_date.as_ref().unwrap()).unwrap_or_default();
    let bb = DateTime::parse_from_rfc2822(b.pub_date.as_ref().unwrap()).unwrap_or_default();
    aa.cmp(&bb)
}

pub fn output_feeder_item(cf_item: &CustomFeederItem) {
    println!(
        "{date} | {owner: <15} | {title}",
        date = Style::new().bold().paint(cf_item.get_pub_date()),
        owner = cf_item.channel_owner,
        title = Style::new()
            .bold()
            .paint(cf_item.item.title.as_ref().unwrap())
    );
}
pub fn differentiator(rx: Receiver<Channel>, item_tx: Sender<CustomFeederItem>) {
    let mut cached_items: HashMap<String, Item> = HashMap::with_capacity(10);
    let mut heap = BinaryHeap::new();

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
                    heap.push(cf_item.to_owned());
                    item_tx
                        .clone()
                        .send(cf_item)
                        .expect("Unable to send feeder item to channel");
                }
            } else {
                eprintln!("No Guid found for item {}", item.link.to_owned().unwrap());
            }
        }
        let max_cache_size: usize;
        unsafe { max_cache_size = SETTINGS.max_cache_size }

        while cached_items.len() > max_cache_size {
            if let Some(item) = heap.pop() {
                println!(
                    "Removing item {} with pub date {}",
                    item.get_guid(),
                    item.get_pub_date()
                );
                cached_items.remove(item.get_guid());
            }
        }
    }
}