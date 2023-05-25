# Phazer the RSS client that runs in the terminal

⚠️ Note: This is a hobby project to get more skilled in Rust. Don't take this repo as an example of good Rust code nor is this 
production ready code. It works well enough for this type of project.  ⚠️



## Running instructions
The list of RSS feeds comes from a config file `~/feeds.toml`:
```toml
feeds = [
    "http://feeds.feedburner.com/tweakers/nieuws",
    ]
```

Build it with `cargo build` or just run it with `cargo run`.
For each feed a separate thread is created. Each fetch loop has a build in thread.sleep of 5min. This value is changeable via CLI parameter.
Each new news item is appended to the end of stout.



## Changelog

- v3: Format output
- v2: Use config file
- v1: Add clap 
- v0: Get simple output from RSS (http://feeds.feedburner.com/tweakers/nieuws)
