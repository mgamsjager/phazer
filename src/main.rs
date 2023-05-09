use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Once};
use std::thread;

use clap::Parser;
use rss::Channel;

use cli::{Args, SETTINGS};
use config::read_config;
use phazer::{create_feed_tx_thread, differentiator, output_feeder_item, CustomFeederItem};

mod cli;
mod config;

pub static ONCE: Once = Once::new();

fn main() {
    let args = Args::parse();
    unsafe {
        ONCE.call_once(|| {
            SETTINGS.interval = args.interval;
        });
    }

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
