use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        println!("Problem parsing arguments:{}", error);
        process::exit(1);
    });
    println!(
        "with query:'{}' in file:'{}'",
        config.query, config.filename
    );
    if let Err(e) = minigrep::run(config) {
        println!("minigrep error:{}", e);
        process::exit(1);
    }
}
