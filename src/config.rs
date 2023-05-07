use serde::Deserialize;
use std::{env, fs};
use toml::de::Error;
use toml::value::Array;

#[derive(Deserialize, PartialEq)]
pub struct Config {
    pub feeds: Array,
}

pub fn read_config(filename: &str) -> Result<Config, Error> {
    let home = env::var("HOME").unwrap();
    println!("reading config file {}/{}", home, filename);
    let content = match fs::read_to_string(format!("{}/{}", home, filename)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading config file, {}", e);
            std::process::exit(1);
        }
    };

    toml::from_str(&content)
}

#[cfg(test)]
mod test {
    use super::*;
    use toml::from_str;

    #[test]
    fn test_config_parser() {
        let config: Config = from_str(
            r#"
            feeds = ['https://fd.nl/?rss']
        "#,
        )
        .unwrap();
        assert_eq!(config.feeds.len(), 1);
    }
}
