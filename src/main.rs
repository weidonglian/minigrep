use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments:{}", error);
        process::exit(1);
    });
    println!(
        "with query:'{}' in file:'{}'",
        config.query, config.filename
    );

    let contents = fs::read_to_string(&config.filename)
        .expect(format!("failed to read file:{}", &config.filename).as_str());
    println!("with content:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enought arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config { query, filename });
    }
}
