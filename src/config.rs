use serde::Deserialize;
use std::path::Path;
use std::{env, fs};
use toml::de::Error;
use toml::value::Array;

#[derive(Deserialize, PartialEq)]
pub struct Config {
    pub feeds: Array,
}

pub fn read_config<P: AsRef<Path>>(filename: P) -> Result<Config, Error> {
    let home = env::home_dir().unwrap();
    let final_path = home.join(filename);
    println!("reading config file {:?}", final_path.display());
    let content = match fs::read_to_string(final_path) {
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
