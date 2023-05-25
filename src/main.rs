use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Once};
use std::time::SystemTime;
use std::{env, fs, thread, time};

use clap::Parser;
use rss::Channel;

use cli::{Args, SETTINGS};
use config::read_config;
use phazer::{create_feed_tx_thread, differentiator, output_feeder_item, CustomFeederItem};

mod cli;
mod config;

pub static ONCE: Once = Once::new();

const CONFIG_RELOAD_SLEEP: u64 = 2;

fn main() {
    let args = Args::parse();
    unsafe {
        ONCE.call_once(|| {
            SETTINGS.interval = args.interval;
        });
    }

    let mut list_of_feeds = vec![];

    let (tx, rx): (Sender<Channel>, Receiver<Channel>) = mpsc::channel();
    let (item_tx, item_rx): (Sender<CustomFeederItem>, Receiver<CustomFeederItem>) =
        mpsc::channel();

    let config = read_config(Path::new("feeds.toml"));

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
            Some(f) => {
                list_of_feeds.push(f.to_owned());
                f
            }
        };

        create_feed_tx_thread(tx.clone(), feed);
    }

    // reread the feeds file
    thread::spawn(move || {
        let mut last_mod_time: SystemTime = SystemTime::now();
        let metadata = fs::metadata(env::home_dir().unwrap().join("feeds.toml")).unwrap();
        if let Ok(time) = metadata.modified() {
            last_mod_time = time;
        } else {
            println!("Not supported on this platform");
        }
        loop {
            let metadata = fs::metadata(env::home_dir().unwrap().join("feeds.toml")).unwrap();
            if let Ok(time) = metadata.modified() {
                if let Ok(diff) = time.duration_since(last_mod_time) {
                    if diff.as_millis() > 0 {
                        println!("Update detected, {}", list_of_feeds.len());

                        let config = read_config(Path::new("feeds.toml"));
                        let mut feed = config.unwrap().feeds.to_vec();
                        feed.reverse();
                        println!("{:?}", list_of_feeds); //TODO fix multi threads
                        for f in feed {
                            println!("feed item  {}", f);
                            if !list_of_feeds.iter().any(|e| *e == f.to_string()) {
                                println!("new thread");
                                create_feed_tx_thread(tx.clone(), f.as_str().unwrap());
                                list_of_feeds.push(f.to_string());
                            }
                        }
                        last_mod_time = time;
                    }
                }
            }

            thread::sleep(time::Duration::from_secs(CONFIG_RELOAD_SLEEP));
        }
    });

    thread::spawn(move || differentiator(rx, item_tx));

    std::process::Command::new("clear").status().unwrap();

    for c_f_item in item_rx {
        output_feeder_item(&c_f_item);
    }
}
