use std::env;
use std::error::Error;
use std::process;

use minigrep::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    minigrep::run(config)
}
