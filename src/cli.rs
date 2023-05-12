use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Interval of fetching the rss feed file
    #[arg(short, long, default_value_t = 10)]
    pub interval: u64,
}

#[derive(Copy, Clone)]
pub struct Settings {
    pub interval: u64,
    pub max_cache_size: usize,
}

pub static mut SETTINGS: Settings = Settings {
    interval: 5,
    max_cache_size: 30,
};
